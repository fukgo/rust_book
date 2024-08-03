
//从当前工作目录中的压缩包 archive.tar.gz，
// 解压（GzDecoder）和提取（Archive::unpack）所有文件，
// 并放在同一位置。

use std::fs::File;
use flate2::Compression;
use flate2::read::{GzDecoder, GzEncoder};
use tar::Archive;
use std::path::PathBuf;
//解压 tar 包
fn pack()->Result<(), std::io::Error> {
    let path = "archive.tar.gz";
    // 打开一个 .tar.gz 文件
    let tar_gz = File::open(path)?;
    // 创建 GzDecoder 用于解压 gzip 部分
    //let tar = GzDecoder::new(tar_gz);
    // 打开一个 .tar.gz 文件
    let mut archive = Archive::new(tar_gz);
    // 提取 tar 文件内容到指定目录
    archive.unpack("out")?;

    Ok(())
}
/*
压缩 /var/log 目录内的内容到 archive.tar.gz 压缩包中。

创建一个用 GzEncoder 和 tar::Builder 包裹的 File。

使用 Builder::append_dir_all，将 /var/log 目录内的内容递归添加到 backup/logs
路径下的归档文件中。在将数据写入压缩包 archive.tar.gz 之前，GzEncoder 负责清晰地将数据压缩。

*/
fn unpack()->Result<(), std::io::Error> {
    //// 创建一个新的 .tar.gz 文件
    let tar_gz = File::create("archive.tar.gz")?;
    // 创建 GzEncoder 用于将 tar 数据压缩为 gzip 格式
    let enc = GzEncoder::new(tar_gz,Compression::default());
    // 创建 tar 构建器，将压缩数据写入到 GzEncoder 中
    let mut tar = tar::Builder::new(enc);
    // 将目录 /var/log 打包到 archive.tar.gz 文件中
    // 这里的 "out" 是打包后的路径，"index" 是实际的源目录
    tar.append_dir_all("out","index")?;
    Ok(())
}



fn main() {
    pack().unwrap();
}