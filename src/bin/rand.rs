use std::process::id;
use std::thread;
use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError,Standard};
use rand::distributions::Alphanumeric;
use rand::seq::index::sample;

fn single_thread(){
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("u8: {}", n1);
    println!("u16: {}", n2);
    println!("u32: {}", rng.gen::<u32>());
    println!("i32: {}", rng.gen::<i32>());
    println!("f32: {}", rng.gen::<f32>());
}
fn multiple_thread(){
    let handles:Vec<_> = (0..10).map(|_|{
        thread::spawn(||{
            let mut rng = rand::thread_rng();
            println!("random num: {}", rng.gen::<u32>());
        })
    }).collect();

    for handle in handles{
        //handle.join()等待所有线程完成
        //等待所有已经启动的线程完成
        handle.join().unwrap();
    }
}
fn range_rand(){
    let handles:Vec<_> = (0..1000).map(|_|{
        thread::spawn(||{
            let mut rng = rand::thread_rng();
            let int = rng.gen_range(0..1000);
            let float = rng.gen_range(0.0..10.0);
            println!("{},{}",int,float);
        })
    }).collect();
    for handle in handles{
        handle.join().unwrap();
    }
}
fn uniform(){
    //rand库中用于处理各种概率分布的模块。
    use rand::distributions::{Distribution, Uniform};
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("roll the die {}",throw);
        if throw==6{
            break;
        }
    }
}

//生成给定分布随机数
//rand_distr crate 提供其它的分布类型,使用正态（Normal）分布
fn normal_random(){
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0,3.0).unwrap();
    //创建满足正态分布的随机数
    let v = normal.sample(&mut rng);
    println!("{} is from a Normal(2,9) distribution",v);

}

//生成自定义类型随机值
#[derive(Debug)]
struct Point{
    x:i32,
    y:i32,
}
//为 Distribution 这个泛型接口实现了对 Point 类型的支持
//实现 Distribution 特性，使其可以生成 Point 类型的实例。
impl Distribution<Point> for Standard{
    //?Sized动态大小
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x,rand_y)=rng.gen();
        Point{
            x:rand_x,
            y:rand_y
        }
    }
}
fn defined_rond(){
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32,bool,f64)>();
    let rand_point:Point = rng.gen();
    println!("random tuple {:?}",rand_tuple);
    println!("random point: {:?}",rand_point);

}
//从一组字母数字字符创建随机密码,随机生成一个给定长度的 ASCII 字符串，范围为 A-Z，a-z，0-9，使用字母数字样本。
fn generate_pass(){
    let rand_string:Vec<_> = rand::thread_rng()
        //创建一个无限的迭代器，该迭代器会从 Alphanumeric 分布中采样。
        //也就是说，这个迭代器会不断生成随机的字母和数字字符。
        .sample_iter(&Alphanumeric)
        //从迭代器中获取前 30 个值。也就是说，它将生成 30 个随机的字母和数字字符。
        .take(30)
        .map(char::from)
        .collect();
    println!("random pass: {:?}",rand_string.iter().collect::<String>());

}
//从一组用户定义字符创建随机密码
fn defined_pass(){
    const CHARSET:&[u8]=b"!@#$%^&*()_+1234567890";
    const PASS_LEN:usize = 30;
    let mut rng = rand::thread_rng();
    let pass:Vec<_> = (0..PASS_LEN).map(|_|{
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    }).collect();
    println!("{:?}",pass.iter().collect::<String>());
}
fn main(){
    defined_pass();
}