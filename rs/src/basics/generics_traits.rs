


// 下面是一个泛型的最简单的例子
// 使用泛型参数，有一个先决条件，必需在使用前对其进行声明
// fn add<T>(a:T, b:T) -> T {
//     a + b
// }

// =================== 函数泛型 =================
// 我们可以这样理解这个函数定义：函数 `largest` 有泛型类型 `T`，它有个参数 `list`，其类型是元素为 `T` 的数组切片，最后，该函数返回值的类型也是 `T`。
// 因为 `T` 可以是任何类型，但不是所有的类型都能进行比较，因此上面的错误中，编译器建议我们给 `T` 添加一个类型限制：使用 `std::cmp::PartialOrd` 特征（Trait）对 `T` 进行限制
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// 同样的，不是所有 `T` 类型都能进行相加操作，因此我们需要用 `std::ops::Add<Output = T>` 对 `T` 进行限制：
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}


// 有时候，编译器无法推断你想要的泛型参数：
use std::fmt::Display;

fn create_and_print<T>() where T: From<i32> + Display {
    let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
    println!("a is: {}", a);
}

// 结构体中的字段类型也可以用泛型来定义，下面代码定义了一个坐标点 `Point`，它可以存放任何类型的坐标值：
struct Point<T> {
    x: T,
    y: T,
}

// 枚举中的泛型比如Option和下方的Result
enum Result<T, E> {
    Ok(T),
    Err(E),
}


// ====================== 方法泛型 ==========================
// 泛型参数前，依然需要提前声明：`impl<T>`，只有提前声明了，我们才能在`Point<T>`中使用它，这样 Rust 就知道 `Point` 的尖括号中的类型是泛型而不是具体类型。需要注意的是，这里的 `Point<T>` 不再是泛型声明，而是一个完整的结构体类型，因为我们定义的结构体就是 `Point<T>` 而不再是 `Point`。
struct Point_1<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point_1<T, U> {
    fn mixup<V, W>(self, other: Point_1<V, W>) -> Point_1<T, W> {
        Point_1 {
            x: self.x,
            y: other.y,
        }
    }
}

// 不仅能定义基于 `T` 的方法，还能针对特定的具体类型，进行方法定义：
// 这段代码意味着 `Point<f32>` 类型会有一个方法 `distance_from_origin`，而其他 `T` 不是 `f32` 类型的 `Point<T>` 实例则没有定义此方法。
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ======================== 值泛型 ======================
// `[i32; 2]` 和 `[i32; 3]` 是不同的数组类型,这就是值泛型

// fn display_array<T: std::fmt::Debug>(arr: &[T]) {
//     println!("{:?}", arr);
// }

// 有了 const 泛型，也就是针对值的泛型，正好可以用于处理数组长度的问题
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}


// ======================== const 函数 ======================
// `const fn` 允许我们在编译期对函数进行求值
// 要定义一个常量函数，只需要在函数声明前加上 `const` 关键字。
// const fn add(a: usize, b: usize) -> usize {
//     a + b
// }
// const RESULT: usize = add(5, 10);

// 将 `const fn` 与 `const 泛型` 结合，可以实现更加灵活和高效的代码设计。例如，创建一个固定大小的缓冲区结构，其中缓冲区大小由编译期计算确定：

struct Buffer<const N: usize> {
    data: [u8; N],
}

const fn compute_buffer_size(factor: usize) -> usize {
    factor * 1024
}

pub fn const_fn_run(){
    const SIZE: usize = compute_buffer_size(4);
    let buffer = Buffer::<SIZE> {
        data: [0; SIZE],
    };
    println!("Buffer size: {} bytes", buffer.data.len());
}

// ========================= trait ======================
// 定义trait
// 将 `Summary` 定义成了 `pub` 公开的。这样，如果他人想要使用我们的 `Summary` 特征，则可以引入到他们的包中，然后再进行实现。
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

pub fn trait_example(){
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());
}

// **如果你想要为类型** `A` **实现特征** `T`**，那么** `A` **或者** `T` **至少有一个是在当前作用域中定义的

// 可以在特征中定义具有**默认实现**的方法，这样其它类型无需再实现该方法，或者也可以选择重写该方法：
// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// impl Summary for Post {}

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

// 默认实现允许调用相同特征中的其他方法，哪怕这些方法没有默认实现。


// ====================== 特征作为函数参数 ==========================
// 它的意思是 **实现了`Summary`特征** 的 `item` 参数,这是泛型的一个语法糖
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// 除了单个约束条件，我们还可以指定多个约束条件，例如除了让参数实现 `Summary` 特征外，还可以让参数实现 `Display` 特征以控制它的格式化输出
// pub fn notify(item: &(impl Summary + Display)) {}

// Where 约束
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {}

// 特征约束，可以让我们在指定类型 + 指定特征的条件下去实现方法
// `cmp_display` 方法，并不是所有的 `Pair<T>` 结构体对象都可以拥有，只有 `T` 同时实现了 `Display + PartialOrd` 的 `Pair<T>` 才可以拥有此方法
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 可以有条件地实现特征**，例如，标准库为任何实现了 `Display` 特征的类型实现了 `ToString` 特征：
// impl<T: Display> ToString for T {
//     // --snip--
// }

// 可以通过 `impl Trait` 来说明一个函数返回了一个类型，该类型实现了某个特征
// 这种 `impl Trait` 形式的返回值，在一种场景下非常非常有用，那就是返回的真实类型非常复杂，你不知道该怎么声明时
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}
// 这种返回值方式有一个很大的限制：只能有一个具体的类型
// fn returns_summarizable_1(switch: bool) -> impl Summary {
//     if switch {
//         Post {
//             title: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Weibo {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//         }
//     }
// }
// 以上的代码就无法通过编译，因为它返回了两个不同的类型 `Post` 和 `Weibo`

// =====================`derive` 派生特征======================


// 调用方法需要引入特征

// use std::convert::TryInto;
// let a: i32 = 10;
// let b: u16 = 100;

// let b_ = b.try_into()
//         .unwrap();


// **如果你要使用一个特征的方法，那么你需要将该特征引入当前的作用域中**，我们在上面用到了 `try_into` 方法，因此需要引入对应的特征。
use std::ops::Add;
#[derive(Debug)]
struct Point_2<T: Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point_2<T> {
    type Output = Point_2<T>;

    fn add(self, p: Point_2<T>) -> Point_2<T> {
        Point_2{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add1<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}

// ================= 特征对象 ==================
pub trait Draw {
    fn draw(&self);
}
// 只要组件实现了 `Draw` 特征，就可以调用 `draw` 方法来进行渲染。假设有一个 `Button` 和 `SelectBox` 组件实现了 `Draw` 特征：
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
    }
}
// 此时，还需要一个动态数组来存储这些 UI 对象：
// pub struct Screen {
//     pub components: Vec<?>,
// }

// **特征对象**指向实现了 `Draw` 特征的类型的实例，也就是指向了 `Button` 或者 `SelectBox` 的实例，这种映射关系是存储在一张表中，可以在运行时通过特征对象找到具体调用的类型方法。
// 可以通过 `&` 引用或者 `Box<T>` 智能指针的方式来创建特征对象。


trait Draw_1 {
    fn draw(&self) -> String;
}

impl Draw_1 for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw_1 for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

// - `draw1` 函数的参数是 `Box<dyn Draw>` 形式的特征对象，该特征对象是通过 `Box::new(x)` 的方式创建的
// - `draw2` 函数的参数是 `&dyn Draw` 形式的特征对象，该特征对象是通过 `&x` 的方式创建的
// - `dyn` 关键字只用在特征对象的类型声明上，在创建时无需使用 `dyn`





