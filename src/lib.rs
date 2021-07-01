//! Fast, easy deletion of files and folders with async and cross-platform support.
//! ## Overview
//!
//! This crate allows for fast and easy deletion of files and folders. It has `async` and cross-platform support.
//!
//! Many of the functions in this crate directly call `std::fs` and `tokio::fs`.
//!
//! This crate aims to be:
//!
//! - Fast
//! - Easy To Use
//! - Powerful
//!
//! ## Examples
//!
//! ### Non-Async Implementation
//!
//! ```rust
//! use delete::{delete_file};
//!
//! fn main() {
//!   // Delete file.txt
//!   delete_file("file.txt").unwrap();
//! }
//! ```
//!
//! ```rust
//! use delete::{delete_folder};
//!
//! fn main() {
//!   // Delete tests folder
//!   delete_folder("tests").unwrap();
//! }
//! ```
//!
//! ### Async Implementation
//!
//! ```rust
//! use delete::{delete_file_async};
//!
//! #[tokio::main]
//! async fn main() {
//!   // Delete file.txt asynchronously
//!   delete_file_async("file.txt").await.unwrap();
//! }
//! ```
//!
//! ```rust
//! use delete::{delete_folder_async};
//!
//! #[tokio::main]
//! async fn main() {
//!   // Delete tests folder asynchronously
//!   delete_folder_async("tests").await.unwrap();
//! }
//! ```
//!
//! ## Rapid Implementations
//!
//! ```rust
//! use delete::{rapid_delete_dir_all};
//!
//! #[tokio::main]
//! async fn main() {
//!   // 2-3x faster than std::fs::remove_dir_all
//!   // removes all files and folders in subfolders recursively using tokio workers
//!   rapid_delete_dir_all("node_modules", None, None).await;
//! }
//! ```
//!
//! ### Credits
//!
//! [tokio](https://crates.io/crates/tokio)

use futures::stream::{FuturesUnordered, StreamExt};

fn walkdir(path: &str) -> (Vec<String>, Vec<String>) {
    let mut files_vec: Vec<String> = vec![];
    let mut folders_vec: Vec<String> = vec![];
    for entry in jwalk::WalkDir::new(path) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            files_vec.push(entry.path().to_str().unwrap().to_string());
        } else {
            folders_vec.push(entry.path().to_str().unwrap().to_string());
        }
    }
    (files_vec, folders_vec)
}

async fn priv_delete_files(files: &[String]) {
    for f in files {
        tokio::fs::remove_file(f).await.unwrap();
    }
}

async fn priv_delete_folder(folders: &[String]) {
    for f in folders {
        let _ = tokio::fs::remove_dir_all(f).await;
    }
}

/// Delete a file from the filesystem.
///
/// Uses `std::fs` internally.
/// ## Examples
/// ```
/// use delete::{delete_file};
///
/// fn main() {
///   // Delete file.txt
///   delete_file("file.txt").unwrap();
/// }
/// ```
///
pub fn delete_file(path: &str) -> std::io::Result<()> {
    std::fs::remove_file(path)?;
    Ok(())
}

/// Delete a file from the filesystem using `async` and `tokio`.
///
/// Uses `tokio::fs` internally.
/// ## Examples
/// ```
/// use delete::{delete_file_async};
///
/// #[tokio::main]
/// async fn main() {
///   // Delete file.txt
///   delete_file_async("file.txt").await.unwrap();
/// }
/// ```
///
pub async fn delete_file_async(path: &str) -> std::io::Result<()> {
    tokio::fs::remove_file(path).await?;
    Ok(())
}

/// Delete an empty folder from the filesystem.
///
/// Uses `std::fs` internally.
/// ## Examples
/// ```
/// use delete::{delete_folder};
///
/// fn main() {
///   // Delete tests folder
///   delete_folder("tests").unwrap();
/// }
/// ```
///
pub fn delete_folder(path: &str) -> std::io::Result<()> {
    std::fs::remove_dir(path)?;
    Ok(())
}

/// Delete an empty folder from the filesystem using `async` and `tokio`.
///
/// Uses `tokio::fs` internally.
/// ## Examples
/// ```
/// use delete::{delete_folder_async};
///
/// #[tokio::main]
/// async fn main() {
///   // Delete tests folder
///   delete_folder_async("tests").await.unwrap();
/// }
/// ```
///
pub async fn delete_folder_async(path: &str) -> std::io::Result<()> {
    tokio::fs::remove_dir(path).await?;
    Ok(())
}

/// Delete a folder from the filesystem after recursively deleting all its contents.
///
/// Uses `std::fs` internally.
/// ## Examples
/// ```
/// use delete::{delete_folder_all};
///
/// fn main() {
///   // Delete node_modules folder and all its contents.
///   delete_folder_all("node_modules").unwrap();
/// }
/// ```
///
pub fn delete_folder_all(path: &str) -> std::io::Result<()> {
    std::fs::remove_dir_all(path)?;
    Ok(())
}

/// Delete a folder from the filesystem after recursively deleting all its contents using `async` and `tokio`.
///
/// Uses `tokio::fs` internally.
/// ## Examples
/// ```
/// use delete::{delete_folder_all_async};
///
/// #[tokio::main]
/// async fn main() {
///   // Delete node_modules folder and all its contents.
///   delete_folder_all_async("node_modules").await.unwrap();
/// }
/// ```
///
pub async fn delete_folder_all_async(path: &str) -> std::io::Result<()> {
    tokio::fs::remove_dir_all(path).await?;
    Ok(())
}

/// Rapidly delete a folder from the filesystem after recursively deleting all its contents using `async` and `tokio`.
///
/// Benchmarked to be 2-3x faster than `std::fs::remove_dir_all()`
///
/// Uses tokio workers to delete files and folders parallely.
///
/// ## Parameters
/// path: `&str` - path to the folder to delete
///
/// (Optional) folders_chunk_size: `Option<u64>` - number of folders to be deleted per worker.
///
/// if this value is lower, more workers are spawned.
/// (Optional) files_chunk_size: `Option<u64>` - number of files to be deleted per worker.
///
/// if this value is lower, more workers are spawned.
///
/// Uses `tokio::fs` internally.
/// ## Examples
/// ```
/// use delete::{rapid_delete_dir_all};
///
/// #[tokio::main]
/// async fn main() {
///   // Delete node_modules folder and all its contents 2x faster.
///   rapid_delete_dir_all("node_modules", None, None).await.unwrap();
/// }
/// ```
///
pub async fn rapid_delete_dir_all(
    path: &str,
    folders_chunk_size: Option<u64>,
    files_chunk_size: Option<u64>,
) -> std::io::Result<()> {
    let (files, directories) = walkdir(path);

    let mut workers = FuturesUnordered::new();

    let file_chunk_size;

    if files_chunk_size.is_some() {
        file_chunk_size = files_chunk_size.unwrap();
    } else {
        file_chunk_size = 350;
    }

    let chunks = files.chunks(file_chunk_size as usize);

    for chunk in chunks {
        workers.push(async move {
            priv_delete_files(chunk).await;
        });
    }

    while workers.next().await.is_some() {}

    let folder_chunk_size;

    if folders_chunk_size.is_some() {
        folder_chunk_size = folders_chunk_size.unwrap();
    } else {
        folder_chunk_size = 25;
    }

    let folders = directories.chunks(folder_chunk_size as usize);

    let mut workers = FuturesUnordered::new();

    for folder in folders {
        workers.push(async move {
            priv_delete_folder(folder).await;
        })
    }

    while workers.next().await.is_some() {}

    let _ = std::fs::remove_dir_all(path);

    Ok(())
}
