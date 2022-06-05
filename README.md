# idl-macro

Generates an Anchor CPI crate from a JSON IDL.

## Usage

In a new crate, write:

```rust
idl_macro::generate_cpi_crate!("src/idl.json");

declare_id!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
```

This will generate a fully functional Rust CPI client for your IDL.

## License

Apache 2.0
