use rand::Rng;
use rayon::prelude::*;

fn main(){
    //使用 rayon::find_any 和 par_iter 并行搜索 vector 集合，以查找满足指定闭包中的断言的元素。
    //如果有多个元素满足 rayon::find_any 闭包参数中定义的断言，
    //rayon 将返回搜索发现的第一个元素，但不一定是 vector 集合的第一个元素。
    let mut rng = rand::thread_rng();
    let v:Vec<_> = (0..100).map(|_|rng.gen_range(0..100)).collect();
    /*
    在闭包参数中使用 &&x 时，Rust 会自动进行解引用（dereferencing）。
    这是 Rust 的一种特性，称为解引用强制多态（deref coercions）。
    这意味着，即使 x 是一个对引用的引用，你也可以像处理普通的值一样处理 x。
     */
    let f1 = v.par_iter().find_any(|x|**x==9);
    let f2 = v.par_iter().find_any(|x|**x%2==0 && **x>6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);
    assert_eq!(f1,Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
}