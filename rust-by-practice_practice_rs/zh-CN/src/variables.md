# 变量绑定与解构

### 绑定和可变性
1. 🌟 变量只有在初始化后才能被使用

```rust,editable

// 修复下面代码的错误并尽可能少的修改
fn main() {
    let x: i32; // 未初始化，但被使用
    let y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x); 
}
```

2. 🌟🌟 可以使用 `mut` 将变量标记为可变
```rust,editable

// 完形填空，让代码编译
fn main() {
    let __ = 1;
    __ += 2; 
    
    println!("x = {}", x); 
}
```

### 变量作用域
3. 🌟 作用域是一个变量在程序中能够保持合法的范围

```rust,editable

// 修复下面代码的错误并使用尽可能少的改变
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}, y 的值是 {}", x, y); 
}
```

4. 🌟🌟 

```rust,editable
// 修复错误
fn main() {
    println!("{}, world", x); 
}

fn define_x() {
    let x = "hello";
}
```

### 变量遮蔽( Shadowing )
5. 🌟🌟 若后面的变量声明的名称和之前的变量相同，则我们说：第一个变量被第二个同名变量遮蔽了( shadowing )

```rust,editable

// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x); // 输出 "42".
}
```

6. 🌟🌟 修改一行代码以通过编译
```rust,editable

fn main() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let x = x; 
    x += 3;


    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!"; 
}
```

### 未使用的变量
7. 使用以下方法来修复编译器输出的 warning :

- 🌟  一种方法
- 🌟🌟  两种方法

> 注意: 你可以使用两种方法解决，但是它们没有一种是移除 `let x = 1` 所在的代码行

```rust,editable

fn main() {
    let x = 1; 
}

// compiler warning: unused variable: `x`
```

### 变量解构
8. 🌟🌟 我们可以将 `let` 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量

> 提示: 可以使用变量遮蔽或可变性

```rust,editable

// 修复下面代码的错误并尽可能少的修改
fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}
```

### 解构式赋值
该功能于 Rust 1.59 版本引入：你可以在赋值语句的左式中使用元组、切片或结构体进行匹配赋值。

9. 🌟🌟

> Note: 解构式赋值只能在 Rust 1.59 或者更高版本中使用

```rust,editable

fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], __);
} 
```

> [答案](https://github.com/sunface/rust-by-practice/blob/master/solutions/variables.md) 在 solutions 下面
