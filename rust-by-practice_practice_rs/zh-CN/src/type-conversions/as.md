# 使用 as 进行类型转换
Rust 并没有为基本类型提供隐式的类型转换( coercion )，但是我们可以通过 `as` 来进行显式地转换。

1. 🌟
```rust,editable
// 修复错误，填空
// 不要移除任何代码
fn main() {
    let decimal = 97.123_f32;

    let integer: __ = decimal as u8;

    let c1: char = decimal as char;
    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8);

    println!("Success!")
}
```

2. 🌟🌟 默认情况下, 数值溢出会导致编译错误，但是我们可以通过添加一行全局注解的方式来避免编译错误(溢出还是会发生)
```rust,editable
fn main() {
    assert_eq!(u8::MAX, 255);
    // 如上所示，u8 类型允许的最大值是 255.
    // 因此以下代码会报溢出的错误： literal out of range for `u8`.
    // **请仔细查看相应的编译错误，从中寻找到解决的办法**
    // **不要修改 main 中的任何代码**
    let v = 1000 as u8;

    println!("Success!")
}
```

3. 🌟🌟  当将任何数值转换成无符号整型 `T` 时，如果当前的数值不在新类型的范围内，我们可以对当前数值进行加值或减值操作( 增加或减少 `T::MAX + 1` )，直到最新的值在新类型的范围内，假设我们要将 `300` 转成 `u8` 类型，由于`u8` 最大值是 255，因此 `300` 不在新类型的范围内并且大于新类型的最大值，因此我们需要减去 `T::MAX + 1`，也就是 `300` - `256` = `44`。

```rust,editable
fn main() {
    assert_eq!(1000 as u16, __);

    assert_eq!(1000 as u8, __);

    // 事实上，之前说的规则对于正整数而言，就是如下的取模
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, __);
    

    // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
    assert_eq!(300.1_f32 as u8, __);
    assert_eq!(-100.1_f32 as u8, __);
    

    // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
    // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
    // 总之，请小心使用
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
```

4. 🌟🌟🌟 裸指针可以和代表内存地址的整数互相转换
```rust,editable

// 填空
fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 __; 
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address __; // p2 指向 values 数组中的第二个元素
    unsafe {
        // 将第二个元素加 1
        __
    }
    
    assert_eq!(values[1], 3);

    println!("Success!")
}
```


5. 🌟🌟🌟 
```rust,editable
fn main() {
    let arr :[u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), __)
    }
}
```

> 你可以在[这里](https://github.com/sunface/rust-by-practice/blob/master/solutions/type-conversions/as.md)找到答案(在 solutions 路径下) 
