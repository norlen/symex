# SYMEX

Symbolic execution engine that operates on LLVM IR. Main use case is to analyze Rust programs but
as it operates on LLVM IR it allows it to analyze all software that can generate LLVM IR.

## Dependencies

- [boolector](https://github.com/Boolector/boolector), Boolector is a Satisfiability Modulo Theories
  (SMT) solver for the theories of fixed-size bit-vectors, arrays and uninterpreted functions.
- [LLVM](https://llvm.org/), used as a library for decoding the LLVM-IR (internal representation)
  of the program under analysis.

The project currently uses LLVM 14 which require a relatively recent version of Rust.

### Optional dependencies

It is possible to use Z3 instead of Boolector by using the feature flag `z3`.

- [Z3](https://github.com/Z3Prover/z3), Z3 is a theorem prover from Microsoft Research.

## Cargo subcommand

A cargo subcommand is available to easily compile Rust programs into bitcode files and run them
in the symbolic execution engine.

It can be installed with

```shell
> cargo install --path cli
```

## Getting started

Check out the examples contained in `examles/examples`. These can either be run with the cargo
subcommand or with

```shell
> cd examples
> cargo run -p cli -- --example <example> [--function <function>]
```

To compile and run the example `examples/rust_simple` using the cargo subcommand

```shell
> cd examples
> cargo symex --example simple --function rust_simple_test
```

This will display the results of the analysis of the example, showing all the paths it took and
concrete values for all inputs and output.

## Known issues

Sometimes when running examples the runner may give

```shell
error: could not copy "<project_dir>/target/debug/examples/<some_file>.bc" to "<project_dir>/target/debug/examples/<some_file>.bc": No such file or directory (os error 2)

error: could not copy "<project_dir>/target/debug/examples/<some_file>.ll" to "<project_dir>/target/debug/examples/<some_file>.ll": No such file or directory (os error 2)

error: could not compile `examples` due to 2 previous errors
```

Until the is fixed it can remedied by running `cargo clean` and trying again.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
