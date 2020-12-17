# OpenShaiya Core Library
![Build status](https://github.com/OpenShaiya/core/workflows/Rust/badge.svg)

The core library, which houses common functionality used by multiple projects. This includes packet data structures, cryptography, and accessing the proprietary data files.

OpenShaiya requires the original Shaiya data files to properly emulate the game's behaviour, which we cannot legally distribute.

## Using this library
The recommended way to use this library is to simply add the Git repository as a dependency in your `Cargo.toml` file, such as:
```toml
[dependencies]
shcore = { git = "https://github.com/OpenShaiya/core/" }
```

## License
OpenShaiya is available under the terms of the [MIT license](https://tldrlegal.com/license/mit-license). The fully copyright notice and terms are available in the `LICENSE` file.