# Rust 学习笔记

> [Rust 官方网站](https://www.rust-lang.org/) </br>
> [Rust 开发环境搭建](https://www.rust-lang.org/tools/install) </br>
> [Rust 官方文档](https://doc.rust-lang.org/book/) | 
> [Rust 文档中文译本](https://kaisery.github.io/trpl-zh-cn/) </br>
> [Rust 在线练习](https://play.rust-lang.org/) </br>
> [Rust 语言参考](https://doc.rust-lang.org/reference/) </br>
> [通过例子学习 Rust(Rust By Example)](https://doc.rust-lang.org/rust-by-example/)</br>
> [Rust 小练习(Rustlings)](https://github.com/rust-lang/rustlings) </br>
> [Rust 语言标准库](https://doc.rust-lang.org/std/) </br>
> [Rust 中文 Wiki](https://rustwiki.org/) </br>
> Rust 在[哔哩哔哩](https://www.bilibili.com/)上也有视频教程, 
> 推荐[此视频](https://www.bilibili.com/video/BV1m1sreSEoh/)或[此视频](https://www.bilibili.com/video/BV15y421h7j7/)

## 1 变量与常见数据类型
1. 在 Rust 中, 使用 `let` 关键字声明变量.
2. Rust 支持类型推导, 但也可以显式指定变量类型: `let x: i32 = 5; //显式指定 x 的类型为 i32.`
3. 变量命名使用蛇形命名法, 而枚举与结构提使用帕斯卡命名法; 若变量没有使用则前置下划线以消除警告.
4. 强制类型转换(Casting a value to a different type):
   `let a = 3.1; let b = a as i32;`
5. 打印变量:
   ```rust
   println!("val: {}", x);
   println!("val: {x}");
   ```
### 1.1 Rust 的变量默认不可变
不可变性是 Rust 实现其可靠性与安全性目标的关键.
#### 1.1.1 使用 `mut` 关键字使变量变得可变
```rust
fn main() { 
    let mut y = 5; //可变变量   
    y = 20; //合法修改
}
```
#### 1.1.2 遮蔽(Shadow)
Rust 允许隐藏一个变量, 这意味着可以声明一个与现有变量同名的新变量,
并隐藏前一个变量.
- 可以改变值
- 可以改变类型
- 可以改变可变性
```rust
fn variable() {
    //不可变与命名
    let count_a = 15; //自动推导变量类型为 i32
    let count_b: i16 = 15; //显式指定变量类型为 i16
    /*count_a = 2;*/ //尝试改变不可变变量的值,会报错
    let mut count_c: i32 = 2147483647; //声明可变
    count_c = -2147483648;
    //遮蔽(Shadow)
    let x = 1;
    {
        let x: bool = false; //覆盖x(数值）
        {
            let x = "Shadowing"; //遮蔽x(布尔值)
            println!("{}", x); //打印以验证
        } //x(字符串)作用域结束(释放)
        println!("{}", x); //打印以验证
    } //x(布尔值)作用域结束(释放)
    println!("{}", x); //打印以验证
    let mut y = 4;
}
```

### 1.2 常量(const)与静态变量(staic)
#### 1.2.1 常量(const)
- 常量使用 `const` 关键字声明, 必须指定类型和值.
- 常量的值直接嵌到底层代码中, 而不是简单的字符替换.
- 常量名必须全部大写, 使用下划线分隔单词.
- 常量的声明只在作用域内有效.
#### 1.2.2 静态变量(static)
- 与常量(const)不同, 静态变量实在运行时分配内存的.
- 并非不可变, 可以使用 `unsafe` 关键字修改静态变量(前提是使用 `mut` 声明其可变).
- 静态变量的生命周期整个程序的运行时间.
#### 示例
```rust
static DAY_IN_SOLAR_YEAR_APPROXIMATE: f64 = 365.2422; //静态变量
static  mut DAY_IN_YAER: u16 = 365; //可变静态变量

fn c2_const_and_static() {
    const SECONDS_IN_WEEK: u32 = 60 * 60 * 24; //使用`const`关键字声明常量
    /* SECONDS_IN_WEEK = 5; */ //尝试改变常量的值,会报错
    const SECONDS_IN_DAY: u32 = 86_400; //常量(另一种写法)
    {
        const A: u8 = 255;
    }
    /*println!("{A}");*/ //尝试打印不在作用域内的常量A,会报错
    const SECONDS_IN_SOLAR_YEAR: u64 = 60 * 60 * 24 * 365 + 60 * 60 * 5 + 60 * 48 + 46;
  
    /*unsafe {
        DAY_IN_YAER = 366;
        println!("{DAY_IN_YAER}"); //打印可变静态变量
    }*/ //使用`unsafe`修改可变静态变量
  
    println!("One week has {SECONDS_IN_WEEK} seconds |\
    One day has {SECONDS_IN_DAY} seconds |\
    One solar year has {SECONDS_IN_SOLAR_YEAR} seconds");
  
    println!("One solar year has {DAY_IN_SOLAR_YEAR_APPROXIMATE} days");
}
```
### 1.3 数据类型
#### 1.3.1 整数类型(整型)
1. 整数类型有符号与无符号两种, 分别使用 `i` 和 `u` 前缀.
    - 有符号整数: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
    - 无符号整数: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
2. 有符号整数不支持负数, 无符号整数支持负数.
    - 对于有符号整数 `i<x>`, 其支持范围为$[-2^{x-1}, 2^{x-1}-1]$.
    - 对于无符号整数 `u<x>`, 其支持范围为$[0, 2^{x}-1]$.
3. 整数类型默认为 `i32`.
4. 整数类型的大小与平台无关, 但 `isize` 与 `usize` 的大小与平台相关.

__表1.1 不同整数类型支持的范围__

|   类型   | 范围                      |        说明         |
|:------:|:------------------------|:-----------------:|
|  `i8`  | $[-2^7, 2^7-1]$         |  8 位有符号整数, 支持负数   |
| `i16`  | $[-2^{15}, 2^{15}-1]$   |  16 位有符号整数, 支持负数  |
| `i32`  | $[-2^{31}, 2^{31}-1]$   |  32 位有符号整数, 支持负数  |
| `i64`  | $[-2^{63}, 2^{63}-1]$   |  64 位有符号整数, 支持负数  |
| `i128` | $[-2^{127}, 2^{127}-1]$ | 128 位有符号整数, 支持负数  |
|  `u8`  | $[0, 2^8-1]$            |  8 位无符号整数, 不支持负数  |
| `u16`  | $[0, 2^{16}-1]$         | 16 位无符号整数, 不支持负数  |
| `u32`  | $[0, 2^{32}-1]$         | 32 位无符号整数, 不支持负数  |
| `u64`  | $[0, 2^{64}-1]$         | 64 位无符号整数, 不支持负数  |
| `u128` | $[0, 2^{128}-1]$        | 128 位无符号整数, 不支持负数 |

- 可以使用任何一种形式编写数字字面值，同时也允许使用`_`做为分隔符以方便读数，例如`1_000`，
  它的值与你指定的 1000 相同。

__表1.2 整数类型的字面值__

| 字面值  |      示例       | 实际数值 |
|:----:|:-------------:|:----:|
| 十进制  |    `1_000`    | 1000 |
| 十六进制 |    `0xE3`     | 227  |
| 八进制  |    `0o77`     |  63  |
| 二进制  | `0b1111_0000` | 240  |

> _整数溢出_ </br>
> 整数溢出是指在计算中，结果超出了整数类型的表示范围. 
> 例如: 在使用 `u8` 类型时, 计算 `255 + 1` 会导致溢出.</br>
> 在 Rust 中, 整数溢出会导致程序 _panic_, 这是一种运行时错误.
> 使用 `--release` flag 在 release 模式中构建时，
> Rust 不会检测会导致 panic 的整型溢出.
> 相反发生整型溢出时，Rust 会进行一种被称为二进制补码 wrapping 的操作。
> 简而言之，比此类型能容纳最大值还大的值会回绕到最小值, 
> 值 256 变成 0，值 257 变成 1，依此类推. 程序不会 panic
> 不过变量可能也不会是你所期望的值.依赖整型溢出 wrapping 的行为被认为是一种错误。
> 为了显式地处理溢出的可能性，可以使用这几类标准库提供的原始数字类型方法：
> - 所有模式下都可以使用 wrapping_* 方法进行 wrapping，如 wrapping_add
> - 如果 checked_* 方法出现溢出，则返回 None值
> - 用 overflowing_* 方法返回值和一个布尔值，表示是否出现溢出
> - 用 saturating_* 方法在值的最小值或最大值处进行饱和处理

#### 1.3.2 浮点数类型(浮点型)
Rust 支持两种浮点数类型: `f32` 与 `f64`, 默认为 `f64`, `f64` 的精度更高.

#### 1.3.3 布尔类型(bool)
布尔类型只有两个值: `true` 与 `false`, 其字面值为 `true` 与 `false`. 通常在`if` 语句中使用.

#### 1.3.4 字符类型(char)

Rust 的字符类型是 `char`, 它表示单个 Unicode 字符, 而不是单个字节. `char` 类型的大小为 4 字节,

---

```rust
//示例: 数据类型
fn data_type() {
   let a = 5; //整数, 默认i32
   let b: i32 = -5; //整数, 显式指定类型为i32
   let c: u32 = 5; //整数, 显式指定类型为u32, 仅支持自然数
   let d = 1.14; //浮点数, 默认f64
   let e: f32 = 3.14; //浮点数, 显式指定类型为f32
   let t = true; //布尔值
   let f: bool = false; //布尔值
   let character = '😅'; //字符类型, 4字节
}
```

#### 1.3.5 数值运算

Rust 中的所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取余. 整数除法会
向零舍入到最接近的整数.
```rust
fn calculate() {
   let a: i32 = 5;
   let b: u64 = 455432378;
   let c: f64 = 3.14;
   let d: f32 = 2.5;
   let e: i32 = 10;
   let f: i32 = 17;
   let add = a + b; //加法
   let sub = b - d; //减法
   let mul = d * c; //乘法
   let div = e / a; //除法
   let rem = e % a; //取余(能整除, 结果为0)
   let rems = f % a; //取余(不能整除）
}
```