[![License](https://img.shields.io/badge/License-BSD%202--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)

# About
This library provides a simple timeout-based API for IO-operations.

We provide the following features:
 - DNS-resolution (currently uses a background-thread)
 - TCP-accept (uses [libselect](https://github.com/KizzyCode/libselect))
 - TCP-read/read-until/write (uses [libselect](https://github.com/KizzyCode/libselect))
 - UDP-receive/send (uses [libselect](https://github.com/KizzyCode/libselect))

All functions are defined as traits, so that you can easily wrap your own IO-channels without breaking compatibility.

_Note: We currently do not provide a function for timeout-based `connect`-calls; use
`std::net::TcpStream::connect_timeout` for TCP-connections or build sth. using `io::libselect` (and feel free to commit
if you did so 😇)_

# Dependencies
Because Rust does not provide timeout-based APIs for all functions we currently rely on
[libselect](https://github.com/KizzyCode/libselect)

# Build Library and Documentation
Make sure that [libselect](https://github.com/KizzyCode/libselect) and the Rust-toolchain are installed properly and
up-to-date.

To build the documentation, go into the projects root-directory and run `cargo doc --release`; to open the documentation
in your web-browser, run `cargo doc --open`.

To build the library, go into the projects root-directory and run `cargo build --release`; you can find the build in
target/release.