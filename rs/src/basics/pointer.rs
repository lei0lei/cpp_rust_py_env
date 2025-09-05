

// Box
// `Box<T>`，可以将值分配到堆上,下面的情况可以使用Box
// - 特意的将数据分配在堆上
// - 数据较大时，又不想在转移所有权时进行数据拷贝
// - 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
// - 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型

pub fn box_pointer_test() {

    let a = Box::new(3);
    println!("a = {}", a); // a = 3

    let b = *a + 1;
    println!("b = {}", b); // a = 3

    let arr = [0;1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0;1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());


    // `Box::leak`，它可以消费掉 `Box` 并且强制目标值从内存中泄漏
    // 使用 `Box::leak` 就可以将一个运行期的值转为 `'static`。
    fn gen_static_str() -> &'static str{
        let mut s = String::new();
        s.push_str("hello, world");

        Box::leak(s.into_boxed_str())
    }
    let s = gen_static_str();
    println!("{}", s);

    











}
















// Rc


// Arc


// Cell


// RefCell