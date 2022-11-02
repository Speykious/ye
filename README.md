# ye

> `yes`, but faster, and written in Rust.

See [the following discussion](https://www.reddit.com/r/unix/comments/6gxduc/comment/diua761/?context=8&depth=9) on why GNU `yes` is so fast.

Unfortunately I could not recompile `pv` myself to test it against the C implementation of `yes` that uses `vmsplice` (see [the `ye.c` file](ye.c)), but with the default `pv` program, the Rust implementation seems to be slightly faster on my machine on release mode (about 55 GiB/s for ye.rs, 52 GiB/s for ye.c).

Not that it makes use of libc's `vmsplice` function, which is of course unsafe. Somehow, the "safe" version that uses the Nix API is faster than the unsafe version that uses libc directly... This is probably because Rust was way better at optimizing that than my direct unsafe code.

I'll do thorough concrete benchmarks when I feel like it.