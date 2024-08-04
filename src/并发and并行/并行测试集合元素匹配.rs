use rayon::prelude::*;

fn main(){
    //至少有一个元素满足使得闭包返回 true，那么 any 方法就返回 true
    let vec = vec![2, 4, 6, 8];
    let has_even = vec.iter().any(|&x| x % 2 == 0);
    println!("{}", has_even); // prints "true"
    //所有元素满足使得闭包返回 true，那么 any 方法就返回 true
    let vec = vec![2, 4, 6, 8];
    let all_even = vec.iter().all(|&x| x % 2 == 0);
    println!("{}", all_even); // prints "true"
    

    ///
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8 ));
    assert!(vec.par_iter().all(|n| *n <= 8 ));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8 ));
    assert!(!vec.par_iter().all(|n| *n <= 8 )); 
}