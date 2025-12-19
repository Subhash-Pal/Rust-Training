# lab39C

Advanced Rust CLI with:
- Compile-time metadata
- Versioning
- Optimized release build

## Build
cargo build --release

## Usage
lab39C greet <name>
lab39C info
lab39C --help
lab39C --version


🧠 Key Advanced Concepts
Feature	                          Why
Compile-time metadata	        Zero runtime cost
env! macro	Safe,               immutable constants
--version	                    Professional CLI standard
Optimized release	            Production-ready

Summary (1-line takeaway per option)

-opt-level → trade binary size vs speed

-lto → cross-crate optimization vs build time

-codegen-units → compile speed vs optimization quality

-panic → recovery support vs minimal binary

-strip → debuggability vs smallest output