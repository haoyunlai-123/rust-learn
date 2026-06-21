# Rust 数据类型详解

> 对应项目：`projects/03-data-types`  
> 掌握目标：熟悉 Rust 所有内置类型、字面量写法、类型转换规则

---

## 一、Rust 是静态类型语言

每一行代码中，所有变量的类型在**编译时**就已确定。但 Rust 的类型推断非常强大，不用每个变量都手写类型。

```rust
let x = 42;      // 编译器推断 x: i32
let y = 3.14;    // 编译器推断 y: f64
let z = "hello"; // 编译器推断 z: &str
```

---

## 二、整数类型（12 种）

Rust 的整数类型极为丰富，分三组共 12 种：

### 类型一览

```
有符号 (signed, 补码)          无符号 (unsigned)         指针宽度
─────────────────────────    ──────────────────────    ────────────
i8    -128 ~ 127              u8     0 ~ 255
i16   -32768 ~ 32767          u16    0 ~ 65535
i32   -21亿 ~ 21亿            u32    0 ~ 42亿           isize
i64   -9.2e18 ~ 9.2e18        u64    0 ~ 1.8e19
i128  -(2^127) ~ 2^127-1      u128   0 ~ 2^128-1       usize
```

### 默认类型

```rust
let x = 42;     // i32 —— 整数默认类型
let y = 3.14;   // f64 —— 浮点默认类型
```

`i32` 是 Rust 整数默认类型，性能和空间的平衡选择。编译器偏好它，除非上下文有明确的其他类型要求。

### isize / usize —— 特殊的两兄弟

```rust
let idx: usize = 5;           // 数组索引只能用 usize
let arr = [1, 2, 3];
let val = arr[idx];           // 必须是 usize
```

这两个类型的大小取决于**目标平台的指针宽度**：
- 64 位系统上，`isize` = `i64`，`usize` = `u64`  
- 32 位系统上，`isize` = `i32`，`usize` = `u32`

### 整数溢出行为

| 编译模式 | 溢出行为 |
|----------|----------|
| Debug (dev) | **panic!** 程序崩溃 |
| Release | **回绕 (wrapping)** — 200 + 100 = 44 |

如果你想要**显式控制**溢出行为：

```rust
let a: u8 = 200;
a.wrapping_add(100)    // → 44  (回绕)
a.saturating_add(100)  // → 255 (饱和)
a.checked_add(100)     // → None (检测到溢出返回 None)
a.overflowing_add(100) // → (44, true) (带溢出标志)
```

### 整数字面量的 5 种写法

```rust
98_222         // 十进制 (下划线作分隔符，纯为了可读性)
0xff           // 十六进制 → 255
0o77           // 八进制   → 63
0b1111_0000    // 二进制   → 240
b'A'           // 字节字面量 → 65u8
```

类型后缀：
```rust
42_i32       // 明确指定 i32
99u8         // u8（后缀写法）
0xffu16      // 组合使用
```

---

## 三、浮点类型（2 种）

| 类型 | 精度 | 尺寸 | 默认 |
|------|------|------|------|
| `f32` | ~7 位有效数字 | 4 字节 | |
| `f64` | ~15 位有效数字 | 8 字节 | ✅ |

### 特殊值

```rust
f64::NAN           // Not a Number：0.0/0.0 的结果
f64::INFINITY      // 正无穷：1.0/0.0
f64::NEG_INFINITY  // 负无穷：-1.0/0.0
f64::MIN           // 最小正数
f64::MAX           // 最大有限值
```

### 重要的坑

```rust
// NaN 不等于任何东西，包括它自己
f64::NAN != f64::NAN  // → true (唯一违反"自己==自己"的值)
f64::NAN.is_nan()     // → true  (正确判断 NaN 的方法)

// 浮点运算不是精确的
0.1 + 0.2 == 0.3     // → false! (0.1+0.2 = 0.30000000000000004)
```

---

## 四、布尔类型

```rust
let t: bool = true;
let f = false;
```

| 事实 | 说明 |
|------|------|
| 大小 | **1 字节**（不是 1 bit） |
| 取值 | 只有 `true` 和 `false` |
| 运算 | `&&` `||` `!` `&` `|` `^` |

**注意**：Rust 不会自动把数字/字符串转成 bool。

```rust
let x = true;
// if x == 1 { }   // ❌ 类型错误：bool vs i32
if x == true { }   // ✅ 可以（但冗余，直接 if x 即可）
```

---

## 五、字符类型 `char`

`char` 是 Rust 最大的"坑人"点之一——**它不是 C 的 char**：

| | Rust `char` | C `char` |
|---|---|---|
| 大小 | **4 字节** | 1 字节 |
| 范围 | Unicode 标量值 (U+0000 ~ U+10FFFF) | 0~255 |
| 编码 | Unicode | ASCII (通常是) |

```rust
let a = 'A';       // ASCII，字符码 65
let zh = '中';     // CJK 汉字，3 字节 UTF-8，但 char 仍是 4 字节
let emoji = '🦀';  // Emoji，4 字节 UTF-8，char 也是 4 字节
```

### char 常用方法

```rust
'a'.is_alphabetic()   // → true
'1'.is_digit(10)      // → true
'a'.is_uppercase()    // → false
'a'.to_uppercase()    // → "A" (返回迭代器，因为有些字符大写是多字符)
```
### char 与 UTF-8 的关系

一个 `char` = 一个 Unicode **标量值 (scalar value)**，不是 code point 的全部——代理对 (surrogate, U+D800~U+DFFF) 不是合法的 char。

一个 `String` = UTF-8 字节序列。所以：
- "A" → `String` 占 1 字节，但 `char` 占 4 字节
- "中" → `String` 占 3 字节，`char` 占 4 字节
- "🦀" → `String` 占 4 字节，`char` 占 4 字节

---

## 六、元组 (Tuple)

```rust
let tup: (i32, f64, bool) = (42, 3.14, true);

// 访问方式：点索引
let x = tup.0;   // 42
let y = tup.1;   // 3.14
let z = tup.2;   // true

// 解构
let (a, b, c) = tup;
```

### 单元类型 `()`

```rust
let unit = ();           // 零元素元组，只有这一个值
fn no_return() { }       // 隐式返回 ()
```

`()` 就是 "unit type"（单元类型），类似于 C/Java 的 `void`，但它是一个实际的值。`fn main()` 的返回类型就是 `()`。你可以显式 `return ()`，虽然通常不需要。

---

## 七、数组 (Array)

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];   // 类型 + 长度
let zeros = [0; 100];                     // 100 个 0
let mixed = [1, 2, 3];                    // 类型推断
```

### 数组的关键事实

| 事实 | 说明 |
|------|------|
| 长度是类型的一部分 | `[i32; 3]` 和 `[i32; 5]` 是不同的类型 |
| 分配在栈上 | 和 C 数组一样，没有堆开销 |
| 索引用 usize | `arr[0]` 的 `0` 实际上是 `0usize` |
| 越界检查 | 运行时 panic，编译器不会警告 |

### 常用方法

```rust
let arr = [10, 20, 30, 40, 50];

arr.len()                // → 5
arr.first()              // → Some(&10)
arr.last()               // → Some(&50)
arr.contains(&30)        // → true
arr.iter().sum::<i32>()  // → 150
&arr[1..3]               // → [20, 30] (切片)
```

### 数组 vs 切片

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];   // 数组——有所有权的具体类型
let slice: &[i32] = &arr[1..3];         // 切片——引用视图，不拥有数据
```

数组和切片的区别是 Rust 学习的重要节点，后面"切片"章节会深入展开。

---

## 八、类型转换

### `as` —— 显式强制转换

```rust
let a = 300u32 as u8;     // → 44 (截断！)
let b = 3.7f64 as i32;    // → 3 (截断，不是四舍五入)
let c = 10i32 as f64;     // → 10.0 (安全)
let d = 'A' as u32;       // → 65 (char 可以转成整数)
```

**规则**：`as` 是"尽力转换"，不保证数据完整。

### `From` / `Into` —— 安全无损转换

```rust
let a: i32 = i32::from(42i16);   // i16 → i32 永远安全
let b: u16 = 100u8.into();       // .into() 简洁写法，目标类型需可推断
```

只有**保证不丢数据**的类型转换才会实现 `From`。比如 `i16 → i32` 有 `From`，但 `i32 → i16` 没有。

### `TryFrom` / `TryInto` —— 可能失败的转换

```rust
use std::convert::TryFrom;

let ok = u32::try_from(100i32);   // Ok(100)
let err = u32::try_from(-5i32);   // Err(...) 负数不能转无符号
```

返回 `Result<T, E>`，让你显式处理失败情况。

### 转换方式选择指南

```
目标类型能无损包含源类型？ → From / Into
可能失败但你想知道？      → TryFrom / TryInto
不在乎截断 / 快速转换？   → as
数字 ↔ 字符串？           → parse() / format!() (不是类型转换)
```

---

## 九、类型别名

```rust
type Score = u32;           // 语义别名
type Point = (f64, f64);    // 复合类型别名

let s: Score = 100;         // Score 就是 u32
let p: Point = (3.0, 4.0);  // Point 就是 (f64, f64)
```

类型别名**不创建新类型**，只是给原名起个新名字。`Score` 和 `u32` 可以互相赋值，不需要任何转换。

---

## 十、大小速查表

```rust
std::mem::size_of::<i32>()   // 4
std::mem::size_of::<f64>()   // 8
std::mem::size_of::<bool>()  // 1
std::mem::size_of::<char>()  // 4
std::mem::size_of::<()>()    // 0
std::mem::size_of::<&str>()  // 16 (指针+长度，64位系统)
// 数组: size_of::<T>() × N
```

---

## 十一、一个常见误解

Rust 没有"基本类型 (primitive type)"这个说法的准确对应。从语言角度看，i32、char、() 都是类型——有些实现了 Copy，有些没有；有些可以放在栈上，有些在堆上。不要把其他语言的"primitive vs reference"二分法直接套过来。
