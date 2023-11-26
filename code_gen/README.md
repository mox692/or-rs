## Code generation of `Or` type

The `Or` type provided by this library provides a separate enum type for each number of elements in the generics, but its boilerplate is generated automatically by this binary crate.  


## Generate code

```bash
# Output code to `or/src/enums.rs`
cargo run --bin code_gen
```
