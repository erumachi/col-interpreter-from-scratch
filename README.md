# interpreter-from-scratch

This repo contains the code for this challenge: https://regyxxer.xyz/interpreter-from-scratch/

The link is dead, but the language we implemented is StupidStackLanguage: https://esolangs.org/wiki/StupidStackLanguage

## Build and run

You can build this as any other Rust repository running `cargo build` at the root of the repo.
This will generate a bunch of executables called `part_N`, one for each part.
You can run them manually with the arguments they should have or you can use:

`cargo run --bin part_N -- args_of_the_binary`

to run the binary called `part_N` with arguments `args_of_the_binary`.

Part 5 implements a debugger. Use `--debug` as the third argument to use it:

`cargo run --bin part_N -- run aaaxbx --debug`

You can see the available debugger commands using `help`
