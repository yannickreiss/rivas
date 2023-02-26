# rivas: RISC-V Assember
Currently Version 0.6
Not implemented any features!

## Version 1.0 goals
- Input a file to generate RISC-V binary with same name 
- commandline option `-o` to specify output name (Done) (not outputting atm)
- Output a .vhdlsc file with code ready to embed in VHDL (Done) (Output to stdout)
- Assemble all 32I-Base instructions 

# Rust Version
Located in `Rust/rivas`, compile with cargo (Rust build tool).   

## using precompiled version
Every 4 to 5 subversions a release will be dropped to `rivas/target/release/rivas`.
Just execute it with `./rivas file.S`.

# Ada Version
Currently not started, planned after stable Rust version.