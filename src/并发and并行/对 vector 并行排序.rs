
 /*分配空字符串 vector；然后，通过 par_iter_mut().for_each 并行对
 vector 填充随机值。尽管存在多种选择，可以对可枚举数据类型进行排序，
 但 par_sort_unstable 通常比稳定排序（相同的值排序后相对顺序不变）算法快。 */
 use rand::distributions::Alphanumeric;
 use rand::{thread_rng, Rng};
 use rayon::prelude::*;
 
 fn main() {
     let mut vec = vec![String::new(); 1000];
     vec.par_iter_mut().for_each(|val| {
         let mut rng = thread_rng();
         *val = (0..5).map(|_| rng.sample(&Alphanumeric) as char).collect::<String>();
     });
     vec.par_sort_unstable();
     println!("{:?}", vec);
 }