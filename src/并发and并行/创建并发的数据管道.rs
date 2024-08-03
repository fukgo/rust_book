use std::thread;
use std::time::Duration;
use crossbeam_channel::bounded;

fn main() {
    // 创建两个有界通道
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);

    // 定义消息数量和工作线程数量
    let n_msgs = 4;
    let n_workers = 2;

    // 创建一个新的线程范围
    crossbeam::scope(|s| {
        // 在一个新的线程中运行源
        s.spawn(|_| {
            for i in 0..n_msgs {
                // 源发送消息
                snd1.send(i).unwrap();
                println!("Source sent {}", i)
            }
            // 关闭第一个通道的发送端
            drop(snd1);
        });

        // 为每个工作线程创建一个新的线程
        for _ in 0..n_workers {
            let (sender, recvr) = (snd2.clone(), rcv1.clone());
            s.spawn(move |_| {
                // 让线程暂停一段时间
                thread::sleep(Duration::from_millis(500));
                for msg in recvr {
                    // 工作线程接收消息，处理消息（这里是将消息乘以2），然后发送到第二个通道
                    println!("Worker {:?} received {}", thread::current().id(), msg);
                    sender.send(msg * 2).unwrap();
                }
            });
        }

        // 关闭第二个通道的发送端
        drop(snd2);

        // 接收器从第二个通道接收消息
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }

    }).unwrap();
}