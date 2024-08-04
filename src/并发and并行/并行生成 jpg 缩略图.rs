//当前目录中的所有 .jpg 图像文件生成缩略图，然后将生成的缩略图保存在一个名为 thumbnails 的新文件夹中。
use error_chain::error_chain;

use std::path::Path;
use std::fs::create_dir_all;

use error_chain::ChainedError;
//glob 库在 Rust 中用于文件路径的模式匹配。它提供了一种方式来查找符合特定模式的所有文件和目录
use glob::{glob_with, MatchOptions};
//处理图像操作的过滤类型
use image::ImageError;
use image::imageops::FilterType;
use rayon::prelude::*;

//error_chain! 宏定义的错误类型可以自动从列出的 "foreign" 错误类型转换
error_chain!{
    foreign_links{
        Image(ImageError);
        Io(std::io::Error);
        Glob(glob::PatternError);
    }
}
fn main()->Result<()>{
    //创建一个默认配置的 MatchOptions 实例
    let options: MatchOptions = Default::default();
    //x.ok() 会将成功的 Result 转换为 Some，并过滤掉失败的 Result
    let files:Vec<_> = glob_with("img/*.jpg", options)?.filter_map(|x|x.ok()).collect();

     // 如果没有找到 .jpg 文件，返回错误
    if files.len()==0{
       error_chain::bail!("No .jpg files found in current directory")
    };
    // 创建缩略图目录
    let thumb_dir = "thumbnails";
    //递归创建目录以及父目录，如果存在就忽略，只有发生io错误会返回
    create_dir_all(thumb_dir)?;
    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);
      // 并行处理每个 .jpg 文件
    let images_failures:Vec<_>=files
        .par_iter()
        .map(|path|{
            make_thumbnail(path, thumb_dir, 300)
            //e.chain_err(|| path.display().to_string()) 使用 chain_err 方法为错误添加上下文信息。
            //path.display().to_string() 将路径转换为可读字符串，以便在错误信息中包含路径。
            .map_err(|e|e.chain_err(||path.display().to_string()))
        })
        .filter_map(|x|x.err())
        .collect();
    Ok(())

/*
AsRef 特性是一个通用的转换工具，它允许将一个类型引用转换为另一个类型引用。
AsRef<Path> 特性意味着类型实现了将自身转换为 Path 引用的能力。这在处理文件路径时特别有用，
因为它允许函数接受多种类型的路径参数，而不必对每种类型进行特殊处理。
接受多种类型的路径参数，而不需要为每种类型编写单独的函数
接受 &str、String、&Path 和 PathBuf 类型的参数
 */

}
fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{   //as_ref() 是 AsRef 特性提供的一个方法，用于将一个类型的引用转换为另一个类型的引用。
    let img = image::open(original.as_ref())?;//thumb_dir.as_ref()将 thumb_dir 转换为 Path 引用。
    let file_path = thumb_dir.as_ref().join(original.as_ref().file_name().unwrap());
    //resize调整图像大小，使用 FilterType::Nearest 进行最近邻插值。
    Ok(img.resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}

fn test() {
    let path = Path::new("/some/path/to/a/file.jpg");
    println!("Path: {:?}\n{}", path,path.display());
} 
