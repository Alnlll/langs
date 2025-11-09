# 2.1 变量
## 变量绑定
Rust中, `let a = "hello world"` 被定义为“变量绑定”。
如此命名的原因与Rust的核心原则--所有权相关，绑定就是把一个对象绑定给一个变量，这个变量成为了它的主人（同时也意味着原有的主人失去了所有权）。

## 变量可变性
Rust默认情况下，变量是不可变的，可以通过`mut`关键字指定为可变变量。
```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// ##################################
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

error: aborting due to previous error
```

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

## 变量解构
```rust
fn main() {
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}
```
### 结构式赋值
在rust 1.59之后，可以在赋值语句的左式中使用元祖，切片和结构体模式了。
```rust
struct Struct {
    e: i32
}

fn main() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}
```

## 变量和常量
- 常量不允许使用`mut`, 常量不仅默认不可变，而且始终不可变，常量在编译完成后就已经确认了他的值。
- 常量使用`const`来声明，并且必须标注类型

```rust
const MAX_POINTS: u32 = 100_000;
```
常量可以在任意作用域中声明，包括全局作用域，在声明的作用域内，常量在程序运行的整个过程中都有效。

## 变量遮蔽（shadowing）
Rust允许声明相同的变量，在后面声明的变量会遮蔽（shadowing）前面声明的变量。
```rust
fn main() {
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
```

# 2.2 基本类型
Rust每个变量都有其确切的数据类型，通常分为两类：基本类型和复合类型：
- 数值类型：有符号整数 `(i8, i16, i32, i64, isize)`、 无符号整数 `(u8, u16, u32, u64, usize)` 、浮点数 `(f32, f64)`、以及有理数、复数
- 字符串：字符串字面量和字符串切片 `&str`
- 布尔类型：`true` 和 `false`
- 字符类型：表示单个 Unicode 字符，存储为 4 个字节
- 单元类型：即 () ，其唯一的值也是 ()

## 类型推倒与标注
与其他动态语言不同，Rust是静态类型语言，编译器必须在编译期确定所有变量的类型。但是这不意味着我们必须为每一个变量标注类型，编译器也可以进行类型推导。
但是在某些情况下，编译器不能推导出确切类型的时候，就需要明确的标准类型。

```rust
let guess = "42".parse().expect("Not a number!");

$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type
```

## 2.2.1 数值类型
### 整数类型
长度 | 有符号类型 |无符号类型
-- | -- | --
8 位 | i8 | u8
16 位 | i16 | u16
32 位 | i32 | u32
64 位 | i64 | u64
128 位 | i128 | u128
视架构而定 | isize | usize

每个有符号类型规定的数字范围是 $-(2^{n - 1})$ ~ $ 2^{n - 1} - 1$，其中 n 是该定义形式的位长度。因此 `i8`可存储数字范围是 -128 ~ 127。无符号类型可以存储的数字范围是 0 ~ $2^n - 1$，所以 u8 能够存储的数字为 0 ~ 255。

数字字面量 | 示例
-- | --
十进制 | 98_222
十六进制 | 0xff
八进制 | 0o77
二进制 | 0b1111_0000
字节 (仅限于 u8) | b'A'

### 整形溢出
在debug模式下，Rust会检查整形溢出并panic。
但是在release模式下，Rust不会检测溢出，而是按照补码循环溢出（two's complement wrapper）的规则处理。
大于该类型最大值的数值会被补码转化成该类型能够支持的最小值，e.x., `u8`下256 -》 0， 257 -》 1.
如果想要显式处理溢出，则可以使用如下的方法：
- 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 `wrapping_add`
- 如果使用 checked_* 方法时发生溢出，则返回 None 值
- 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
- 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值，例如:
```Rust
assert_eq!(100u8.saturating_add(1), 101);
assert_eq!(u8::MAX.saturating_add(127), u8::MAX);
```

## 浮点类型
在 Rust 中浮点类型数字也有两种基本类型： `f32` 和 `f64`.
```Rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```
浮点数由于底层格式的特殊性，导致了如果在使用浮点数时不够谨慎，就可能造成危险，有两个原因：
- 浮点数往往是你想要数字的近似表达
- 浮点数在某些特性上是反直觉的 

Rust 的 HashMap 数据结构，是一个 KV 类型的 Hash Map 实现，它对于 K 没有特定类型的限制，但是要求能用作 K 的类型必须实现了 std::cmp::Eq 特征，因此这意味着你无法使用浮点数作为 HashMap 的 Key.

为了避免上面说的两个陷阱，你需要遵守以下准则：
- 避免在浮点数上测试相等性
- 当结果在数学上可能存在未定义时，需要格外的小心

```rust
fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2);
}

abc (f32)
   0.1 + 0.2: 3e99999a
         0.3: 3e99999a

xyz (f64)
   0.1 + 0.2: 3fd3333333333334
         0.3: 3fd3333333333333

thread 'main' panicked at 'assertion failed: xyz.0 + xyz.1 == xyz.2',
➥ch2-add-floats.rs.rs:14:5
note: run with `RUST_BACKTRACE=1` environment variable to display
➥a backtrace
```

### NaN
对于数学上未定义的结果，例如对负数取平方根 -42.1.sqrt() ，会产生一个特殊的结果：Rust 的浮点数类型使用 NaN (not a number) 来处理这些情况。

所有跟 NaN 交互的操作，都会返回一个 NaN，而且 NaN 不能用来比较.

```rust
fn main() {
  let x = (-42.0_f32).sqrt();
  assert_eq!(x, x);
}

 Compiling playground v0.0.1 (/playground)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/playground`

thread 'main' (12) panicked at src/main.rs:3:3:
assertion `left == right` failed
  left: NaN
 right: NaN
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## 数字运算
Rust 支持所有数字类型的基本数学运算：加法、减法、乘法、除法和取模运算。
```rust
fn main() {
  // 编译器会进行自动推导，给予twenty i32的类型
  let twenty = 20;
  // 类型标注
  let twenty_one: i32 = 21;
  // 通过类型后缀的方式进行类型标注：22是i32类型
  let twenty_two = 22i32;

  // 只有同样类型，才能运算
  let addition = twenty + twenty_one + twenty_two;
  println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

  // 对于较长的数字，可以用_进行分割，提升可读性
  let one_million: i64 = 1_000_000;
  println!("{}", one_million.pow(2));

  // 定义一个f32数组，其中42.0会自动被推导为f32类型
  let forty_twos = [
    42.0,
    42f32,
    42.0_f32,
  ];

  // 打印数组中第一个值，并控制小数位为2位
  println!("{:.2}", forty_twos[0]);
}
```

## 位运算

运算符 | 说明
-- | --
& | 相同位置均为1时则为1,否则为0
｜ | 相同位置只要有1时则为1，否则为0
^ | 相同位置不相同则为1，相同则为0
! | 把位中的0和1相互取反，即0置为1，1置为0
<< 左移 | 所有位向左移动指定位数，右位补0
">> 右移" | 所有位向右移动指定位数，带符号移动（正数补0，负数补1）

```rust
fn main() {
    // 无符号8位整数，二进制为00000010
    let a: u8 = 2; // 也可以写 let a: u8 = 0b_0000_0010;

    // 二进制为00000011
    let b: u8 = 3;

    // {:08b}：左高右低输出二进制01，不足8位则高位补0
    println!("a value is        {:08b}", a);

    println!("b value is        {:08b}", b);

    println!("(a & b) value is  {:08b}", a & b);

    println!("(a | b) value is  {:08b}", a | b);

    println!("(a ^ b) value is  {:08b}", a ^ b);

    println!("(!b) value is     {:08b}", !b);

    println!("(a << b) value is {:08b}", a << b);

    println!("(a >> b) value is {:08b}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {:08b}", a);
}
```

```rust
fn main() {
   let a: u8 = 255;
   let b = a>>7; // ok
   let b = a<<7; // ok
   let b = a>>8; // overflow
   let b = a<<8; // overflow
}
```

## 序列(Range)
Rust 提供了一个非常简洁的方式，用来生成连续的数值/
```rust
for i in 1..=5 {
    println!("{}",i);
}
```

序列只允许用于数字或字符类型，原因是：它们可以连续，同时编译器在编译期可以检查该序列是否为空，字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型

```rust
for i in 'a'..='z' {
    println!("{}",i);
}
```

## 使用 As 完成类型转换
Rust 中可以使用 As 来完成一个类型到另一个类型的转换，其最常用于将原始类型转换为其他原始类型，但是它也可以完成诸如将指针转换为地址、地址转换为指针以及将指针转换为其他指针等功能。


## 有理数和复数
Rust 的标准库相比其它语言，准入门槛较高，因此有理数和复数并未包含在标准库中：
- 有理数和复数
- 任意大小的整数和任意精度的浮点数
- 固定精度的十进制小数，常用于货币相关的场景

好在社区已经开发出高质量的 Rust 数值库：num。

按照以下步骤来引入 num 库：
- 创建新工程 cargo new complex-num && cd complex-num
- 在 Cargo.toml 中的 [dependencies] 下添加一行 num = "0.4.0"
- 将 src/main.rs 文件中的 main 函数替换为下面的代码
- 运行 cargo run

```rust
use num::complex::Complex;

 fn main() {
   let a = Complex { re: 2.1, im: -1.2 };
   let b = Complex::new(11.1, 22.2);
   let result = a + b;

   println!("{} + {}i", result.re, result.im)
 }
```