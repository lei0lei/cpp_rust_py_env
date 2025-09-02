


pub fn ex1_variable_run() {

    // 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
    fn add(i: i32, j: i32) -> i32 {
        // 返回相加值，这里可以省略return
        i + j
    }


    println!("变量使用 示例代码\n\n");
    // 使用let来声明变量，进行绑定，a是不可变的
    // 此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32，有符号32位整数
    // 语句的末尾必须以分号结尾
    let a = 10;
    // 主动指定b的类型为i32
    let b: i32 = 20;
    // 这里有两点值得注意：
    // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
    // 2. c是可变的，mut是mutable的缩写
    let mut c = 30i32;
    // 还能在数值和类型中间添加一个下划线，让可读性更好
    let d = 30_i32;
    // 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
    let e = add(add(a, b), add(c, d));

    // println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
    // 该函数将指定的格式化字符串输出到标准输出中(控制台)
    // {}是占位符，在具体执行过程中，会把e的值代入进来
    println!("( a + b ) + ( c + d ) = {}", e);

}

pub fn ex2_name_rules() {
    println!("命名规则 示例代码\n\n");
    println!( "| -------------------------------   | -----------------------------------    |");
    println!( "| 模块 Modules                      | `snake_case`                            |");
    println!( "| 类型 Types                        | `UpperCamelCase`                        |");
    println!( "| 特征 Traits                       | `UpperCamelCase`                        |");
    println!( "| 枚举 Enumerations                 | `UpperCamelCase`                        |");
    println!( "| 结构体 Structs                    | `UpperCamelCase`                        |");
    println!( "| 函数 Functions                    | `snake_case`                            |");
    println!( "| 方法 Methods                      | `snake_case`                            |");
    println!( "| 通用构造器 General constructors    | `new` or `with_more_details`            |");
    println!( "| 转换构造器 Conversion constructors | `from_some_other_type`                  |");
    println!( "| 宏 Macros                         | `snake_case`                            |");
    println!( "| 局部变量 Local variables          | `snake_case`                             |");
    println!( "| 静态类型 Statics                  | `SCREAMING_SNAKE_CASE`                   |");
    println!( "| 常量 Constants                    | `SCREAMING_SNAKE_CASE`                   |");
    println!( "| 类型参数 Type parameters          | `UpperCamelCase`，通常使用一个大写字母: `T`|");
    println!( "| 生命周期 Lifetimes                | 通常使用小写字母: `'a`，`'de`，`'src`      |");

    println!("\n\n");
    println!( "- **驼峰命名法**，复合词的缩略形式我们认为是一个单独的词语，所以**只对首字母进行大写**");
    println!( "- **蛇形命名法**，缩略词用全小写：`is_xid_start`");
    println!( "- 包名**不应该**使用 `-rs` 或者 `-rust` 作为后缀");
    println!( "- 特征的名称应该使用动词");
    println!( "- **类型转换要遵守 `as_`，`to_`，`into_` 命名惯例**");
    println!("\n\n");
    println!( "| 前缀                     | 开销                          | 所有权                 |");
    println!( "| --------------------     | -----------------------      |------------------      |");
    println!( "|  `as_`                   | 无                           | borrowed -> borrowed   |");
    println!( "|  `to_`                   | 大                           | borrowed -> borrowed  borrowed -> owned (non-Copy types)  owned -> owned (Copy types)|");
    println!( "|  `into_`                 | 可变                         | owned -> owned (non-Copy types)  |");

    println!("\n\n");
    println!( "- 如果 `mut` 限定符在返回类型中出现，那么在命名上也**应该**体现出来");
    println!( "- 在 Rust代码中 `get` 前缀不用于 Getter");
    println!( "- **一个集合上的方法，如果返回迭代器，需遵循命名规则：`iter`，`iter_mut`，`into_iter` (C-ITER)**");
    println!( "- **迭代器的类型应该与产生它的方法名相匹配(C-ITER-TY)**");
    println!( "- 使用 `谓语-宾语-错误` 的词序");
    println!( "- 可以用下划线作为变量名的开头忽略未使用的变量");
    println!("\n\n");
}

pub fn ex3_unpack_run() {

    struct Struct {
            e: i32
        }

    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    // `let` 会重新绑定，而这里仅仅是对之前绑定的变量进行再赋值。
}

pub fn ex4_const_run() {
    const MAX_POINTS: u32 = 100_000;
}

pub fn ex5_variable_shadowing_run() {
    println!("变量遮蔽\n\n");
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

}

pub fn ownership_borrow() {
    println!("所有权与借用\n\n");

    // 值拷贝
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;
    // 这句话会报错因为s1已经无效，所有权从s1转到s2
    // println!("{}, world!", s1);

    // 拷贝指针、长度和容量而不拷贝数据听起来就像浅拷贝，但是又因为 Rust 同时使第一个变量 `s1` 无效了，因此这个操作被称为 **移动(move)**


    // 在 `String` 的例子中 `s1` 持有了通过`String::from("hello")` 创建的值的所有权
    // 而这个例子中，`x` 只是引用了存储在二进制可执行文件( binary )中的字符串 `"hello, world"`，并没有持有所有权。
    // 因此 `let y = x` 中，仅仅是对该引用进行了拷贝，此时 `y` 和 `x` 都引用了同一个字符串
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);



    // **Rust 永远也不会自动创建数据的 “深拷贝”**。因此，任何**自动**的复制都不是深拷贝，可以被认为对运行时性能影响较小。
    // 如果我们**确实**需要深度复制 `String` 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 `clone` 的方法。
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // 对于执行较为频繁的代码（热点路径），使用 `clone` 会极大的降低程序性能
    println!("s1 = {}, s2 = {}", s1, s2);

    println!("\n\n");
    println!("**任何基本类型的组合可以 `Copy` ，不需要分配内存或某种形式资源的类型是可以 `Copy` 的**");
    println!("- 所有整数类型，比如 `u32`");
    println!("- 布尔类型，`bool`，它的值是 `true` 和 `false`");
    println!("- 所有浮点数类型，比如 `f64`");
    println!("- 字符类型，`char`");
    println!("- 元组，当且仅当其包含的类型也都是 `Copy` 的时候。比如，`(i32, i32)` 是 `Copy` 的，但 `(i32, String)` 就不是");
    println!("- 不可变引用 `&T` ，**但是注意：可变引用 `&mut T` 是不可以 Copy的**");
    println!("\n\n");



    // 将值传递给函数，一样会发生 `移动` 或者 `复制`，就跟 `let` 语句一样，下面的代码展示了所有权、作用域的规则：
    fn takes_ownership(some_string: String) { // some_string 进入作用域
        println!("{}", some_string);
    } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

    fn makes_copy(some_integer: i32) { // some_integer 进入作用域
        println!("{}", some_integer);
    } // 这里，some_integer 移出作用域。不会有特殊操作

    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x





    // 函数返回值也有所有权
    fn gives_ownership() -> String {             // gives_ownership 将返回值移动给调用它的函数

        let some_string = String::from("hello"); // some_string 进入作用域.

        some_string                              // 返回 some_string 并移出给调用的函数
    }

    // takes_and_gives_back 将传入字符串并返回该值
    fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

        a_string  // 返回 a_string 并移出给调用的函数
    }

    let s1 = gives_ownership();         // gives_ownership 将返回值移给 s1
    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到takes_and_gives_back 中,它也将返回值移给 s3




    // **获取变量的引用，称之为借用(borrowing)**
    let x = 5;
    let y = &x;
    // `y` 是 `x` 的一个引用
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // 如果希望对 `y` 的值做出断言，必须使用 `*y` 来解出引用所指向的值



    // 用 `s1` 的引用作为参数传递给 `calculate_length` 函数
    fn calculate_length(s: &String) -> usize {
        s.len()
        // 当引用离开作用域后，其指向的值也不会被丢弃
    }
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);



    // 引用指向的值默认也是不可变的, 
    // **同一作用域，特定数据只能有一个可变引用**
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    let mut s = String::from("hello");

    change(&mut s);

    // 下面这段代码会被borrow checker拦下来
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);



    // 可变引用与不可变引用不能同时存在
    // let mut s = String::from("hello");

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题

    // println!("{}, {}, and {}", r1, r2, r3);


    // 在 Rust 中编译器可以确保引用永远也不会变成悬垂状态：当你获取数据的引用后，编译器可以确保数据不会在引用结束前被释放，要想释放数据，必须先停止其引用的使用。


}