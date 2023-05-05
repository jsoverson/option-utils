# option-utils

A collection of utilities for working with Rust's Option type

## Example

```rust
use option_utils::OptionUtils;
let mut x = Some("Hello world".to_owned());
x.inner_mut(|s| s.push('!'));
assert_eq!(x, Some("Hello world!".to_owned()));

let path = Some("dir");
let path: Option<std::path::PathBuf> = path.map_into();
assert_eq!(path, Some(std::path::Path::new("dir").to_owned()));

let num = Some(10_u64);
let num: Option<u8> = num.try_map_into()?;
assert_eq!(num, Some(10_u8));```

## License

This project is licensed under the Apache-2.0 license.
