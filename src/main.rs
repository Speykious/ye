use std::io::{self, IoSlice};

use nix::fcntl::{fcntl, SpliceFFlags};

const BUF_LEN: usize = 2 * 1024 * 1024;
const IOV_LEN: usize = 1024;

fn main() -> io::Result<()> {
    let yes = "y\n".repeat(BUF_LEN / "y\n".len()).into_bytes();
    let iov = [IoSlice::new(&yes); IOV_LEN];

    fcntl(1, nix::fcntl::FcntlArg::F_SETPIPE_SZ(16 * 1024 * 1024))?;

    safe_vmsplice_loop(&iov);
    // unsafe { unsafe_vmsplice_loop(&iov) }
}

unsafe fn unsafe_vmsplice_loop(iov: &[IoSlice]) -> ! {
    let iov_ptr = iov.as_ptr() as *const libc::iovec;
    let splice_bits = SpliceFFlags::SPLICE_F_GIFT.bits();

    loop {
        libc::vmsplice(1, iov_ptr, IOV_LEN, splice_bits);
    }
}

fn safe_vmsplice_loop(iov: &[IoSlice]) -> ! {
    loop {
        let _ = nix::fcntl::vmsplice(1, iov, SpliceFFlags::SPLICE_F_GIFT).unwrap();
    }
}
