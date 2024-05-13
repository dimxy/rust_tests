use async_std::fs as async_fs;
use async_std::io::WriteExt;
//use futures::AsyncWriteExt;
use std::io::{self};
use std::path::{Path, PathBuf};

pub type IoResult<T> = Result<T, std::io::Error>;



pub async fn rename_file(path: &Path, use_tmp_file: bool) -> IoResult<()>
{
    let path_tmp = if use_tmp_file {
        PathBuf::from(format!("{}.tmp", path.display()))
    } else {
        path.to_path_buf()
    };

    let fs_fut = async {
        let mut file = async_fs::File::create(&path_tmp).await?;
        file.write_all("file renamed\n1234567890".as_bytes()).await?;
        file.flush().await?;

        if use_tmp_file {
            async_fs::rename(path_tmp, path).await?;
        }
        Ok(())
    };
    let res: io::Result<_> = fs_fut.await;
    res
}

#[tokio::main]
async fn main() {
    let _r = rename_file(&Path::new("file1.txt"), true).await;
}
