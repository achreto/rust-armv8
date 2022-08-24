# Rust ARMv8 (aarch64/aarch32)

Rust library to program ARMv8-A hardware. It provides access and construction functions
for ARMv8 hardware-defined state and data structures.

## Acknowledgements

The library closely follows the structure of [rust-x86](https://github.com/gz/rust-x86),
and borrows some of its code.

## Supports

 * architectural registers (generated from arm xml spec)
 * paging definitions (4k granule)
 
## Todo's
 
 * AArch32 support
 * Paging granules 16k and 64k

## Register Binding Generation

To generate/update the Rust register bindings execute:
```
    $ bash tools/generate-register-bindings.sh
```

Dependencies for the Rust register binding generations:
```
    # apt-get install python3-plumbum
```
