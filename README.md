# const-twiddle
#### Bitwise utility functions

`const-twiddle` is a set of bitwise utility functions designed to be similar to `twiddle`.
It is also compatible with const and supports `#![no_std]` out of the box.

This library was originally created for use in a Game Boy Advance emulator.

## Usage

Add `const-twiddle` to your Cargo.toml file:

```toml
[dependencies]
const-twiddle = "0.0.4"
```

Usage outside of `const`:

```rust
use const_twiddle::Twiddle;

fn test() {
  let x = 5;
  let y = x.set_bit(0, false); // y == 4
}
```

Usage inside `const` contexts:

```rust
use const_twiddle::ConstU32;
 
const fn test() {
  // Traits are not supported in const yet
  let x = ConstU32(5).set_bit(0, false); //x = 4
}
```

## License

`const-twiddle` is licensed under the MIT License. See the LICENSE file for more details.

