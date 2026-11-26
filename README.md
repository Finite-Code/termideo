# termideo
ASCII Video, now over the terminal with Image Support

termideo is a terminal-based video player that supports viewing videos in ASCII format. It is different from the the traditional terminal video players that utilize Unicode characters to display frames.

termideo uses ASCII characters to ensure broad compatibility across various terminal clients, specially those that might not support unicode.

This project was written by me in Rust. Initially as I planned it to be a Cyberpunk-themed video player that lives in the terminal. It has since turned into a more general purpose media player.

## Downloads
The Downloads for the project will be available on the [Releases](https://github.com/Finite-Code/termideo/releases) page very soon. Till then you can build the project from source by following the instructions below.


## Building from Source

To build:
> Building termideo from source requires you to have Rust installed on your system. Do so by following the instructions from the [official Rust website](https://www.rust-lang.org/tools/install).

1. Clone the repository:
   ```bash
   git clone https://github.com/Finite-Code/termideo.git
   ```

2. Navigate to the project folder:
   ```bash
   cd /path/to/termideo
   ```

3. Build it!
   ```bash
   cargo build --release
   ```
    > Tip: You can switch the `--release` flag to `--debug` if you want to debug or help with the code. Note that debug builds can be slower, although the project is notably quick to compile due to it's small size.
