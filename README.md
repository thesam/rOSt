# rOSt

A kernel based on rustboot.

Depends on libcore from the main Rust repository.

Current features:
- Change the background color of the screen
- Print static strings and integers
- On any keyboard input: Echo a character back to the terminal
- Partial translation of keyboard scancodes to ASCII characters
- Allocate memory using the Box syntax and a 1 byte heap :)
