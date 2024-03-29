# rivas: RISC-V Assember #

Current Version: 0.2.0

## Table of Contents ##

- [rivas: RISC-V Assember](#rivas-risc-v-assember)
  - [Table of Contents](#table-of-contents)
  - [What is this](#what-is-this)
    - [Using with VHDL](#using-with-vhdl)
  - [Installation](#installation)
    - [Linux](#linux)
      - [Using the prebuilt binary](#using-the-prebuilt-binary)
      - [Using cargo](#using-cargo)
    - [Windows](#windows)
  - [Version goals](#version-goals)
    - [Version 0.2.0 goals](#version-020-goals)
    - [Version 0.3.0 goals](#version-030-goals)
  - [Current release](#current-release)

## What is this ##

This is a very little riscv32 assembler.
It's main feature is to generate binary code, ready to embed in hardware description code.
There are barely any plans to add any other features.
Also I will only continue working on this, when I need to generate riscv-assembly for VHDL myself.

### Using with VHDL ###

Rivas is meant to be used with vhdl (and maybe verilog in the future, if I need that).
The assembler can output code, that is ready to embed in a 32-bit readonly-memory.

## Installation ##

### Linux ###

There are two ways to use this program using linux:

#### Using the prebuilt binary ####

The prebuilt binary is located at `target/release/rivas` and should be working on it's own on any linuy distribution.

#### Using cargo ####

Install **git** and **cargo**.

1. `git clone https://github.com/yannickreiss/rivas`
2. `cd rivas`
3. `cargo build --release`
4. `./target/release/rivas`

### Windows ###

To build and use this program under windows, you need to have rust and cargo installed.

1. `git clone https://github.com/yannickreiss/rivas`
2. `cd rivas`
3. `cargo build --release`
4. run the executable in `target/release/rivas`

## Version goals ##

### Version 0.2.0 goals ###

- commandline option `-o` to specify output name (Done)
- Output a file with code ready to embed in VHDL (Done) (Output to stdout and .o.vhdl)
- Assemble all 32I-Base instructions (Done)
- reserved registers names (done)
- ABI-Syntax     (done)

### Version 0.3.0 goals ###

- Input a file to generate RISC-V binary with same name
- labels
- Generate full VHDL memory entity

## Current release ##

There is currently no release, as the program is still in developement of it's basic features.
The first release will be the version **0.2.0**.
