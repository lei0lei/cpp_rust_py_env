

pub fn char_type() {
    // rust中字符用单引号并且是unicode字符
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

}

pub fn bool_type() {
    let t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型
}

pub fn unit_type() {
    // 单元类型就是 `()`,`main` 函数就返回这个单元类型 `()`
    // 可以用 `()` 作为 `map` 的值

}


pub fn statements_expressions_type() -> i32{
    let x = 1;
    let y = 1;
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式

}
// 表达式如果不返回任何值，会隐式地返回一个 `()`

pub fn functions_type() {
    // 函数名和变量名使用[蛇形命名法(snake case)](https://course.rs/practice/naming.html)，例如 `fn add_two() {}`
    // 函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
    // 每个函数参数都需要标注类型

    // - 函数没有返回值，那么返回一个 `()`
    // - 通过 `;` 结尾的语句返回一个 `()`


    // 用 `!` 作函数返回类型的时候，表示该函数永不返回( diverging functions )
}

