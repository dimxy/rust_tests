
use std::io::stdin;
use async_std::fs as async_fs;
use async_std::io::{ReadExt, WriteExt};
//use futures::AsyncWriteExt;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub type IoResult<T> = Result<T, std::io::Error>;

pub async fn read_file(path: &Path) -> IoResult<()>
{

    let mut file = async_fs::File::open(&path).await?;
    let mut s = String::new();
    let mut buffer = Vec::<u8>::new();

    println!("waiting for input...");
    stdin().read_line(&mut s)?;
    let r = file.read_to_end(&mut buffer).await?;
    println!("buffer={:?}", buffer);
    Ok(())
}

#[tokio::main]
async fn main() {
    let r = read_file(&Path::new("file1.txt")).await;
    println!("read_file err={:?}", r);
}

