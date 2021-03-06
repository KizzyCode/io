[![docs.rs](https://docs.rs/timeout_io/badge.svg)](https://docs.rs/timeout_io)
[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/timeout_io.svg)](https://crates.io/crates/timeout_io)
[![Download numbers](https://img.shields.io/crates/d/timeout_io.svg)](https://crates.io/crates/timeout_io)
[![Travis CI](https://travis-ci.org/KizzyCode/timeout_io.svg?branch=master)](https://travis-ci.org/KizzyCode/timeout_io)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/timeout_io?svg=true)](https://ci.appveyor.com/project/KizzyCode/timeout-io)
[![dependency status](https://deps.rs/crate/timeout_io/0.5.0/status.svg)](https://deps.rs/crate/timeout_io/0.5.0)

# About
This library provides a simple timeout-based API for IO-operations.

It provides the following features:
 - DNS-resolution (currently uses a background-thread)
 - TCP-accept
 - TCP-read/read-until/write
 - StdIOE-read/read-until/write
 - UDP-receive/send
 - A select-like API to wait on multiple source simultaneously

All functions are defined as traits, so that you can easily wrap your own IO-channels without 
breaking compatibility.

_Note: We currently do not provide a function for timeout-based `connect`-calls; use
`std::net::TcpStream::connect_timeout` for TCP-connections or build sth. using `io::libselect` (and
feel free to commit if you do so 😇)_