//Map-reduce 并行计算
use rayon::prelude::*;
struct Person{
    age:u32,
}

fn main(){
    let v:Vec<Person>=vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];
    let num_over_30 = v.par_iter().filter(|val|val.age>30).count();
    //rayon::map 对每个元素执行一次计算，创建一个新的迭代；然后，
    //基于前一次的 reduce 计算结果和当前元素一起

    //使用加法将迭代器中的所有元素归约为一个单一的值
    //reduce 方法接受两个参数：一个是生成初始值的闭包（在这里是 || 0，总是返回 0），另一个是将两个元素组合成一个元素的闭包（|x, y| x + y，它返回两个元素的和）
    let sum_over_30 = v.par_iter().map(|x|x.age).filter(|val|*val>30).reduce(|| 0, |x, y| x + y);
    //x.age 不需要显式解引用是因为 Rust 的自动解引用特性，而 *x > 30 需要显式解引用是因为你需要获取实际的年龄值来进行比较。
    let alt_sum_30 = v.par_iter().map(|x|x.age).filter(|x|*x>30).sum::<u32>();
    let avg_over_30 = sum_over_30 as f32 / num_over_30 as f32;
    let alt_avg_over_30 = alt_sum_30 as f32/ num_over_30 as f32;

    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
}

