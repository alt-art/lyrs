# lyrs
Command line aplication to view lyrics.

## Summary
- [How to build](#how-to-build)
    - [Building a .deb package](#building-a-deb-package)
- [Usage](#usage)
- [Screnshot](#screenshot)

## How to build
Install rust tools like `cargo` on this [link](https://www.rust-lang.org/tools/install)

```sh
cargo build # Build in debug mode
# or 
cargo build --release # Build in release mode
```

### Building a .deb package
```sh
cargo install cargo-deb # Install cargo-deb
```
```sh
cargo deb # Build .deb package in release mode
```

## Usage

```
lyrs v0.1.0
Pedro H. M. <pedromendescraft@gmail.com>
Command line aplication to view lyrics

USAGE:
    lyrs [FLAGS] <SEARCH>...

FLAGS:
    -h, --help       Prints help information
    -l, --login      Login in with a genius account
    -v, --version    Prints version information

ARGS:
    <SEARCH>...    Search for a song like (e.g Slipknot Duality)
```

## Screenshot

<div align="center">

![lyrs](https://media.githubusercontent.com/media/alt-art/lyrs/main/resource/lyrs.png)

</div>