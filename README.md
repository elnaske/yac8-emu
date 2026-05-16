# Yet Another CHIP-8 Emulator

A CHIP-8 emulator/interpreter and debugger written in Rust + SDL2.

## Roadmap
| Feature      | Status      |
| ------------ | ----------- |
| Display      | Done*       |
| Input        | Done*       |
| System Specs | Done*       |
| Timers       | Done*       |
| Interpreter  | In progress |
| Debugger     | In progress |
| OpCodes      | 6/35        |

## Installation

This project uses SDL2 for display, keyboard input, and sound.
To be able to build it, you need to have libsdl2 installed on your system.
For more information, see the [Rust-SDL2 crate](https://github.com/Rust-SDL2/rust-sdl2) or http://www.libsdl.org/.

On Linux:
```shell
# Ubuntu
sudo apt install libsdl2-dev

# Arch
sudo pacman -S sdl2
```

Then, build the project using cargo:
```shell
cargo build
```