# Rust 项目基本组成、Cargo 与 Hello World 详解

> 对应项目：`projects/01-hello-world`
> 掌握目标：理解 Rust 项目骨架，能用 cargo 创建/构建/运行项目

---

## 一、Rust 项目的最小组成

一个最小的 Rust 可执行项目只需要两个文件：

```
01-hello-world/
├── Cargo.toml          ← 项目"身份证"：元数据 + 依赖声明
└── src/
    └── main.rs         ← 程序入口：main 函数
```

除了这两个文件，其他一切（`target/`、`Cargo.lock`、`.gitignore`）都是**生成产物或辅助文件**，不是必须的。

---

## 二、Cargo.toml 逐行拆解

```toml
[package]                    # ← 节（section），配置包本身的元信息
name = "hello-world"         # 包名：唯一标识符，发布到 crates.io 时不能重名
version = "0.1.0"            # 语义化版本号：主版本.次版本.修订号
edition = "2024"             # Rust 语法的"年代版"，决定了编译器接受的语法

[dependencies]               # ← 依赖声明节，当前为空（无第三方依赖）
```

### 深入解释

#### `[package]` 节

| 字段 | 必填 | 说明 |
|------|------|------|
| `name` | ✅ | 包名。命名规则：字母 + 数字 + `-` + `_`，不能以数字开头 |
| `version` | ✅ | 语义化版本。`0.1.0` 表示"开发阶段，API 不稳定" |
| `edition` | ✅ | Rust 的向后不兼容版本边界。`2015` → `2018` → `2021` → `2024` |
| `authors` | ❌ | 作者信息（2024 edition 后默认不生成） |
| `description` | ❌ | 项目简介（发布用） |

#### Edition（年代版）是什么意思？

Rust 保证**同一个 edition 内的代码永远能编译**。新增关键字、改变语法时，Rust 会通过 edition 机制让旧代码不受影响：

```
2015 edition:  async、await 是普通标识符
2018 edition:  async、await 变成关键字
2024 edition:  最新的语法改进
```

你的 `main.rs` 第一行 `fn main()` 就运行在 2024 edition 的规则下。

#### `[dependencies]` 节

```toml
[dependencies]
serde = "1.0"           # 声明依赖 serde 库，版本 >=1.0.0, <2.0.0
rand = { version = "0.8", features = ["std"] }  # 带 feature 选择
```

`"1.0"` 是 Cargo 的**脱字符 `^` 语义**：等价于 `>=1.0.0, <2.0.0`。

---

## 三、src/main.rs 逐行拆解

```rust
fn main() {
    println!("Hello, world!");
}
```

### 逐 token 解释

```
fn          ← 关键字，声明一个函数（function）
main        ← 函数名。main 有特殊含义：它是程序的入口点
()          ← 参数列表。空括号 = 不接受任何参数
{           ← 函数体开始
println!    ← 宏调用（注意 ! 后缀）。宏在编译时展开为代码
(           ← 宏的参数开始
"Hello, world!"  ← 字符串字面量，类型是 &str（字符串切片）
)           ← 参数结束
;           ← 语句结束符（分号表示"不返回值"，即丢弃表达式的值）
}           ← 函数体结束
```

### println! 是宏，不是函数

这是 Rust 初学时最容易混淆的点。`!` 后缀是关键区别：

```rust
println!("hello");    // ← 宏，编译时展开；参数数量可变
std::io::stdin();     // ← 函数，运行时调用；参数数量固定
```

`println!` 宏的内部会：
1. 检查格式字符串和参数数量是否匹配（编译时检查！）
2. 获取标准输出的锁
3. 写入格式化后的字符串
4. 追加换行符

```rust
// 下面的代码会直接编译失败，而不是运行时崩溃
println!("{} + {} = {}", 1, 2);  // ❌ 编译错误：需要 3 个参数，只给了 2 个
```

### main 函数的三要素

```rust
fn main() { }           // ✅ 无参数，无返回值
fn main() -> ! { }      // ✅ 无参数，返回发散类型（不返回）
// fn main(&self) { }   // ❌ main 不能有参数（除了特定场景）
```

程序启动时，操作系统调用 `main`；`main` 结束时，程序退出。返回值通过 `std::process::exit` 或隐式的 `0`（成功）。

---

## 四、Cargo：Rust 的 Swiss Army Knife

Cargo 是 Rust 的**构建系统 + 包管理器 + 测试运行器 + 文档生成器**，一个工具干了四个活。

### cargo new —— 创建项目

```bash
cargo new project-name                 # 创建二进制项目（有 main.rs）
cargo new lib-name --lib               # 创建库项目（有 lib.rs）
cargo new 01-hello-world --name hello-world  # 目录名 ≠ 包名
```

`cargo new` 做了什么：
1. 创建项目目录
2. 生成 `Cargo.toml`（模板）
3. 生成 `src/main.rs`（Hello World 模板）
4. 初始化 `.git` 仓库
5. 生成 `.gitignore`（内含 `/target`）

### cargo build —— 编译

```bash
cargo build                 # 默认：dev profile，有调试信息，未优化
cargo build --release       # release profile：高度优化，去调试信息
```

产物路径：
```
target/debug/hello-world.exe          ← dev 产物（编译快，运行慢）
target/release/hello-world.exe        ← release 产物（编译慢，运行快）
```

### cargo run —— 编译 + 运行

```bash
cargo run                   # = cargo build + 执行程序
cargo run --release         # release 模式
cargo run -- --help         # -- 后面的参数传给程序本身
```

### cargo check —— 只检查不编译

```bash
cargo check                 # 比 cargo build 快很多，只验证语法和类型
```

适合在频繁修改时用，省去生成二进制文件的时间。

### cargo clean —— 清理编译产物

```bash
cargo clean                 # 删除整个 target/ 目录
```

### 其他常用命令

| 命令 | 用途 |
|------|------|
| `cargo test` | 运行测试 |
| `cargo doc --open` | 生成文档并在浏览器打开 |
| `cargo fmt` | 格式化代码（需安装 rustfmt） |
| `cargo clippy` | 静态分析/代码审查（需安装 clippy） |
| `cargo update` | 更新依赖到最新兼容版本 |
| `cargo tree` | 显示依赖树 |

---

## 五、target/ 目录详解

编译后自动生成，永远不要手动修改，不要加入版本控制：

```
target/
├── .rustc_info.json      ← 编译该目录的 rustc 版本信息（用于缓存失效判断）
├── CACHEDIR.TAG          ← 标记此目录是缓存，备份工具可跳过
├── debug/                ← dev 模式编译产物
│   ├── .fingerprint/     ← 增量编译指纹（判断哪些文件需要重编）
│   ├── build/            ← build.rs 脚本产物
│   ├── deps/             ← 编译的依赖库
│   ├── examples/         ← examples/ 目录下的示例可执行文件
│   └── hello-world.exe   ← 你的程序本体
└── release/              ← release 模式编译产物（结构同上）
```

---

## 六、Cargo.lock —— 依赖锁定文件

```toml
# 自动生成，不要手动编辑
[[package]]
name = "hello-world"
version = "0.1.0"
```

- **库项目**：`.gitignore` 应排除 `Cargo.lock`（让使用者自动解析最新兼容版本）
- **可执行项目**：`Cargo.lock` **应该提交**到 git（确保所有人构建出一致的二进制）

---

## 七、一次 cargo run 背后发生了什么

```
cargo run
    │
    ├─ 1. 读取 Cargo.toml → 解析元数据和依赖
    │
    ├─ 2. 检查 Cargo.lock → 锁定依赖版本
    │     没有 lock 文件？→ 解析依赖 → 生成 lock
    │
    ├─ 3. 下载依赖（如果需要）→ 缓存到 ~/.cargo/registry/
    │
    ├─ 4. 编译依赖（如果未编译）→ 输出到 target/debug/deps/
    │     增量编译：只编译改动过的文件
    │
    ├─ 5. 编译自己的代码 → 链接 → 生成 target/debug/hello-world.exe
    │
    └─ 6. 执行 hello-world.exe → 输出到终端
```

---

## 八、常见问题

### Q: 包名和目录名可以不同吗？
可以。使用 `cargo new 目录名 --name 包名`。包名受命名规则限制（如不能以数字开头），目录名没有限制。

### Q: 什么时候用 cargo check 而不是 cargo build？
在编辑器里频繁修改代码时，用 `cargo check` 快速验证语法错误，比 `cargo build` 快很多。

### Q: target/ 占空间太大怎么办？
```bash
cargo clean          # 全删
# 或者定期清理，需要时再编译
```

### Q: 为什么有两个 .gitignore？
- 根目录的 `.gitignore`：排除 `target/`（你写的）
- 项目目录的 `.gitignore`：`cargo new` 自动生成，同样排除 `target/`

---

## 总结速查表

| 概念 | 一句话 |
|------|--------|
| **Cargo.toml** | 项目的身份证：名字、版本、依赖 |
| **src/main.rs** | 可执行项目的入口文件，必须有 `fn main()` |
| **cargo new** | 脚手架：创建项目目录和模板文件 |
| **cargo build** | 编译（dev 或 --release） |
| **cargo run** | 编译 + 运行 |
| **cargo check** | 快速语法检查，不生成二进制 |
| **target/** | 编译产物目录，不提交 git |
| **Cargo.lock** | 精确锁定依赖版本（可执行项目应提交） |
| **edition** | Rust 的"语法年代"，向后兼容的关键 |
| **println!** | 宏（有 `!`），不是函数；编译时做格式检查 |
