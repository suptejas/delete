## About

This crate allows for fast and easy deletion of files and folders. It has `async` and cross-platform support.

Many of the functions in this crate directly call `std::fs` and `tokio::fs`.

This crate aims to be:

- Fast
- Easy To Use
- Powerful

## Examples

### Non-Async Implementation

```rust
use delete::{delete_file};

fn main() {
  // Delete file.txt
  delete_file("file.txt").unwrap();
}
```

```rust
use delete::{delete_folder};

fn main() {
  // Delete tests folder
  delete_folder("tests").unwrap();
}
```

### Async Implementation

```rust
use delete::{delete_file_async};

#[tokio::main]
async fn main() {
  // Delete file.txt asynchronously
  delete_file_async("file.txt").await.unwrap();
}
```

```rust
use delete::{delete_folder_async};

#[tokio::main]
async fn main() {
  // Delete tests folder asynchronously
  delete_folder_async("tests").await.unwrap();
}
```

## Rapid Implementations

```rust
use delete::{rapid_delete_dir_all};

#[tokio::main]
async fn main() {
  // 2-3x faster than std::fs::remove_dir_all
  // removes all files and folders in subfolders recursively using tokio workers
  rapid_delete_dir_all("node_modules", None, None).await;
}
```

### Credits

[tokio](https://crates.io/crates/tokio)
