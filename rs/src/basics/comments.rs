/*! 这里是包或者模块注释 */


/** `add_two` 将指定值加2
let arg = 5;
let answer = my_crate::add_two(arg);
assert_eq!(7, answer);
*/


/// `add_one` 返回一个[`Option`]类型[`Vec`]
pub fn comment_doc(){
    println!("==== ==== ==== ==== 文档注释使用 ==== ==== ==== ====");
    println!("文档注释使用  /// ");
    println!("多行文档注释使用  /**----*/ ");
    println!("包和模块注释使用  /*!----*/ ");

    // Rust 在文档注释中还提供了一个非常强大的功能，那就是可以实现对外部项的链接
    


}


pub fn comment_line(){
    println!("==== ==== ==== ==== 行注释使用 ==== ==== ==== ====");
    // 这是一个行注释
    println!("单行注释使用  // ");


    /*
        我
        是
        S
        u
        n
        ... 哎，好长!
    */
    println!("多行注释使用  /*----*/ ");

}

/// `add_one` 将指定值加1
///
/// # Examples
///
/// ```
/// use rust_code_examples::basics::comments::doc_test;
/// let arg = 5;
/// let answer = doc_test(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn doc_test(x: i32) -> i32 {
    x + 1
}

/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// use rust_code_examples::basics::comments::panic_test;
/// // panics on division by zero
/// panic_test(10, 0);
/// ```
pub fn panic_test(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}