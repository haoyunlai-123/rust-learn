# Rust 系统学习大纲

> 目标：从零基础到能独立开发 Rust 项目
> 每个章节对应 `projects/` 中的一个项目 + `learn-notes/` 中的一篇笔记

---

## 第一阶段：基础入门

| 编号 | 项目 | 知识点 | 学习笔记 |
|------|------|--------|----------|
| 01 | hello-world | cargo 项目管理、println!、注释 | — |
| 02 | variables | 变量绑定、let / mut、常量 const、变量遮蔽 shadowing | variables.md |
| 03 | data-types | 标量类型（i32/f64/bool/char）、复合类型（tuple/array）、类型推导 | data-types.md |
| 04 | functions | 函数定义、参数、返回值、表达式 vs 语句、单元类型 () | functions.md |
| 05 | control-flow | if/else、loop、while、for、break/continue、标签循环 | control-flow.md |
| 06 | ownership | 所有权规则、移动 move、克隆 clone、Copy trait | ownership.md |
| 07 | references | 引用 &、可变引用 &mut、借用规则、悬垂引用 | references.md |
| 08 | slices | 字符串切片 &str、数组切片、范围语法 .. | slices.md |
| 09 | structs | 结构体定义/实例化、字段初始化简写、元组结构体、单元结构体、方法 method、关联函数 | structs.md |
| 10 | enums | 枚举定义、Option\<T\>、模式匹配 match、if let、matches! 宏 | enums.md |

---

## 第二阶段：集合与模块

| 编号 | 项目 | 知识点 | 学习笔记 |
|------|------|--------|----------|
| 11 | vectors | Vec\<T\> 创建/增删/遍历、与数组对比、常用方法 | vectors.md |
| 12 | strings | String vs &str、UTF-8 编码、字符串拼接/格式化、遍历字符 vs 字节 | strings.md |
| 13 | hashmap | HashMap 创建/插入/访问/更新、entry API、哈希函数 | hashmap.md |
| 14 | modules | mod、pub、use、as、嵌套模块、模块文件拆分、可见性规则 | modules.md |
| 15 | packages-crates | package vs crate、Cargo.toml、lib.rs vs main.rs、外部依赖 | packages-crates.md |

---

## 第三阶段：错误处理与泛型

| 编号 | 项目 | 知识点 | 学习笔记 |
|------|------|--------|----------|
| 16 | error-handling | panic!、Result\<T,E\>、? 运算符、unwrap/expect、自定义错误类型 | error-handling.md |
| 17 | generics | 泛型函数/结构体/枚举/方法、单态化、const 泛型 | generics.md |
| 18 | traits | trait 定义与实现、默认实现、trait bound、impl Trait 语法、trait 对象、关联类型、Sized trait | traits.md |
| 19 | lifetimes | 生命周期标注、函数签名中的生命周期、结构体中的生命周期、生命周期省略规则、'static | lifetimes.md |

---

## 第四阶段：标准库深入

| 编号 | 项目 | 知识点 | 学习笔记 |
|------|------|--------|----------|
| 20 | closures | 闭包语法、类型推导、捕获环境（不可变/可变/所有权）、Fn/FnMut/FnOnce | closures.md |
| 21 | iterators | Iterator trait、迭代器适配器（map/filter/take/zip 等）、消费器（collect/sum/fold 等）、惰性求值、into_iter vs iter vs iter_mut | iterators.md |
| 22 | collections-deep | VecDeque、LinkedList、BTreeMap、BinaryHeap、集合操作 | collections-deep.md |
| 23 | smart-pointers | Box\<T\>、Deref trait、Drop trait、Rc\<T\>、RefCell\<T\>、内部可变性、循环引用 | smart-pointers.md |
| 24 | pattern-matching | 全模式语法：字面量/变量/通配符、解构（结构体/枚举/元组）、匹配守卫、@ 绑定、ref/ref mut | pattern-matching.md |

---

## 第五阶段：并发与异步

| 编号 | 项目 | 知识点 | 学习笔记 |
|------|------|--------|----------|
| 25 | threads | thread::spawn、join、move 闭包、消息传递 mpsc、共享内存 Mutex/Arc | threads.md |
| 26 | sync-traits | Send 与 Sync trait、线程安全、原子类型 Atomic、Barrier/Condvar | sync-traits.md |
| 27 | async-basics | async/await 语法、Future trait、tokio 入门、async fn、.await | async-basics.md |
| 28 | tokio-deep | tokio 运行时、spawn/task、select!、channel、互斥锁、IO | tokio-deep.md |
| 29 | streams | Stream trait、迭代/合并/缓冲、背压、channel 流式处理 | streams.md |

---

## 第六阶段：工程实践

| 编号 | 项目 | 知识点 | 学习笔记 |
|------|------|--------|----------|
| 30 | testing | 单元测试 #[test]、集成测试 tests/、文档测试、assert 宏、should_panic、测试组织 | testing.md |
| 31 | cargo-deep | 工作空间 workspace、feature 条件编译、build script、发布配置 profile、自定义命令 | cargo-deep.md |
| 32 | logging | log crate + env_logger、tracing 入门、结构化日志、日志级别 | logging.md |
| 33 | serde | 序列化/反序列化、JSON/YAML/TOML、自定义序列化、#[serde(...)] 属性 | serde.md |
| 34 | cli-app | clap/bpaf 命令行解析、环境变量、退出码、终端输出、管道处理 | cli-app.md |

---

## 第七阶段：高级主题

| 编号 | 项目 | 知识点 | 学习笔记 |
|------|------|--------|----------|
| 35 | unsafe | unsafe 块、原始指针、unsafe 函数/trait/impl、FFI 外部调用、变体（协变/逆变） | unsafe.md |
| 36 | macros-declarative | macro_rules!、声明宏语法、重复模式、递归宏、调试宏 | macros-declarative.md |
| 37 | macros-procedural | 过程宏：derive/attribute/function-like、syn + quote、proc-macro2 | macros-procedural.md |
| 38 | network-tcp | std::net（TcpListener/TcpStream）、阻塞 IO、简单协议设计 | network-tcp.md |
| 39 | network-http | HTTP 协议基础、hyper/axum、路由、中间件、请求/响应处理 | network-http.md |
| 40 | database | sqlx / diesel、连接池、CRUD、事务、迁移 migration | database.md |

---

## 第八阶段：综合项目实战

| 编号 | 项目 | 说明 | 知识点整合 |
|------|------|------|------------|
| 41 | minigrep | 命令行文本搜索工具（参考《Rust 程序设计语言》第 12 章） | 模块、错误处理、迭代器、测试、环境变量 |
| 42 | web-server | 多线程 HTTP 服务器（参考《Rust 程序设计语言》第 20 章） | 线程池、TcpStream、并发、trait 对象 |
| 43 | todo-cli | 命令行待办事项管理 | CLI、文件 IO、serde、错误处理 |
| 44 | chat-room | 简单聊天室（客户端 + 服务端） | tokio、网络、序列化、并发 |
| 45 | blog-api | RESTful 博客 API 服务 | axum/sqlx、认证 JWT、CRUD、测试 |
| 46 | redisish | 迷你内存数据库 | 网络协议、数据结构、并发、持久化 |
| 47 | wasm-game | WebAssembly 小游戏 | wasm-bindgen、web-sys、游戏循环 |
| 48 | final-project | 自选项目：把你学到的做一个自己想用的工具 | 综合运用 |

---

## 推荐学习节奏

```
阶段 1-2（基础）     ：每天 1-2 个项目，预计 2 周
阶段 3-4（核心）     ：每 1-2 天 1 个项目，预计 3 周
阶段 5（并发异步）    ：每 2-3 天 1 个项目，预计 2 周
阶段 6（工程化）      ：每 1-2 天 1 个项目，预计 2 周
阶段 7（高级）        ：每 2-3 天 1 个项目，预计 3 周
阶段 8（实战）        ：每 3-5 天 1 个项目，预计 4 周

总计：约 16 周（4 个月）系统掌握 Rust
```

## 官方资源对照

| 本书章节 | 对应官方资料 |
|----------|-------------|
| 阶段 1-4 | [The Rust Book](https://doc.rust-lang.org/book/) |
| 阶段 4 | [Rust By Example](https://doc.rust-lang.org/rust-by-example/) |
| 阶段 7（宏） | [The Little Book of Rust Macros](https://veykril.github.io/tlborm/) |
| 阶段 5（异步） | [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/) |
| 全程练习 | [Rustlings](https://github.com/rust-lang/rustlings)（放入 exercises/rustlings/） |
| 参考手册 | [The Rust Reference](https://doc.rust-lang.org/reference/) |
