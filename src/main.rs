//在日志信息中包含时间戳
use core::error;
use std::error::Error;
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
//log4rs 将日志输出配置到自定义位置。log4rs 可以使用外部 YAML 文件或生成器配置
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

//使用 Builder 创建自定义记录器配置
//每个日志项调用 Local::now 以获取本地时区中的当前 DateTime，
//并使用 DateTime::format 和 strftime::specifiers 来格式化最终日志中使用的时间戳。
fn add_time(){
    Builder::new()
    .format(|buf,record|{
        write!(buf,"{} [{}] - {}",Local::now().format("%Y-%m-%d %H:%M:%S"),record.level(),record.args())
    })
    .filter(None, LevelFilter::Info)
    .init();
    log::warn!("warn");
    log::info!("info");
    log::debug!("debug");
}
fn setup_logger() -> Result<(), Box<dyn Error>> {


    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(fern::log_file("output.log")?) // 输出到文件
        .apply()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger()?;

    log::warn!("warn");
    log::info!("info");
    log::debug!("debug");

    Ok(())
}