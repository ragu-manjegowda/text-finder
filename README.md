<br/><br/>

[![Text Finder CI](https://github.com/ragu-manjegowda/text-finder/workflows/Text%20Finder%20CI/badge.svg)](https://github.com/ragu-manjegowda/text-finder/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

<br/><br/>

# Why Rust

This project is my first baby step towards learning [RUST](https://www.rust-lang.org/).

If you are system software engineer and looking for a strong reason 
for why Rust (apart from the fact it is getting recommended to be 
included in Linux kernel code), here is the video that I highly 
recommend - [Considering Rust](https://www.youtube.com/watch?v=DnT-LUQgc7s)

<br/><br/>

# Text Finder

Text Finder is a tool for locating files containing text that matches a specified regular expression.  
It uses the facilities of RustDirNav and RustCmdLine libraries, and
std::fs and regex crates.

It can be used to:
 * Find all files in a directory subtree with text that matches a
    specified regular expression.
 * Find all files that have specified extensions (patterns).
 * List all directories and their files in some directory subtree.

<br/><br/>

# Design

<img src="man/figures/design.jpeg" width="900" height="350" />

Traits are specified by caller and implemented by called. This reverses 
the dependency chain and hence called ***Dependency Inversion 
Principle*** or ***Type Erase***

## Pros
 * Each component can be built and compiled separately.
 * Continuous output, good ergonomics for user.
 * Input, Compute and Output are decoupled, they depend only on the 
   interface defined by the upstream component.

## Cons
 * Little complex to implement
 * Each component needs to use factory function to create it's down 
   stream component.

<br/><br/>

# Dependencies and Build instructions

## Using docker

Download image and start Docker interactive container

    $ ./run.sh # $HOME is mounted to /mnt inside container

***You might have to modify run script according to your needs.***

## Native
Highly recommend installing rust with [**rustup**](https://www.rust-lang.org/tools/install).

## Build and Run

Each module has examples and tests (refer workflow file for more info).  
For example, use the below command to run dir nav example app.

    $ cargo run --example dir_nav_example_app

<br/><br/>

# Credits and References

I am indebted to [Prof. Dr. Jim Fawcett](https://jimfawcett.github.io/) for 
motivating me (and many others) to learn rust and taking time to [teach Rust](https://www.youtube.com/playlist?list=PLRqLz6z12zQp3eNC6cOtMa1zlhBimMHkD) for  
all of his former students through bi-weekly sessions.

This project and it's design principles are largely based on what he taught.
