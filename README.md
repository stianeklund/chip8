# chip8
[![Build Status](https://travis-ci.org/stianeklund/chip8.svg?branch=master)](https://travis-ci.org/stianeklund/chip8)

### A CHIP-8 & SuperChip interpreter written in Rust using rust-sdl2

![](http://i.imgur.com/dCPnV7o.png)

This is a CHIP-8 & SuperChip interpreter written in Rust and currently depends on RUST-SDL2 (SDL2 bindings to Rust). Note: this dependency might change in the future.

####  Building the project:

This project depends on Rust-SDL2 & therefor depends on libsdl2 development libraries.

> E.g for Ubuntu / Debian Linux: sudo apt install libsdl2-dev 
> Please refer here for more details: https://github.com/AngryLawyer/rust-sdl2

The project can be built by running: `cargo build` 
It needs to be run with the rom as a passing argument, e.g: `cargo run /path/to/romfile/rom`

#### Running the CHIP8 interpreter:

Key mapping is: 1-9 & A-F as if it were a real Hexadecimal keypad.
 
 CPU cycle speed can be changed by pressing Page Up & Page Down. Some games play better on a higher cycle speed. E.g Brix works better at 4 cycles.
 
 You can also enter debug mode by pressing F12, this will print a LOT of values to CLI and is not pretty.

---

#### Screenshots: 

Cars (SCHIP):
![](http://i.imgur.com/M4BG1LR.png)

Brix (CHIP-8, upscaled):
![](http://i.imgur.com/oD1KEsD.png)

Joust (SCHIP):
![](http://i.imgur.com/p9ag3ub.png)


---

#### Sources used:

* https://en.wikipedia.org/wiki/CHIP-8
* http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
* http://www.multigesture.net/wp-content/uploads/mirror/goldroad/chip8.shtml
* http://www.emutalk.net/threads/19894-Chip-8/
* http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
* https://reddit.com/r/emudev & #emudev on Slack
* https://reddit.com/r/emulation

#### On pixel drawing:

* http://laurencescotford.co.uk/?p=304 (Written with the Cosmac VIP in mind. Gives a good visualization of how drawing works in relation to memory addresses).
* http://craigthomas.ca/blog/2015/02/19/writing-a-chip-8-emulator-draw-command-part-3/

