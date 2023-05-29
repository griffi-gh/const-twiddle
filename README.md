# const-twiddle
#### Bitwise utility functions

`const-twiddle` is a set of bitwise utility functions designed to be similar to `twiddle`.
It is also compatible with const and supports `#![no_std]` out of the box.

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
  let mut x = 5;
  x.set_bit(0, false);
  assert_eq!(x, 4);
  assert_eq!(x.with_bit(0, true), 5);
  assert_eq!(x.bit(0), false);
}
```

Usage inside `const` contexts:

```rust
use const_twiddle::ConstU32;
 
const fn test() {
  // Traits are not supported in const yet
  let x = ConstU32(5).with_bit(0, false).0; //x = 4
}
```

## License

`const-twiddle` is licensed under the MIT License. See the LICENSE file for more details.
