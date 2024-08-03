
// /生成短期线程
fn main(){
    let arr = &[1,23,45234,52342,524];
    let max = find_max(arr);
    assert_eq!(max,Some(52342));
}
fn find_max(arr:&[i32])->Option<i32>{
    const THREADS:usize = 2;
    if arr.len<=THREADS{
        return arr.iter().cloned().max();
    }
    let mid = arr.len()/2;
    //从索引处分割切片;左闭右开
    let (left,right) = arr.split_at(mid);
    //使用crossbeam库创建一个新的线程范围。在这个范围内，可以创建新的线程，并等待它们完成
    crossbeam::scope(|s|{
        let thread_l = s.spawn(|_|find_max(left));
        let thread_r = s.spawn(|_|find_max(right));
        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;
        //比较两个最大值，找出整个数组的最大值。
        Some(max_l.max(max_r))
    }).unwrap();



}