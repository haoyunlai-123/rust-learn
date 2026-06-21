// ============================================================
// Rust 数据类型 完整示例
// 涵盖：整数 / 浮点 / 布尔 / 字符 / 元组 / 数组 / 类型转换
// ============================================================

fn main() {
    println!("{}", "=".repeat(58));
    println!("🦀  Rust 数据类型 — 完整演示");
    println!("{}\n", "=".repeat(58));

    // =====================================================
    //  1. 整数类型 — Rust 有 12 种整数
    // =====================================================
    println!("┌──────────────────────────────────────────────────┐");
    println!("│ 1. 整数类型：12 种                                 │");
    println!("└──────────────────────────────────────────────────┘\n");

    // 有符号整数 (i = signed, 补码表示)
    let i8_val: i8 = -128;         // 范围: -128 ~ 127
    let i16_val: i16 = -32768;     // 范围: -32768 ~ 32767
    let i32_val: i32 = -2147483648;// 范围: ≈ -21亿 ~ 21亿
    let i64_val: i64 = 9223372036854775807; // 范围: ≈ ±9.2e18
    let _i128_val: i128 = 170141183460469231731687303715884105727; // 巨大
    let isize_val: isize = 42;     // 指针宽度(32位系统=32bit, 64位=64bit)

    // 无符号整数 (u = unsigned)
    let u8_val: u8 = 255;          // 范围: 0 ~ 255
    let u16_val: u16 = 65535;      // 范围: 0 ~ 65535
    let u32_val: u32 = 4294967295; // 范围: 0 ~ 42亿
    let _u64_val: u64 = 18446744073709551615; // 范围: 0 ~ 1.8e19
    let _u128_val: u128 = 340282366920938463463374607431768211455;
    let usize_val: usize = 42;     // 指针宽度，常用于索引/长度

    println!("  ┌──────────┬──────────────────────────┬──────────────┐");
    println!("  │  类型     │  范围                     │   示例       │");
    println!("  ├──────────┼──────────────────────────┼──────────────┤");
    println!("  │ i8       │ -128 ~ 127               │ {:>12} │", i8_val);
    println!("  │ i16      │ -32768 ~ 32767           │ {:>12} │", i16_val);
    println!("  │ i32      │ ±21亿                    │ {:>12} │", i32_val);
    println!("  │ i64      │ ±9.2e18                  │ {:>12} │", i64_val);
    println!("  │ i128     │ 巨大                     │ (省略)       │");
    println!("  │ isize    │ 指针宽度(64位系统=64bit) │ {:>12} │", isize_val);
    println!("  ├──────────┼──────────────────────────┼──────────────┤");
    println!("  │ u8       │ 0 ~ 255                  │ {:>12} │", u8_val);
    println!("  │ u16      │ 0 ~ 65535                │ {:>12} │", u16_val);
    println!("  │ u32      │ 0 ~ 42亿                 │ {:>12} │", u32_val);
    println!("  │ u64      │ 0 ~ 1.8e19               │ (省略)       │");
    println!("  │ u128     │ 巨大                     │ (省略)       │");
    println!("  │ usize    │ 指针宽度                  │ {:>12} │", usize_val);
    println!("  └──────────┴──────────────────────────┴──────────────┘\n");

    // 字面量中的类型后缀
    let _one: i32 = 1i32;      // 后缀指定类型
    let _byte: u8 = 42u8;
    let _big: i64 = 999i64;
    println!("  字面量后缀: 1i32 / 42u8 / 999i64\n");

    // =====================================================
    //  1b. 整数字面量格式
    // =====================================================
    println!("┌──────────────────────────────────────────────────┐");
    println!("│ 1b. 整数字面量格式                                │");
    println!("└──────────────────────────────────────────────────┘\n");

    let decimal = 98_222;           // 十进制，_ 可作分隔符
    let hex = 0xff;                 // 十六进制 (255)
    let octal = 0o77;               // 八进制 (63)
    let binary = 0b1111_0000;       // 二进制 (240)
    let byte_literal = b'A';        // u8 字节字面量 (65)

    println!("  十进制   98_222     = {}", decimal);
    println!("  十六进制 0xff       = {}", hex);
    println!("  八进制   0o77       = {}", octal);
    println!("  二进制   0b1111_0000 = {}", binary);
    println!("  字节     b'A'       = {}", byte_literal);
    println!();

    // =====================================================
    //  1c. 整数溢出行为
    // =====================================================
    println!("┌──────────────────────────────────────────────────┐");
    println!("│ 1c. 整数溢出                                      │");
    println!("└──────────────────────────────────────────────────┘\n");

    // dev 模式下：panic!
    // release 模式下：回绕（wrapping）
    let small: u8 = 200;
    // let overflow = small + 100;  // dev模式 panic! ; release模式 wrapping

    // 显式控制溢出行为：
    println!("  u8: 200 + 100");
    println!("    wrapping_add     = {} (回绕: 300 % 256)", small.wrapping_add(100));
    println!("    saturating_add   = {} (饱和: 不会超过255)", small.saturating_add(100));
    println!("    checked_add      = {:?} (Checked: None表示溢出)", small.checked_add(100));
    println!("    overflowing_add  = {:?} (带溢出标志)", small.overflowing_add(100));
    println!();

    // =====================================================
    //  2. 浮点类型
    // =====================================================
    println!("┌──────────────────────────────────────────────────┐");
    println!("│ 2. 浮点类型：f32 与 f64                           │");
    println!("└──────────────────────────────────────────────────┘\n");

    let f32_val: f32 = 3.141592653589793;           // 单精度 ~7位有效数字
    let f64_val = 3.141592653589793f64;             // 双精度 ~15位有效数字 (默认)
    println!("  f32: {}", f32_val);
    println!("  f64: {}  ← 浮点默认类型", f64_val);
    println!();

    // 浮点特殊值
    println!("  特殊值:");
    println!("    f64::NAN           = {} (Not a Number)", f64::NAN);
    println!("    f64::INFINITY      = {} (正无穷)", f64::INFINITY);
    println!("    f64::NEG_INFINITY  = {} (负无穷)", f64::NEG_INFINITY);
    println!("    f64::MIN           = {} (最小正数)", f64::MIN);
    println!("    f64::MAX           = {:.5e} (最大有限值)", f64::MAX);
    println!();

    // NaN 特殊性质: NaN != NaN
    let nan = f64::NAN;
    println!("  ⚠️  NaN != NaN  →  {} (这种情况独一无二!)", nan != nan);

    // 浮点精度陷阱
    println!("  ⚠️ 0.1 + 0.2 == 0.3 → {} (浮点精度问题)", 0.1 + 0.2 == 0.3);
    println!();

    // =====================================================
    //  3. 布尔类型
    // =====================================================
    println!("┌──────────────────────────────────────────────────┐");
    println!("│ 3. 布尔类型：bool                                 │");
    println!("└──────────────────────────────────────────────────┘\n");

    let t: bool = true;
    let f = false;                  // 类型推断
    println!("  bool 只有两个值: true / false");
    println!("  t = {}, f = {}", t, f);
    println!();

    // 布尔运算
    println!("  布尔运算:");
    println!("    && (逻辑与):  true && false = {}", true && false);
    println!("    || (逻辑或):  true || false = {}", true || false);
    println!("    !  (逻辑非):  !true         = {}", !true);
    println!("    &= (位与赋值*) (少见)");
    println!();

    // bool 占 1 字节
    println!("  bool 内存大小: {} byte", std::mem::size_of::<bool>());
    println!();

    // =====================================================
    //  4. 字符类型
    // =====================================================
    println!("┌──────────────────────────────────────────────────┐");
    println!("│ 4. 字符类型：char                                 │");
    println!("└──────────────────────────────────────────────────┘\n");

    let a: char = 'A';              // ASCII
    let zh: char = '中';            // 中文
    let emoji: char = '🦀';         // Emoji
    let heart: char = '❤';          // 符号

    println!("  'A'   = {} (ASCII, U+0041)", a as u32);
    println!("  '中'  = {} (CJK, U+4E2D)", zh as u32);
    println!("  '🦀'  = {} (Emoji, U+1F980)", emoji as u32);
    println!("  '❤'   = {} (Symbol, U+2764)", heart as u32);
    println!();

    // char 关键事实
    println!("  🔑 char 关键事实:");
    println!("    - 占 4 字节 (不是1字节！)");
    println!("    - 代表一个 Unicode 标量值 (scalar value)");
    println!("    - 范围: U+0000 ~ U+D7FF 和 U+E000 ~ U+10FFFF");
    println!("    - char 可以用 as 转成数字，但数字不能直接转 char");
    println!();

    // char 方法
    let c: char = 'a';
    println!("  'a'.is_alphabetic()   = {}", c.is_alphabetic());
    println!("  'a'.is_ascii()        = {}", c.is_ascii());
    println!("  'a'.is_digit(10)      = {}", c.is_digit(10));
    println!("  'a'.is_uppercase()    = {}", c.is_uppercase());
    println!("  'a'.to_uppercase()    = {:?}", c.to_uppercase().collect::<String>());
    println!();

    // char vs byte vs &str
    let byte: u8 = b'A';            // u8 字节字面量
    let _ch: char = 'A';             // char，4 字节
    let s: &str = "A";              // &str，1字节 (因为ASCII)
    println!("  b'A'  大小: {} byte (u8)", std::mem::size_of_val(&byte));
    println!("  'A'   大小: {} bytes (char)", std::mem::size_of::<char>());
    println!("  \"A\"   大小: {} byte (&str)", s.len());
    println!();

    // =====================================================
    //  5. 元组类型
    // =====================================================
    println!("┌──────────────────────────────────────────────────┐");
    println!("│ 5. 元组 (Tuple)：固定长度的异质组合               │");
    println!("└──────────────────────────────────────────────────┘\n");

    // 元组创建
    let tup: (i32, f64, bool, char) = (42, 3.14, true, 'R');
    println!("  let tup = ({}, {}, {}, '{}')", tup.0, tup.1, tup.2, tup.3);
    println!();

    // 访问：点索引（不是方括号！）
    println!("  访问方式: tup.0 / tup.1 / tup.2 ...");
    println!("    tup.0 = {}", tup.0);   // 42
    println!("    tup.1 = {}", tup.1);   // 3.14
    println!("    tup.2 = {}", tup.2);   // true
    println!("    tup.3 = {}", tup.3);   // 'R'
    println!();

    // 单元类型 (Unit Type) —— 唯一的"零元素元组"
    let _unit: () = ();
    println!("  单元类型 () — 零元素元组");
    println!("    大小: {} byte", std::mem::size_of::<()>());
    println!("    用法: 函数无返回值时隐式返回 ()");
    println!();

    // 解构元组
    let pair = (100, "hello");
    let (num, word) = pair;
    println!("  解构: let (num, word) = (100, \"hello\"); → {} / {}", num, word);
    println!();

    // =====================================================
    //  6. 数组类型
    // =====================================================
    println!("┌──────────────────────────────────────────────────┐");
    println!("│ 6. 数组 (Array)：固定长度的同质集合               │");
    println!("└──────────────────────────────────────────────────┘\n");

    // 数组创建
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];  // 显式类型+长度
    let _arr2 = [3; 100];                      // [3, 3, 3, ..., 3] (100个)
    let _arr3 = [1, 2, 3];                     // 类型推断

    println!("  let arr1: [i32; 5] = [1, 2, 3, 4, 5];");
    println!("  let arr2 = [3; 100];  // 100 个 3");
    println!();

    // 索引访问 (usize)
    println!("  索引访问:");
    println!("    arr1[0]  = {}", arr1[0]);
    println!("    arr1[4]  = {}", arr1[4]);
    println!("  ⚠️  索引越界会导致 panic！(编译时无法检测)");
    println!();

    // 数组方法
    let arr = [10, 20, 30, 40, 50];
    println!("  常用方法:");
    println!("    arr.len()          = {}   (元素个数)", arr.len());
    println!("    arr.first()        = {:?} (第一个元素)", arr.first());
    println!("    arr.last()         = {:?} (最后一个)", arr.last());
    println!("    arr.contains(&30)  = {}   (是否包含)", arr.contains(&30));
    println!("    arr.iter().sum()   = {}   (求和)", arr.iter().sum::<i32>());
    println!("    arr[1..3] = {:?}    (切片)", &arr[1..3]);
    println!();

    // 数组在栈上
    println!("  数组大小 = {} bytes (5 × 4 = 20)", std::mem::size_of_val(&arr));
    println!("  数组在栈上分配，长度是类型的一部分！");
    println!("  [i32; 3] 和 [i32; 5] 是不同的类型！");
    println!();

    // =====================================================
    //  7. 类型转换
    // =====================================================
    println!("┌──────────────────────────────────────────────────┐");
    println!("│ 7. 类型转换：as / From / TryFrom                  │");
    println!("└──────────────────────────────────────────────────┘\n");

    // as：显式强制转换（危险！可能截断）
    let a_u32: u32 = 300;
    let a_u8 = a_u32 as u8;     // 300 % 256 = 44  (截断！)
    let a_i32 = a_u32 as i32;   // 安全 (u32的300在i32范围内)
    let f_to_i = 3.7f64 as i32; // 去掉小数部分 → 3 (不是四舍五入!)
    let i_to_f = 10i32 as f64;  // 安全

    println!("  as 转换 (显式，可能截断):");
    println!("    300u32 as u8   = {} (300 % 256，非预期!)", a_u8);
    println!("    300u32 as i32  = {} (安全转换)", a_i32);
    println!("    3.7f64 as i32  = {} (截断，不是四舍五入!)", f_to_i);
    println!("    10i32 as f64   = {} (安全提升)", i_to_f);
    println!();

    // From trait：安全转换（不会丢失数据）
    let from_i16: i32 = i32::from(42i16); // i16 → i32 永远安全
    let from_u8: u16 = u16::from(100u8);  // u8 → u16 永远安全
    println!("  From: 无损转换，编译器保证安全");
    println!("    i32::from(42i16)  = {}", from_i16);
    println!("    u16::from(100u8)  = {}", from_u8);
    println!();

    // TryFrom：可能失败的转换
    let try_u32: Result<u32, _> = u32::try_from(100i32);
    let try_neg: Result<u32, _> = u32::try_from(-5i32);
    println!("  TryFrom: 可能失败的转换，返回 Result");
    println!("    u32::try_from(100i32)  = {:?}", try_u32);
    println!("    u32::try_from(-5i32)   = {:?} (负数不能转u32)", try_neg);
    println!();

    // .into() 语法
    let into_u16: u16 = 100u8.into(); // 编译器推断目标类型
    println!("  .into(): 100u8.into()  → {} (类型自推断)", into_u16);
    println!("    当目标类型可从上下文推断时，.into() 更简洁");

    // =====================================================
    //  8. 类型别名
    // =====================================================
    println!("\n┌──────────────────────────────────────────────────┐");
    println!("│ 8. 类型别名 type                                  │");
    println!("└──────────────────────────────────────────────────┘\n");

    type Score = u32;               // 给 u32 取个更有意义的名字
    type Point = (f64, f64);        // 二维坐标

    let s: Score = 100;
    let p: Point = (3.0, 4.0);
    println!("  type Score = u32  → s = {}", s);
    println!("  type Point = (f64, f64) → p = ({}, {})", p.0, p.1);
    println!("  Score 和 u32 是同一个类型，可以互换使用");

    // =====================================================
    //  9. 总结
    // =====================================================
    println!("\n{}", "=".repeat(58));
    println!("📋  数据类型总结表");
    println!("{}", "=".repeat(58));

    println!(
        "
┌───────────────┬──────────────────────────────────────────────┐
│ 类型           │ 说明                                         │
├───────────────┼──────────────────────────────────────────────┤
│ i8/i16/i32... │ 有符号整数 (默认 i32)，补码表示                │
│ u8/u16/u32... │ 无符号整数                                    │
│ isize/usize   │ 指针宽度整数 (数组索引/长度专用)              │
│ f32/f64       │ IEEE-754 浮点 (默认 f64)                     │
│ bool          │ true / false，1 byte                          │
│ char          │ Unicode标量值，4 bytes                        │
│ ()            │ 单元类型 (零元素元组)，函数默认返回值          │
│ (T1,T2,...)   │ 元组，固定长度异质集合                        │
│ [T; N]        │ 数组，固定长度同质集合（栈上分配）            │
│ &[T]          │ 数组切片 (引用，后续章节详解)                 │
│ &str          │ 字符串切片 (引用，后续章节详解)               │
│ String        │ 堆上字符串 (后续章节详解)                     │
│ Vec<T>        │ 动态数组 (后续章节详解)                       │
└───────────────┴──────────────────────────────────────────────┘

  类型转换速查:
    as          → 显式转换，可能截断/丢失数据
    From::from  → 安全无损转换
    TryFrom     → 可能失败，返回 Result
    .into()     → From 的语法糖，目标类型需可推断
"
    );
}
