# sdl3-test

A test SDL3 application in Rust. Renders a black screen and toggles fullscreen
and hiding the mouse on left-click.

## Building

### Linux

* Install [Rust](https://www.rust-lang.org/),
[SDL3](https://wiki.libsdl.org/SDL3/FrontPage), and
[pkg-config](https://www.freedesktop.org/wiki/Software/pkg-config/) through
your system's package manager.
* Run `cargo build` to build the application or `cargo run` to run it.

### Nix

Building and running the application using [Nix](https://nixos.org/) is as easy
as running `nix develop` to enter the dev shell and either `cargo build` or
`cargo run` respectively to build or run the application once inside.
