# arbitrary-wrappers

This crate defines newtype wrappers for a number of commonly used types from the
[CAP](https://github.com/EspressoSystems/cap) protocol. The newtype wrappers have implementations of
the [Arbitrary](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait, so that
these types can be used with `Arbitrary` and
[QuickCheck](https://docs.rs/quickcheck/latest/quickcheck/) without adding clutter to CAP.

## Usage

Add to your Cargo.toml:
```
arbitrary-wrappers = { git = "https://github.com/EspressoSystems/arbitrary-wrappers.git" }
```
