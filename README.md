# ye

> `yes`, but faster, and written in Rust.

See [the following discussion](https://www.reddit.com/r/unix/comments/6gxduc/comment/diua761/?context=8&depth=9) on why GNU `yes` is so fast.

Unfortunately I could not recompile `pv` myself to test it against the C implementation of `yes` that uses `vmsplice` (see [the `ye.c` file](ye.c)), but with the default `pv` program, the Rust implementation seems to be faster than the C version on my machine (about **62.0 GiB/s** in C, **62.7 GiB/s** in Rust).

Note that it makes use of libc's `vmsplice` function, which is of course unsafe. Somehow, the "safe" version that uses the Nix API (which provides a safe wrapper around libc) is faster than the unsafe version that uses libc directly... My guess is that Rust was way better at optimizing that than my direct unsafe code.

I'll do thorough serious benchmarks when I feel like it.

# How to run

**Very important:** since these programs are inspired by the reddit discussion above, know that you absolutely need to run the following command before testing either version.
```sh
$ sudo sysctl fs.pipe-max-size=$((1024*1024*16))
```
This will increase the pipe buffer size from the default 1 MiB to 16 MiB.

## C version

To compile `ye.c` with all optimizations, execute the following:
```sh
$ gcc -O3 -funsafe-loop-optimizations -funsafe-math-optimizations ye.c -o ye
```
You can then measure how fast it writes to the pipe with:
```sh
$ ./ye | pv > /dev/null
```

## Rust version

To compile the `ye` cargo project in release mode, execute the following:
```sh
$ cargo build --release
```
You can then measure how fast it writes to the pipe with:
```sh
$ target/release/ye | pv > /dev/null
```

## Controlling the CPU core of the tasks

According to u/_mrb, *"cache affinity starts playing a big role"* after all these optimizations, so you can use `taskset` to ensure that both programs use the same CPU core:
```sh
# C
$ taskset -c 0 ./ye | taskset -c 0 pv > /dev/null
# Rust
$ taskset -c 0 target/release/ye | taskset -c 0 pv > /dev/null
```
Interestingly, I have tried both `taskset -c 0 ./ye | taskset -c 0 pv > /dev/null` (both on the same core) and `taskset -c 0 ./ye | taskset -c 1 pv > /dev/null` (both on a different core) and have found the *latter* to be 10 GiB faster for some reason. Maybe the fact that I couldn't compile a patched version of `pv` for myself played a role in that.

# My really not serious benchmarks

```sh
# C, same core
❯ taskset -c 0 ./ye | taskset -c 0 pv > /dev/null
 457GiB 0:00:10 [45.6GiB/s] #[ (...) <=> (...) ]

# C, different cores
❯ taskset -c 0 ./ye | taskset -c 1 pv > /dev/null
 621GiB 0:00:10 [62.0GiB/s] #[ (...) <=> (...) ]

# Rust, same core
❯ taskset -c 0 target/release/ye | taskset -c 0 pv > /dev/null
 459GiB 0:00:10 [45.8GiB/s] #[ (...) <=> (...) ]

# Rust, different cores
❯ taskset -c 0 target/release/ye | taskset -c 1 pv > /dev/null
 626GiB 0:00:10 [62.7GiB/s] #[ (...) <=> (...) ]
```
## My machine

If you're curious.

```yaml
        /\         Distro:     Arch Linux x86_64
       /  \        Kernel:     Linux 6.0.5-arch1-1
      /\   \       Shell:      zsh 5.9
     /      \      Resolution: 1920x1080 @ 300Hz
    /   ,,   \     DE:         Plasma 5.26.2
   /   |  |  -\    WM:         KWin
  /_-''    ''-_\   CPU:        AMD Ryzen 9 5900HX with Radeon Graphics (16) @ 3.3GHz
                   GPU:        AMD ATI Radeon RX 6800M
                   Memory:     16GB
```
