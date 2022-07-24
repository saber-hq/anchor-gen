# anchor-gen

Generates a crate for cross-program invocations to an Anchor program from a JSON IDL.

## Usage

In a new crate, write:

```rust
anchor_gen::generate_cpi_crate!("../../examples/govern-cpi/idl.json");

declare_id!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
```

This will generate a fully functional Rust CPI client for your IDL.

More examples can be found in the [examples/](https://github.com/saber-hq/anchor-gen/tree/master/examples) directory.

License: Apache-2.0
