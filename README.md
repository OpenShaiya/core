# OpenShaiya Core Library
![Build status](https://github.com/OpenShaiya/core/workflows/build/badge.svg)

The core library, which houses common functionality used by multiple projects. This includes packet data structures, cryptography, and accessing the proprietary data files.

OpenShaiya requires the original Shaiya data files to properly emulate the game's behaviour, which we cannot legally distribute. Therefore you must provide your own copy of any relevant files you wish to use.

## Using this library
The recommended way to use this library is to simply add the Git repository as a dependency in your `Cargo.toml` file, such as:
```toml
[dependencies]
shcore = { git = "https://github.com/OpenShaiya/core/" }
```

## Querying the client's data archive
Querying a data archive from the game client can be done with a simple:
```rust
use shcore::client::Workspace;
use shcore::Result;

fn main() -> Result<()> {
    let workspace = Workspace::from_archive("data.sah", "data.saf")?;
    let items = workspace.file("item/item.sdata")?;
    println!("Item file: {:?}", items);
    Ok(())
}

```
## License
OpenShaiya is available under the terms of the [MIT license](https://tldrlegal.com/license/mit-license). The full copyright notice and terms are available in the `LICENSE` file.