use error_chain::error_chain;

//将绘制分形的线程分派到线程池
use std::{iter, sync::mpsc::{channel,Receiver,RecvError}};
//创建线程池
use threadpool::ThreadPool;
use num::complex::Complex;
use image::{ImageBuffer,Pixel,Rgb};
//error_chain! 宏定义的错误类型可以自动从列出的 "foreign" 错误类型转换
error_chain!{
    //foreign_links 块用于定义外部错误类型的链接
    foreign_links{
        MpscRecv(RecvError);
        Io(std::io::Error);
    }
}

// 定义函数，将波长转换为 RGB 颜色
fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
    let wave = wavelength as f32;

    let (r, g, b) = match wavelength {
        380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
        440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
        490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
        510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
        580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
        645..=780 => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0),
    };

    let factor = match wavelength {
        380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
        701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
        _ => 1.0,
    };

    let (r, g, b) = (normalize(r, factor), normalize(g, factor), normalize(b, factor));
    Rgb::from_channels(r, g, b, 0)
}
// 将茱莉亚集距离映射为强度值，计算茱莉亚集的值
fn julia(c: Complex<f32>, x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
    let width = width as f32;
    let height = height as f32;

    let mut z = Complex {
        // scale and translate the point to image coordinates
        re: 3.0 * (x as f32 - 0.5 * width) / width,
        im: 2.0 * (y as f32 - 0.5 * height) / height,
    };

    let mut i = 0;
    for t in 0..max_iter {
        if z.norm() >= 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }
    i
}

// 规格 RGB 颜色值范围内的强度值
fn normalize(color: f32, factor: f32) -> u8 {
    ((color * factor).powf(0.8) * 255.) as u8
}
fn main()->Result<()>{
    let (width,height) = (1920,1080);
    //指定宽度和高度的输出图像分配内存
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    //定义迭代次数;
    let iterations = 300;
    // 定义复数 c
    let c = Complex::new(-0.8, 0.156);
    let pool = ThreadPool::new(num_cpus::get());
    //创建一个用于线程间用于通信的通道
    let (sender,receiver)=channel();
    // 对于图像中的每一个像素，都在一个新的线程中计算其对应的茱莉亚集的值，并将计算结果发送到主线程
    for y in 0..height{
        let sender_clone = sender.clone();
        pool.execute(move ||for x in 0..width{
            let i = julia(c,x,y,width,height,iterations);
            let pixel = wavelength_to_rgb(380+i*400/iterations);
            sender_clone.send((x,y,pixel)).expect("Could not send data!");
        });
    }
    // 主线程接收计算结果，并将结果转换为颜色值，然后将颜色值设置到对应的像素上
    for _ in 0..(width*height){
        let (x,y,pixel)=receiver.recv()?;
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save("out.png");
    Ok(())
}