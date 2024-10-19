#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::{
    fmt::Display,
    fs::{read, read_dir, read_to_string, File},
    path::Path,
    time::Duration,
};

use anyhow::Context;
use csv::ReaderBuilder;
use serde::{de::DeserializeOwned, Deserialize};
use tokio::time::sleep;

use crate::fetch::ASYNC_CLIENT;

#[derive(Deserialize, Debug)]
struct KlineUrl {
    u: String,
    f: String,
}
pub async fn download_binance_kline<J, O>(json_path: J, output: O) -> anyhow::Result<()>
where
    J: AsRef<Path>,
    O: AsRef<Path> + Clone + Display,
{
    let json = read_to_string(json_path).unwrap();
    let futures = serde_json::from_str::<Vec<KlineUrl>>(&json)
        .unwrap()
        .into_iter()
        .map(|i| (ASYNC_CLIENT.get(i.u).send(), i.f))
        .collect::<Vec<_>>();
    for (future, filename) in futures {
        let content = future.await?.bytes().await?;
        let mut dest = File::create(format!("{}/{}", output, filename))?;
        std::io::copy(&mut content.as_ref(), &mut dest)?;
    }
    Ok(())
}

pub async fn unzip<P: AsRef<Path> + Clone>(dir: P) -> anyhow::Result<()> {
    let file_list = read_dir(dir.clone())?;
    let zip_list = file_list
        .into_iter()
        .map(|i| i.unwrap().path())
        .collect::<Vec<_>>();
    for path in zip_list {
        let file = File::open(path.clone())?;
        let mut archive =
            zip::ZipArchive::new(file).context(format!("zip archive error. path: {:?}", path))?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).context("archive error.")?;
            let outpath = match file.enclosed_name() {
                Some(path) => dir.as_ref().join(path),
                None => continue,
            };

            // 创建目录结构
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).context("create dir all error.")?;
                }
            }

            if file.is_dir() {
                // 如果是目录，创建它
                std::fs::create_dir_all(&outpath)?;
            } else {
                // 如果是文件，写入内容
                let mut outfile = File::create(&outpath).context("create path error.")?;
                std::io::copy(&mut file, &mut outfile).context("copy bytes error.")?;
            }

            // 获取并设置文件的权限
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))
                        .context("set permission error.")?;
                }
            }
        }
    }

    Ok(())
}

pub fn load_csv<R: DeserializeOwned>(dir: impl AsRef<Path>) -> anyhow::Result<Vec<R>> {
    let file_list = read_dir(dir)?;
    let path_list = file_list
        .into_iter()
        .map(|i| i.unwrap().path())
        .collect::<Vec<_>>();
    let mut res = vec![];
    for path in path_list {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(path.clone())?;
        let mut item_res = vec![];
        for (index, item) in reader.deserialize::<R>().enumerate() {
            if let Ok(item) = item {
                item_res.push(item);
            } else {
                eprintln!("deserialize file error. path: {:?}, index: {}", path, index)
            }
        }
        res.extend(item_res);
    }
    Ok(res)
}

#[tokio::test]
async fn test_download_binance() {
    download_binance_kline("/Users/siliterong/Project/rust/TradingSystem/utils/src/history_data_load/binance_kline_csv.json", "src/history_data_load/binance_kline/").await.unwrap();
}

#[tokio::test]
async fn unzip_binance() {
    unzip("src/history_data_load/binance_kline/").await.unwrap();
}
