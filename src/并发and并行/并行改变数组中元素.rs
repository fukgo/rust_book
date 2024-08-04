//并行改变数组中元素

use rand::Rng;
use rayon::prelude::*;
fn main() {
    //100个全0数组
    let mut arr = [0;100];
    let mut rng = rand::thread_rng();
    for i in 0..100{
        arr[i]=rng.gen_range(0..100);
    }
    println!("{:?}",arr); 
    arr.par_iter_mut().for_each(|val|*val-=10);
    println!("{:?}",arr); 

}
fn iter_run(){
    let mut rng = rand::thread_rng();
    let mut arr:Vec<_> = (0..100).map(|_| rng.gen_range(0..100)).collect();
    println!("{:?}",arr);
}