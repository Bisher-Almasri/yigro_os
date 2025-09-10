# Yigro OS

Yigro os is a x86_64 bit kernel/OS which supports a ps2 keyboard. it is written in rust.

## Why make this

I did this since i really wanted to make a new [OS](https://github.con/bisher-almasri/bigroos). So i decided to do it in rust. my main goal wsa to have a ps2 keyboard support which i have acomplished. so th next step are:

- [ ] Shell
- [ ] Fs
- [ ] Mouse support
- [ ] Beyond VGA,

# How to run

You will need:

- Rust
- Qemu x86_64

1. Clone the project and cd into it
2. run `cargo run`
3. Done!

or

1. Same as first time
2. run `cargo bootimage`
3. run `qemu-system-x86_64 -drive format=raw,file=target/x86_64_yigro_os/debug/bootimage-yigro_os.bin`
