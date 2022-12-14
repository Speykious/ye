use std::io::{self, IoSlice};

use clap::{command, Parser};
use nix::fcntl::{fcntl, SpliceFFlags};

const BUF_LEN: usize = 1024 * 1024;
const IOV_LEN: usize = 1024;

#[derive(Parser)]
#[command(name = "ye")]
#[command(version, author, about)]
struct Cli {
    #[arg(
        long,
        default_value_t = false,
        help = "Whether to directly use libc's vmsplice"
    )]
    use_unsafe: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    if let Err(e) = fcntl(1, nix::fcntl::FcntlArg::F_SETPIPE_SZ(16 * 1024 * 1024)) {
        eprintln!("Warning: could not set pipe size (fcntl F_SETPIPE_SZ: {}). Maybe you're not piping?", e.desc());
        std::process::exit(1);
    };

    let ye = "y\n";
    let ye = ye.repeat(BUF_LEN / ye.len()).into_bytes();
    let iov = [IoSlice::new(&ye); IOV_LEN];

    if args.use_unsafe {
        unsafe {
            unsafe_vmsplice_loop(&iov);
        }
    } else {
        safe_vmsplice_loop(&iov);
    }
}

unsafe fn unsafe_vmsplice_loop(iov: &[IoSlice]) -> ! {
    let iov_ptr = iov.as_ptr() as *const libc::iovec;
    let iov_len = iov.len();
    let splice_bits = SpliceFFlags::SPLICE_F_GIFT.bits();

    loop {
        libc::vmsplice(1, iov_ptr, iov_len, splice_bits);
    }
}

fn safe_vmsplice_loop(iov: &[IoSlice]) -> ! {
    loop {
        let _ = nix::fcntl::vmsplice(1, iov, SpliceFFlags::SPLICE_F_GIFT).unwrap();
    }
}
