
//整数 Vector 排序
fn sort_int(){
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}

//浮点数 Vector 排序
fn float_sort(){
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    //partial_cmp 是 PartialOrd trait 的方法，适用于可以部分比较的类型（如浮点数）。
    // 它返回一个 Option<Ordering>，表示两个值的相对顺序
    vec.sort_by(|a,b|a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}

//结构体 Vector 排序
#[derive(Debug,Eq,Ord,PartialOrd, PartialEq)]
struct Person{
    name:String,
    age:u32,
}
impl Person{
    pub fn new(name:String,age:u32)->Self{
        Person{
            name,
            age
        }
    }
}
fn main() {
    sort_int();
    float_sort();
    let mut peoples = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    //字典顺序（也称为字母顺序或词典顺序）进行
    peoples.sort();
    assert_eq!(
        peoples,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]);

    // 根据 age 值对 people 进行排序
    peoples.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        peoples,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]);
}