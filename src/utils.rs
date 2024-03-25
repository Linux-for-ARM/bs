//! A number of additional functions to simplify development

use crate::error::Error;

use std::cmp::min;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use futures_util::StreamExt;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use reqwest;
use tokio;

/// Reads the specified file into a `String`. Wrapper over [`std::fs::read_to_string()`].
pub fn read_to_string<P: AsRef<Path>>(pth: P) -> Result<String, Error> {
    fs::read_to_string(&pth)
        .map_err(|err| Error::ReadingError(pth.as_ref().display().to_string(), err.to_string()))
}

/// Writing the specified data to a file. Wrapper over [`std::fs::write()`].
pub fn write<P, C>(pth: P, cont: C) -> Result<(), Error>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    fs::write(&pth, &cont)
        .map_err(|err| Error::WritingError(pth.as_ref().display().to_string(), err.to_string()))
}

fn get_downloaded_fname(url: &str) -> Option<String> {
    let fname = url.rsplit_once('/');

    if let Some(fname) = fname {
        Some(fname.1.to_string())
    } else {
        None
    }
}

#[tokio::main]
pub async fn download(url: &str) -> Result<String, Error> {
    let fname = get_downloaded_fname(url).unwrap_or("tmp.bin".to_string());
    let res = reqwest::get(url)
        .await
        .map_err(|err| Error::DownloadError(err.to_string()))?;
    let total = res.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar}] {bytes}/{total_bytes}")
            .map_err(|err| Error::DownloadError(err.to_string()))?
            .progress_chars("=> "),
    );
    pb.set_message(format!("Download '{url}':"));

    let mut dest = fs::File::create(&fname)
        .map_err(|err| Error::WritingError(fname.clone(), err.to_string()))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|err| Error::DownloadError(err.to_string()))?;
        dest.write_all(&chunk)
            .map_err(|err| Error::WritingError(fname.clone(), err.to_string()))?;

        let new = min(downloaded + (chunk.len() as u64), total);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish();

    Ok(fname)
}

pub fn extract(file: &str) -> Result<(), Error> {
    let cmd = Command::new("/bin/tar")
        .arg(format!("-xvf {file}"))
        .status()
        .map_err(|err| Error::ExtractError(err.to_string()))?;

    if cmd.success() {
        Ok(())
    } else {
        Err(Error::ExtractError("unknown error".to_string()))
    }
}
