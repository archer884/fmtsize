# fmtsize

Format memory sizes.

## Usage

Import the trait and call the method.

```rust
use fmtsize::{Conventional, FmtSize};

println!("{}", 492_752_310_u64.fmt_size(Conventional)); // 469.93 MB
```

## License

MIT or Apache 2.0. Pick whichever you like.
