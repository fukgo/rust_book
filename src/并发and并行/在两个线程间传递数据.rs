use std::thread;
use std::time;
use crossbeam_channel::unbounded;
//unbounded创建一个无界通道，通道内的消息数量没有上限
fn main() {

    //crossbeam::scope函数在Rust中提供了一种创建线程的方式，它允许线程安全地共享栈上的数据。
    //这是通过确保子线程在scope函数返回之前完成执行来实现的。
    let (sender,recver)=unbounded();
    let num_msgs = 1000;
    crossbeam::scope(|s|{
        s.spawn(|_|{
            for i in 0..num_msgs{
                sender.send(i).unwrap();
                //创建一个表示特定毫秒数的 Duration 对象。
                //thread::sleep(time::Duration::from_millis(100));
            }
        });


    }).unwrap();
    //// 主线程接收并打印消息
    for _ in 0..num_msgs{
        let msg = recver.recv().unwrap();
        println!("Received {}",msg)
    }
}