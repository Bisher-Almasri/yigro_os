# Yigro OS

Yigro os is a x86_64 bit kernel/OS which supports a ps2 keyboard. it is written in rust.

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
