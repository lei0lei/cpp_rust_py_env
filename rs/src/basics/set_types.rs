// ======================= Vector =======================
// 动态数组类型用 `Vec<T>` 表示
// 动态数组允许你存储多个值，这些值在内存中一个紧挨着另一个排列，因此访问其中某个元素的成本非常低。动态数组只能存储相同类型的元素，如果你想存储不同类型的元素，可以使用之前讲过的枚举类型或者特征对象

pub fn vec_test() {
    
    let v: Vec<i32> = Vec::new();
    
    let v = vec![1, 2, 3];

    // push更新，v必须可变
    let mut v = Vec::new();
    v.push(1);

    // 自动回收
    {
        let v = vec![1, 2, 3];

            // ...
    } // <- v超出作用域并在此处被删除

    // 下标访问和get访问
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }

    // 遍历
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{i}");
    }

    // enum实现存放不同类型
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String)
    }
    fn show_addr(ip: IpAddr) {
        println!("{:?}",ip);
    }

    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    for ip in v {
        show_addr(ip)
    }

    // 特征对象实现存放不同类型
    trait IpAddr_ {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr_ for V4 {
        fn display(&self) {
            println!("ipv4: {:?}",self.0)
        }
    }
    struct V6(String);
    impl IpAddr_ for V6 {
        fn display(&self) {
            println!("ipv6: {:?}",self.0)
        }
    }

    let v: Vec<Box<dyn IpAddr_>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }

    // 初始化
    let v = vec![0; 3];   // 默认值为 0，初始长度为 3
    let v_from = Vec::from([0, 0, 0]);
    assert_eq!(v, v_from);

    // 容量修改
    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]);    // 附加数据到 v
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.reserve(100);        // 调整 v 的容量，至少要有 100 的容量
    println!("Vector（reserve） 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.shrink_to_fit();     // 释放剩余的容量，一般情况下，不会主动去释放容量
    println!("Vector（shrink_to_fit） 长度是: {}, 容量是: {}", v.len(), v.capacity());


    // 一些常见方法
    let mut v =  vec![1, 2];
    assert!(!v.is_empty());         // 检查 v 是否为空

    v.insert(2, 3);                 // 在指定索引插入数据，索引值不能大于 v 的长度， v: [1, 2, 3] 
    assert_eq!(v.remove(1), 2);     // 移除指定位置的元素并返回, v: [1, 3]
    assert_eq!(v.pop(), Some(3));   // 删除并返回 v 尾部的元素，v: [1]
    assert_eq!(v.pop(), Some(1));   // v: []
    assert_eq!(v.pop(), None);      // 记得 pop 方法返回的是 Option 枚举值
    v.clear();                      // 清空 v, v: []

    let mut v1 = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
    v.append(&mut v1);              // 将 v1 中的所有元素附加到 v 中, v1: []
    v.truncate(1);                  // 截断到指定长度，多余的元素被删除, v: [11]
    v.retain(|x| *x > 10);          // 保留满足条件的元素，即删除不满足条件的元素

    let mut v = vec![11, 22, 33, 44, 55];
    // 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
    let mut m: Vec<_> = v.drain(1..=3).collect();    

    let v2 = m.split_off(1);        // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]

    // 数组切片的方式获取元素
    let v = vec![11, 22, 33, 44, 55];
    let slice = &v[1..=3];
    assert_eq!(slice, &[22, 33, 44]);

    // 在 rust 里，实现了两种排序算法，分别为稳定的排序 `sort` 和 `sort_by`，以及非稳定排序 `sort_unstable` 和 `sort_unstable_by`。
    let mut vec = vec![1, 5, 10, 2, 15];    
    vec.sort_unstable();    
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);


    // 浮点数没有实现全序Ord只实现了PartialOrd
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];    
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());    
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);

    // 结构体数组排序
    // 实现 `Ord` 需要我们实现 `Ord`、`Eq`、`PartialEq`、`PartialOrd` 这些属性
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    // 定义一个按照年龄倒序排序的对比函数
    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));

    println!("{:?}", people);

}






// ======================= Hashmap ========================
// `HashMap` ==需要手动通过 `use ...` 从标准库中引入
use std::collections::HashMap;

pub fn hashmap_test() {
    let mut my_gems = HashMap::new();

    // 将宝石类型和对应的数量写入表中
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);


    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
    
    println!("{:?}",teams_map);

    // `get` 方法可以获取元素
    // - `get` 方法返回一个 `Option<&i32>` 类型：当查询不到时，会返回一个 `None`，查询到时返回 `Some(&i32)`
    // - `&i32` 是对 `HashMap` 中值的借用，如果不使用借用，可能会发生所有权的转移
    // - `get` 方法的 `key` 参数必须是一个引用，如这里的 `scores.get(&team_name)`，这是因为 `HashMap<K, V>` 的 `get` 方法的签名


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);

    let score: i32 = scores.get(&team_name).copied().unwrap_or(0);

    // ### 更新 HashMap 中的值
    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入

}






































// ================================= VecDeque ==============================
 
 pub fn vec_deque_test() {

 }
 
 
 
 
 
 
 
 
 
 
//  ============================ LinkedList =========================================

pub fn link_list_test() {
}


// BTreeMap

pub fn btree_map_test() {

}


// HashSet
pub fn hashset_test() {

}



// BTreeSet

pub fn btree_set_test() {

}


// BinaryHeap

pub fn binary_heap_test() {
}
