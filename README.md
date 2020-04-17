# tilejson-rs

This library is used to encode/decode the `TileJson` format which is described in [TileJson spec 2.2.0](https://github.com/mapbox/tilejson-spec/blob/master/2.2.0/README.md)

## Usage

This library is very simple wrapper on top of [serde](https://serde.rs/). Examples are available [here](https://github.com/mr1sunshine/tilejson-rs/tree/master/examples).

Usage is very simple:

#### Encoding
```rust
    let tilejson = TileJson::default();
    let json = encode(&tilejson);
```

#### Decoding
```rust
    let json = fs::read_to_string(&args[1]).unwrap();
    let tile = decode(&json);
```