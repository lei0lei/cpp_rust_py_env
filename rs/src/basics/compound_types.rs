

pub fn string_slice_type() {

    // 切片就是对 `String` 类型中某一部分的引用
    // `hello` 没有引用整个 `String s`，而是引用了 `s` 的一部分内容，通过 `[0..5]` 的方式来指定。
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    // 如果你的切片想要包含 `String` 的最后一个字节，则可以这样使用：
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[4..len];
    let slice = &s[4..];

    // 也可以截取完整的 `String` 切片
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];


    // 字符串切片的类型标识是 `&str`，因此我们可以这样声明一个函数，输入 `String` 类型，返回它的切片：`fn first_word(s: &String) -> &str` 。

    // 切片是对集合的部分引用，因此不仅仅字符串有切片，其它集合类型也有，例如数组
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let s: &str = "Hello, world!";
    // 该切片指向了程序可执行文件中的某个点，这也是为什么字符串字面量是不可变的，因为 `&str` 是一个不可变引用。


    // 将 `String` 类型转为 `&str` 类型,这种灵活用法是因为 **`deref` 隐式强制转换**
    fn say_hello(s: &str) {
        println!("{}",s);
    }
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());



    // 在其它语言中，使用索引的方式访问字符串的某个字符或者子串是很正常的行为，但是在 Rust 中就会报错
    // let s1 = String::from("hello");
    // let h = s1[0];

    // 由于 `String` 是可变字符串，下面介绍 Rust 字符串的修改，添加，删除等常用方法：
    // 在字符串尾部可以使用 `push()` 方法追加字符 `char`
    let mut s = String::from("Hello ");
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);
    s.push('!');
    println!("追加字符 push() -> {}", s);

    // 可以使用 `insert()` 方法插入单个字符 `char`，也可以使用 `insert_str()` 方法插入字符串字面量
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    // 如果想要把字符串中的某个字符串替换成其它的字符串，那可以使用 `replace()` 方法
    // 该方法可适用于 `String` 和 `&str` 类型。`replace()` 方法接收两个参数，第一个参数是要被替换的字符串，第二个参数是新的字符串。该方法会替换所有匹配到的字符串。**该方法是返回一个新的字符串，而不是操作原来的字符串**。
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);

    // 该方法可适用于 `String` 和 `&str` 类型。`replacen()` 方法接收三个参数，前两个参数与 `replace()` 方法一样，第三个参数则表示替换的个数。**该方法是返回一个新的字符串，而不是操作原来的字符串**。
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);



    // 该方法仅适用于 `String` 类型。`replace_range` 接收两个参数，第一个参数是要替换字符串的范围（Range），第二个参数是新的字符串。**该方法是直接操作原来的字符串，不会返回新的字符串。该方法需要使用 `mut` 关键字修饰**。
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);


    
    // 与字符串删除相关的方法有 4 个，它们分别是 `pop()`，`remove()`，`truncate()`，`clear()`。这四个方法仅适用于 `String` 类型。
    // `pop` —— 删除并返回字符串的最后一个字符
    // **该方法是直接操作原来的字符串**。但是存在返回值，其返回值是一个 `Option` 类型，如果字符串为空，则返回 `None`。
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);


    // `remove` —— 删除并返回字符串中指定位置的字符
    // **该方法是直接操作原来的字符串**。但是存在返回值，其返回值是删除位置的字符串，只接收一个参数，表示该字符起始索引位置。`remove()` 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    // `truncate` —— 删除字符串中从指定位置开始到结尾的全部字符
    // **该方法是直接操作原来的字符串**。无返回值。该方法 `truncate()` 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    // `clear` —— 清空字符串
    // **该方法是直接操作原来的字符串**。调用后，删除字符串中的所有字符，相当于 `truncate()` 方法参数为 0 的时候。
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // 使用 `+` 或者 `+=` 连接字符串
    // 使用 `+` 或者 `+=` 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型
    // **`+` 是返回一个新的字符串，所以变量声明可以不需要 `mut` 关键字修饰**。
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!";

    println!("连接字符串 + -> {}", result);

    // `add()` 方法的定义：fn add(self, s: &str) -> String
    // `self` 是 `String` 类型的字符串 `s1`，该函数说明，只能将 `&str` 类型的字符串切片添加到 `String` 类型的 `s1` 上，
    // 然后返回一个新的 `String` 类型，所以 `let s3 = s1 + &s2;` 就很好解释了，将 `String` 类型的 `s1` 与 `&str` 类型的 `s2` 进行相加，最终得到 `String` 类型的 `s3`。

    // 可以通过转义的方式 `\` 输出 ASCII 和 Unicode 字符
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);


    // 在某些情况下，可能你会希望保持字符串的原样
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中包含 # 号，可以在开头和结尾加多个 # 号，最多加255个，只需保证与字符串中连续 # 号的个数不超过开头和结尾的 # 号的个数即可
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);


    // 如果你想要以 Unicode 字符的方式遍历字符串，最好的办法是使用 `chars` 方法，例如：
    for c in "中国人".chars() {
        println!("{}", c);
    }

    // 这种方式是返回字符串的底层字节数组表现形式：
    for b in "中国人".bytes() {
        println!("{}", b);
    }


}


pub fn tuple_type() {
    // 元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的。
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 用模式匹配解构元组
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // 模式匹配可以让我们一次性把元组中的值全部或者部分获取出来，如果只想要访问某个特定元素，那模式匹配就略显繁琐，对此，Rust 提供了 `.` 的访问方式：
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


}


pub fn struct_type() {

    // 一个结构体由几部分组成：
    // - 通过关键字 `struct` 定义
    // - 一个清晰明确的结构体 `名称`
    // - 几个有名字的结构体 `字段

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // 为了使用上述结构体，我们需要创建 `User` 结构体的**实例**：
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 通过 `.` 操作符即可访问结构体实例内部的字段值，也可以修改它们：
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    // 需要注意的是，必须要将结构体实例声明为可变的，才能修改其中的字段，Rust 不支持将某个结构体某个字段标记为可变

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    // 当函数参数和结构体字段同名时，可以直接使用缩略的方式进行初始化


    // 有一种情况很常见：根据已有的结构体实例，创建新的结构体实例，例如根据已有的 `user1` 实例来构建 `user2`：
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // `..` 语法表明凡是我们没有显式声明的字段，全部从 `user1` 中自动获取。需要注意的是 `..user1` 必须在结构体的尾部使用。
    // 结构体更新语法跟赋值语句 = 非常相像，因此在上面代码中，user1 的部分字段所有权被转移到 user2 中：username 字段发生了所有权转移，作为结果，user1 无法再被使用。
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // 结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 单元结构体没有任何字段和属性,如果你定义一个类型，但是不关心该类型的内容，只关心它的行为时，就可以使用 `单元结构体`：
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    // impl SomeTrait for AlwaysEqual {
    // }

    // 在之前的 `User` 结构体的定义中，有一处细节：我们使用了自身拥有所有权的 `String` 类型而不是基于引用的 `&str` 字符串切片类型。
    // 这是一个有意而为之的选择：因为我们想要这个结构体拥有它所有的数据，而不是从其它地方借用数据。
    // 也可以让 `User` 结构体从其它对象借用数据，不过这么做，就需要引入生命周期(lifetimes)这个新概念,下面的例子编译器会抱怨它需要生命周期标识符
    // struct User {
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    //     active: bool,
    // }

    // fn main() {
    //     let user1 = User {
    //         email: "someone@example.com",
    //         username: "someusername123",
    //         active: true,
    //         sign_in_count: 1,
    //     };
    // }

    // 在前面的代码中我们使用 `#[derive(Debug)]` 对结构体进行了标记，这样才能使用 `println!("{:?}", s);` 
    // Rust 默认不会为我们实现 `Debug`，为了实现，有两种方式可以选择：

    // - 手动实现
    // - 使用 `derive` 派生实现

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);


    // 还有一个简单的输出 debug 信息的方法，那就是使用 `dbg!` 宏
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);


}


pub fn enum_type() {
    // 枚举(enum 或 enumeration)允许你通过列举可能的成员来定义一个**枚举类型**
    // **枚举类型是一个类型，它会包含所有可能的枚举成员，而枚举值是该类型中的具体某个成员的实例。**
    enum PokerSuit {
        Clubs,
        Spades,
        Diamonds,
        Hearts,
    }

    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    // 我们通过 `::` 操作符来访问 `PokerSuit` 下的具体成员

    enum PokerCard {
        Clubs(u8),
        Spades(u8),
        Diamonds(u8),
        Hearts(u8),
    }
   let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds(13);

    // 不仅如此，同一个枚举类型下的不同成员还能持有不同的数据类型，例如让某些花色打印 `1-13` 的字样，另外的花色打印上 `A-K` 的字样：
    enum PokerCard1 {
        Clubs(u8),
        Spades(u8),
        Diamonds(char),
        Hearts(char),
    }

    // **任何类型的数据都可以放入枚举成员中**：例如字符串、数值、结构体甚至另一个枚举。
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Rust 吸取了众多教训，决定抛弃 `null`，而改为使用 `Option` 枚举变量来表述这种结果。
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // 其中 `T` 是泛型参数，`Some(T)`表示该枚举成员的数据类型是 `T`，换句话说，`Some` 可以包含任何类型的数据。

    // 为了使用 `Option<T>` 值，需要编写处理每个成员的代码。你想要一些代码只当拥有 `Some(T)` 值时运行，允许这些代码使用其中的 `T`。
    // 也希望一些代码在值为 `None` 时运行，这些代码并没有一个可用的 `T` 值。`match` 表达式就是这么一个处理枚举的控制流结构：
    // 它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据。

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}


pub fn array_type() {
    // 在 Rust 中，最常用的数组有两种，第一种是速度很快但是长度固定的 `array`，第二种是可动态增长的但是有性能损耗的 `Vector`
    // 当你不确定是使用数组还是动态数组时，那就应该使用后者
    // - 长度固定
    // - 元素必须有相同的类型
    // - 依次线性排列

    // 定义
    let a = [1, 2, 3, 4, 5];
    // 在一些时候，还需要为**数组声明类型**，如下所示：
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 可以使用下面的语法初始化一个**某个值重复出现 N 次的数组**：
    let a = [3; 5];

    // 因为数组是连续存放元素的，因此可以通过索引的方式来访问存放其中的元素：
    let a = [9, 8, 7, 6, 5];
    let first = a[0]; // 获取a数组第一个元素
    let second = a[1]; // 获取第二个元素


    // 实际开发中还会碰到一种情况，就是**数组元素是非基本类型**的
    // let array = [String::from("rust is good!"); 8];
    // **正确的写法**，应该调用`std::array::from_fn`
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));

    // 数组切片允许我们引用数组的一部分
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}