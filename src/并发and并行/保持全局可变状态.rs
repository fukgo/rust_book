//保持全局可变状态
use lazy_static::lazy_static;
use std::{error::Error, sync::Mutex};

lazy_static!{
    //static ref 声明一个静态引用
    static ref FRUIT:Mutex<Vec<String>>=Mutex::new(Vec::new());
}
fn insert(fruit:&str)->Result<(),Box<dyn Error>>{
    let mut db = FRUIT.lock().map_err(|_|"Filed to acquire MutexGuard ")?;
    db.push(fruit.to_string());
    Ok(())
}

fn main()->Result<(),Box<dyn Error>>{
    insert("apple");
    insert("peach");
    insert("grape");
    {
        let db = FRUIT.lock().map_err(|_|"Failed to acquire MutexGuard")?;
        db.iter().enumerate().for_each(|(i,item)|println!("{}:{}",i,item));
    }
    insert("strawberry");
    Ok(())
}