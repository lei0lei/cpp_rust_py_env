// Rust 使用 `impl` 来定义方法

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
// ===============================================================================
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

// 在 `area` 的签名中，我们使用 `&self` 替代 `rectangle: &Rectangle`，
// `&self` 其实是 `self: &Self` 的简写（注意大小写）。在一个 `impl` 块内，
// `Self` 指代被实现方法的结构体类型，`self` 指代此类型的实例，换句话说，`self` 指代的是 `Rectangle` 结构体实例

// - `self` 表示 `Rectangle` 的所有权转移到该方法中，这种形式用的较少
// - `&self` 表示该方法对 `Rectangle` 的不可变借用
// - `&mut self` 表示可变借用
pub fn impl_method(){
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

}

// 在 Rust 中，允许方法名跟结构体的字段名相同
// 当我们使用 `rect1.width()` 时，Rust 知道我们调用的是它的方法，如果使用 `rect1.width`，则是访问它的字段
// 一般来说，方法跟字段同名，往往适用于实现 `getter` 访问器