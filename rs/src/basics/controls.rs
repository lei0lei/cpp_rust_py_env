

pub fn loop_control_run(){

    // for循环
    for i in 1..=5 {
        println!("{}", i);
    }
    // 使用 `for` 时我们往往使用集合的引用形式，除非你不想在后面的代码中继续使用该集合（比如我们这里使用了 `container` 的引用）。
    // 如果不使用引用的话，所有权会被转移（move）到 `for` 语句块中，后面就无法再使用这个集合了)：
    // for item in &container {
    // // ...
    // }

    // 对于实现了 `copy` 特征的数组（例如 [i32; 10]）而言， `for item in arr` 并不会把 `arr` 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 `arr` 。
    // for item in &mut collection {
    // // ...
    // }

    // 在循环中**获取元素的索引**
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }



    // 第一种
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
    let item = collection[i];
    // ...
    }

    // 第二种
    for item in collection {

    }
    // `collection[index]` 的索引访问，会因为边界检查(Bounds Checking)导致运行时的性能损耗
    // 第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的

    // 第一种如果 collection 是 Vec 或切片，编译器无法保证 i 迭代一定能触发连续的内存访问优化
    // 第二种Rust 的迭代器对数组、切片、Vec 都实现了 顺序遍历。编译器知道这是“从头到尾顺序访问”，更容易 自动优化成线性连续内存访问（比如循环展开、SIMD）。


    // 使用 `continue` 可以跳过当前当次的循环，开始下次的循环：
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    // 使用 `break` 可以直接跳出当前整个循环：
     for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }




    // while循环
    // 如果你需要一个条件来循环，当该条件为 `true` 时，继续循环，条件为 `false`，跳出循环，那么 `while` 就非常适用
    let mut n = 0;

    while n <= 5  {
        println!("{}!", n);

        n = n + 1;
    }



    // loop循环
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

}


pub fn condition_control_run(){
    // `if else` **表达式**
    let condition = true;
    let number = if condition {
            5
        } else {
            6
        };


    // 多分支
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


}