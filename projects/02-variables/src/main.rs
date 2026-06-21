// ============================================================
// Rust 变量绑定 完整示例
// 涵盖：let / mut / const / shadowing / 类型标注 / 解构 / 作用域
// ============================================================

fn main() {
    println!("{}", "=".repeat(55));
    println!("🦀  Rust 变量绑定 — 完整演示");
    println!("{}\n", "=".repeat(55));

    // =====================================================
    //  1. let 绑定：不可变是默认
    // =====================================================
    println!("┌─────────────────────────────────────────┐");
    println!("│ 1. let 绑定：默认不可变                  │");
    println!("└─────────────────────────────────────────┘\n");

    let x = 5;                    // Rust 推断 x: i32
    println!("  let x = 5  →  x = {}", x);

    // 下面这行取消注释会编译失败：
    // x = 6;
    // error[E0384]: cannot assign twice to immutable variable `x`

    println!("  ⚠️  x = 6 会报错：不可变变量不能二次赋值\n");

    // =====================================================
    //  2. mut：声明可变变量
    // =====================================================
    println!("┌─────────────────────────────────────────┐");
    println!("│ 2. mut：显式声明可变性                   │");
    println!("└─────────────────────────────────────────┘\n");

    let mut y = 10;
    println!("  let mut y = 10  →  y = {}", y);
    y = 20;
    println!("  y = 20;         →  y = {}", y);
    y += 5;
    println!("  y += 5;         →  y = {}", y);

    // 但是不能改变类型！
    // y = "hello";  // ❌ 类型不匹配：期望 i32，得到 &str
    println!("  ⚠️  y = \"hello\" 会报错：类型不匹配\n");

    // =====================================================
    //  3. const：编译时常量
    // =====================================================
    println!("┌─────────────────────────────────────────┐");
    println!("│ 3. const：编译时常量                      │");
    println!("└─────────────────────────────────────────┘\n");

    const MAX_PLAYERS: u32 = 100;       // 类型必须标注，值必须是编译期已知
    const APP_NAME: &str = "MyGame";     // 字符串常量
    const GOLDEN_RATIO: f64 = 1.618;     // 浮点常量

    println!("  const MAX_PLAYERS: u32 = {};", MAX_PLAYERS);
    println!("  const APP_NAME: &str    = \"{}\";", APP_NAME);
    println!("  const GOLDEN_RATIO: f64 = {};", GOLDEN_RATIO);

    // const vs let 关键区别：
    //   let    → 运行时求值，可以放函数返回值
    //   const  → 编译时求值，只能放常量表达式
    println!();
    println!("  🔑 const vs let 区别:");
    println!("     let   → 运行时求值，可以在任何作用域使用");
    println!("     const → 编译时求值，必须标注类型，可放全局");

    // =====================================================
    //  4. 变量遮蔽 (Shadowing)
    // =====================================================
    println!("\n┌─────────────────────────────────────────┐");
    println!("│ 4. 变量遮蔽 (Shadowing)                 │");
    println!("└─────────────────────────────────────────┘\n");

    let z = 1;
    println!("  let z = 1;                     → z = {}", z);

    let z = z + 2;          // 新变量 z 遮蔽旧 z
    println!("  let z = z + 2;                  → z = {}", z);

    {
        let z = z * 10;     // 作用域内再次遮蔽
        println!("  在内层: let z = z * 10;       → z = {}", z);
    }                       // 内层 z 离开作用域
    println!("  回到外层:                      → z = {} (恢复)", z);

    // shadowing 和 mut 的核心区别
    println!();
    let s = "hello";        // s: &str
    let s = s.len();        // 新 s: usize —— 类型变了！
    println!("  let s = \"hello\"; let s = s.len(); → s = {} (类型也变了)", s);

    let mut m = 5;
    println!("  let mut m = 5;                 → m = {}", m);
    m = 10;                 // 只能改值
    println!("  m = 10;                        → m = {}", m);
    // m = "ten";           // ❌ 类型不能变！
    println!();
    println!("  🔑 shadowing vs mut:");
    println!("     mut → 改变值，类型不变，需要 mut 关键字");
    println!("     遮蔽 → 重新绑定，可以改变类型，不需要 mut");

    // =====================================================
    //  5. 类型标注
    // =====================================================
    println!("\n┌─────────────────────────────────────────┐");
    println!("│ 5. 类型标注                              │");
    println!("└─────────────────────────────────────────┘\n");

    // 显式标注类型
    let a: i32 = 42;
    let b: f64 = 3.14159;
    let c: bool = true;
    let d: char = '🦀';                      // Rust 的 char 是 4 字节 Unicode
    let e: &str = "字符串切片";
    let f: String = String::from("堆上字符串");

    println!("  a: i32   = {}", a);
    println!("  b: f64   = {}", b);
    println!("  c: bool  = {}", c);
    println!("  d: char  = {} (4字节Unicode)", d);
    println!("  e: &str  = {}", e);
    println!("  f: String= {}", f);
    println!();

    // 编译器推断类型
    let guess = 42;             // 默认 i32
    let pi = 3.14159;           // 默认 f64
    let flag = true;            // 默认 bool
    let letter = 'A';           // 默认 char

    println!("  guess  = {} → 类型推断为 i32", guess);
    println!("  pi     = {} → 类型推断为 f64", pi);
    println!("  flag   = {} → 类型推断为 bool", flag);
    println!("  letter = {} → 类型推断为 char", letter);
    println!();

    // parse 必须标注类型（编译器无法推断目标类型）
    let num: i32 = "42".parse().expect("Not a number");
    println!("  \"42\".parse::<i32>() = {}  ← 必须标注目标类型", num);

    // 涡轮鱼语法 turbofish
    let num = "99".parse::<u64>().expect("Not a number");
    println!("  \"99\".parse::<u64>() = {}  ← 涡轮鱼语法", num);

    // =====================================================
    //  6. 解构赋值 (Destructuring)
    // =====================================================
    println!("\n┌─────────────────────────────────────────┐");
    println!("│ 6. 解构 (Destructuring)                 │");
    println!("└─────────────────────────────────────────┘\n");

    // 元组解构
    let pair = (100, 200);
    let (first, second) = pair;
    println!("  元组解构: let (first, second) = ({}, {});", first, second);
    println!("                → first={}, second={}", first, second);

    // 嵌套解构
    let nested = ((1, 2), 3);
    let ((a1, a2), a3) = nested;
    println!("  嵌套解构: let ((a1, a2), a3) = (({}, {}), {});", a1, a2, a3);

    // 部分解构（用 _ 忽略）
    let triple = (10, 20, 30);
    let (x1, _, x3) = triple;
    println!("  忽略字段: let (x1, _, x3) = ({}, {}, {});", triple.0, triple.1, triple.2);
    println!("               → x1={}, x3={}", x1, x3);

    // 用 .. 忽略剩余字段
    let big = (1, 2, 3, 4, 5);
    let (first, .., last) = big;
    println!("  忽略中间: let (first, ..., last) = (1,2,3,4,5);");
    println!("               → first={}, last={}", first, last);

    // =====================================================
    //  7. 作用域与生命周期
    // =====================================================
    println!("\n┌─────────────────────────────────────────┐");
    println!("│ 7. 作用域与 drop                         │");
    println!("└─────────────────────────────────────────┘\n");

    let outer = "外层变量";
    println!("  定义 outer: \"{}\"", outer);

    {
        let inner = "内层变量";
        println!("  进入内层作用域:");
        println!("    可以访问 outer: \"{}\"", outer);
        println!("    定义 inner: \"{}\"", inner);
        // inner 在这里离开作用域 → 被 drop
    }
    println!("  回到外层:");
    println!("    outer 仍然可用: \"{}\"", outer);
    println!("    inner 已经消失（被 drop）");
    // println!("{}", inner);  // ❌ 编译错误：not found in this scope

    // =====================================================
    //  8. 静态变量 static
    // =====================================================
    println!("\n┌─────────────────────────────────────────┐");
    println!("│ 8. static：全局静态变量                  │");
    println!("└─────────────────────────────────────────┘\n");

    // static 有固定内存地址，const 没有（内联展开）
    static COUNTER: u32 = 0; // 不可变 static

    println!("  static COUNTER: u32 = {};", COUNTER);
    println!("  🔑 static vs const:");
    println!("     static → 有固定内存地址，'static 生命周期");
    println!("     const  → 没有地址，编译时内联展开到使用处");

    // =====================================================
    //  9. 总结对比
    // =====================================================
    println!("\n{}", "=".repeat(55));
    println!("📋  总结对比");
    println!("{}", "=".repeat(55));
    println!(
        "
┌──────────┬────────────┬──────────┬──────────────┬────────────┐
│  关键字   │  可变性     │  类型变化  │  作用域       │  求值时机   │
├──────────┼────────────┼──────────┼──────────────┼────────────┤
│ let      │ 不可变      │ 不可变     │ 块级          │ 运行时     │
│ let mut  │ 可变        │ 不可变     │ 块级          │ 运行时     │
│ 遮蔽 let  │ 看情况      │ 可以变化   │ 块级          │ 运行时     │
│ const    │ 永远不可变   │ 必须标注   │ 任意（全局）  │ 编译时     │
│ static   │ 不可变(*)   │ 必须标注   │ 全局          │ 编译时     │
└──────────┴────────────┴──────────┴──────────────┴────────────┘

  (*) static 可变需要用 unsafe {{ ... }} 包裹"
    );
}
