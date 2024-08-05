use std::io::{BufReader, Read, Write};
use error_chain::error_chain;
use data_encoding::HEXUPPER;
use std::fs::File;
//ring 是一个 Rust 语言的加密库，提供了一系列的密码学算法，包括摘要、加密、签名等 
use ring::digest::{Context, Digest, SHA256};
error_chain!(
    foreign_links{
        Io(std::io::Error);
        Decode(data_encoding::DecodeError);
    }
);
fn sha256_digest<R>(mut reader:R)-> Result<Digest>
where R:Read
{   //创建一个新的 Context 对象，这个对象将用于计算 SHA-256 摘要。
    let mut context = Context::new(&SHA256);
    //创建1024缓冲区用来从reader中读取数据
    let mut buffer = [0; 1024];
    loop {
        //从 reader 中读取数据，并将数据放入 buffer 中。read 方法返回读取的字节数，这个值被存储在 count 中。
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        //添加到 SHA-256 摘要的计算上下文
        context.update(&buffer[..count]);
        
    }
    //pub fn finish(self) -> Digest，返回一个 Digest 对象
    Ok(context.finish())
          
}


fn main()->Result<()>{
    let path = "out.png";
    let mut otuput = File::create(path)?;
    //write!可以为实现Write trait实现写入操作
    write!(otuput,"We will generate a digest of this text")?;
    //打开刚才创建的文件，并返回一个 File 对象
    let input = File::open(path)?;
    //创建一个新的缓冲读取器（buffered reader）。这个读取器可以用于高效地从文件中读取数据。
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;
    //将 SHA-256 摘要（存储在 digest 变量中）转换为十六进制字符串
    //digest.as_ref() 将 digest 转换为字节切片
    //HEXUPPER.encode(...) 是使用 HEXUPPER 对象将字节切片编码为一个十六进制字符串
    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));
    Ok(())


}