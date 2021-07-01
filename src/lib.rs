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

async fn delete_files(files: &[String]) {
    for f in files {
        tokio::fs::remove_file(f).await.unwrap();
    }
}

async fn delete_folder(folders: &[String]) {
    for f in folders {
        let _ = tokio::fs::remove_dir_all(f).await;
    }
}

pub fn remove_file(path: &str) -> std::io::Result<()> {
    std::fs::remove_file(path)?;
    Ok(())
}

pub async fn remove_file_async(path: &str) -> std::io::Result<()> {
    tokio::fs::remove_file(path).await?;
    Ok(())
}

pub fn remove_dir(path: &str) -> std::io::Result<()> {
    std::fs::remove_dir(path)?;
    Ok(())
}

pub async fn remove_dir_async(path: &str) -> std::io::Result<()> {
    tokio::fs::remove_dir(path).await?;
    Ok(())
}

pub fn remove_dir_all(path: &str) -> std::io::Result<()> {
    std::fs::remove_dir_all(path)?;
    Ok(())
}

pub async fn remove_dir_all_async(path: &str) -> std::io::Result<()> {
    tokio::fs::remove_dir_all(path).await?;
    Ok(())
}

pub async fn rapid_remove_dir_all(
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
            delete_files(chunk).await;
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
            delete_folder(folder).await;
        })
    }

    while workers.next().await.is_some() {}

    let _ = std::fs::remove_dir_all(path);

    Ok(())
}
