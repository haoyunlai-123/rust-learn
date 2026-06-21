/// 所有权、引用、借用、生命周期综合示例
///
/// 这个程序模拟了一个简单的"图书馆借阅系统"，
/// 涵盖了 Rust 所有权的核心概念。

// ==================== 基础所有权 ====================

/// 一个「书」的结构体 —— 它拥有自己的数据（String 持有堆内存所有权）
#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
        }
    }

    /// 打印书籍信息（&self 是不可变引用 —— 只借用不拿走所有权）
    fn describe(&self) {
        println!(
            "📖 《{}》by {} ({} pages)",
            self.title, self.author, self.pages
        );
    }

    /// 判断是否大部头（同样是不可变借用）
    fn is_thick(&self) -> bool {
        self.pages > 300
    }
}

// ==================== 所有权转移示例 ====================

/// 这个函数「消费」一本书 —— Book 的所有权被转移进来，
/// 然后在函数结束时被 drop 释放。
/// 注意：调用后原变量不能再使用！
fn consume_book(book: Book) {
    println!("🔥 销毁书籍: {}", book.title);
    // book 在这里被 drop
}

/// 这个函数「返回」一本书 —— 所有权先移入，再移出。
fn add_bookmark(book: Book, bookmark: &str) -> Book {
    println!("📌 在《{}》中放入书签: {}", book.title, bookmark);
    book // 所有权返回给调用者
}

// ==================== 引用与借用 ====================

/// 「图书馆」结构体 —— 它借用了 Book 的引用，并不拥有这些书
struct Library<'a> {
    /// 图书馆的名字
    name: String,
    /// 书架 —— 借用了外部的 Book，生命周期标记 'a 确保引用有效
    shelves: Vec<&'a Book>,
}

impl<'a> Library<'a> {
    fn new(name: &str) -> Self {
        Library {
            name: name.to_string(),
            shelves: Vec::new(),
        }
    }

    /// 放入一本书（不可变引用 —— 图书馆只是展示，不拥有）
    fn add_book(&mut self, book: &'a Book) {
        self.shelves.push(book);
    }

    /// 列出所有藏书（遍历不可变引用）
    fn list_books(&self) {
        println!("\n🏛️  【{}】藏书列表:", self.name);
        if self.shelves.is_empty() {
            println!("   (空书架)");
        } else {
            for (i, book) in self.shelves.iter().enumerate() {
                println!("  {}. {}", i + 1, book.title);
            }
        }
    }
}

// ==================== 可变引用与独占借用 ====================

/// 「借书证」—— 持有一本可变借出的书（可变引用）
struct BorrowCard<'a> {
    book: &'a mut Book,
    holder: String,
    due_days: u32,
}

impl<'a> BorrowCard<'a> {
    /// 借出一本书 —— 获得可变引用
    /// 这意味着原书架在此期间不能读取这本书！
    fn borrow(book: &'a mut Book, holder: &str, days: u32) -> Self {
        println!(
            "🔄 {} 借出了《{}》，{}天内归还",
            holder, book.title, days
        );
        BorrowCard {
            book,
            holder: holder.to_string(),
            due_days: days,
        }
    }

    /// 在书上做笔记（通过可变引用修改）
    fn annotate(&mut self, note: &str) {
        // 这里演示了 &mut self 再借用 &mut self.book 的字段
        let title = &self.book.title;
        println!("✏️  在《{}》上写笔记: {}", title, note);
    }

    /// 延期归还 —— 修改借书证信息
    fn extend(&mut self, extra_days: u32) {
        self.due_days += extra_days;
        println!(
            "📅 《{}》延期至 {} 天后归还",
            self.book.title, self.due_days
        );
    }

    /// 还书 —— 放弃可变引用，返回 &mut 以便原所有者恢复访问
    fn return_book(self) -> &'a mut Book {
        println!(
            "✅ {} 归还了《{}》",
            self.holder, self.book.title
        );
        self.book // 转移可变引用所有权出去
    }
}

// ==================== 字符串切片示例 ====================

/// 分析一段文本并返回各部分引用（演示 &str 切片）
fn analyze_text<'a>(text: &'a str, keyword: &str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| line.contains(keyword))
        .collect()
}

// ==================== 生命周期进阶：多个引用 ====================

/// 选择两本书中页数更多的那本 —— 返回的引用生命周期
/// 取两个输入中较短的那个
fn thicker_book<'a>(a: &'a Book, b: &'a Book) -> &'a Book {
    if a.pages >= b.pages { a } else { b }
}

/// 带生命周期约束的结构体 —— "读者"可以引用一本书
struct Reader<'a> {
    name: String,
    current_book: Option<&'a Book>,
}

impl<'a> Reader<'a> {
    fn new(name: &str) -> Self {
        Reader {
            name: name.to_string(),
            current_book: None,
        }
    }

    fn take_book(&mut self, book: &'a Book) {
        println!("👤 {} 开始阅读《{}》", self.name, book.title);
        self.current_book = Some(book);
    }

    fn reading(&self) {
        if let Some(book) = &self.current_book {
            println!("📖 {} 正在阅读《{}》...", self.name, book.title);
        } else {
            println!("📖 {} 没有在读书", self.name);
        }
    }

    fn finish_book(&mut self) {
        if let Some(book) = self.current_book.take() {
            println!("🏁 {} 读完了《{}》", self.name, book.title);
        }
    }
}

// ==================== main：演示所有概念 ====================

fn main() {
    println!("{}", "=".repeat(50));
    println!("📚  Rust 所有权学习示例 —— 图书馆借阅系统");
    println!("{}\n", "=".repeat(50));

    // ---------- 1. 所有权基础 ----------
    println!("【1. 所有权基础 —— 创建书籍】");
    let book1 = Book::new("Rust 程序设计", "张三", 450);
    let book2 = Book::new("算法导论", "李四", 1200);
    let book3 = Book::new("小王子", "圣埃克苏佩里", 96);
    let mut book4 = Book::new("待修改的书", "佚名", 200);

    book1.describe();
    book2.describe();
    book3.describe();
    println!();

    // ---------- 2. 所有权转移（move）----------
    println!("【2. 所有权转移（move）】");
    let moved = add_bookmark(book3, "第42页"); // book3 的所有权移入，又被返回
    println!("  书签已添加，仍然可以访问: {}", moved.title);
    // 如果取消下面这行注释，编译会报错 —— book3 已经 moved！
    // println!("{}", book3.title);
    println!();

    // ---------- 3. 消费与释放 ----------
    println!("【3. 消费所有权】");
    // clone 一份来演示消费，因为 moved 现在拥有 book3 的所有权
    let clone_book = moved.clone();
    consume_book(clone_book);
    // clone_book 这里已不可用
    println!();

    // ---------- 4. 不可变引用（借用）----------
    println!("【4. 不可变引用 —— 同时读取】");
    // 多个不可变引用可以共存
    let ref1 = &book1;
    let ref2 = &book1;
    println!("  两个引用同时读取:");
    println!("    ref1: {}", ref1.title);
    println!("    ref2: {}", ref2.title);
    // 原始所有者仍然可以读取
    println!("    owner: {}", book1.title);
    println!();

    // ---------- 5. 可变引用（独占借用）----------
    println!("【5. 可变引用 —— 独占访问】");
    {
        let mut_card = BorrowCard::borrow(&mut book4, "王五", 14);
        // 下面这行如果取消注释会编译失败 —— book4 已被可变借用
        // println!("{}", book4.title);
        // 但可以通过 mut_card 访问
        println!("  通过借书证访问: {}", mut_card.book.title);

        let mut mut_card = mut_card; // 重新绑定为可变
        mut_card.annotate("这是一本测试书");
        mut_card.extend(7);

        // 还书，恢复可变引用的所有权
        let _ = mut_card.return_book();
    }
    // 现在可以安全访问 book4 了
    println!("  还书后可以再次访问: {}", book4.title);
    println!();

    // ---------- 6. 生命周期与结构体中的引用 ----------
    println!("【6. 生命周期 —— 图书馆与读者】");
    let mut library = Library::new("市中心图书馆");
    library.add_book(&book1);
    library.add_book(&book2);
    library.add_book(&moved);
    library.list_books();

    // 多个读者可以同时借阅同一本书（不可变引用）
    let mut reader1 = Reader::new("Alice");
    let mut reader2 = Reader::new("Bob");
    reader1.take_book(&book1);
    reader2.take_book(&book1); // 没问题！不可变引用可以共享
    reader1.reading();
    reader2.reading();
    reader1.finish_book();
    reader2.finish_book();
    println!();

    // ---------- 7. 比较两本书（生命周期）----------
    println!("【7. 生命周期标注 —— 返回引用】");
    let thicker = thicker_book(&book1, &book2);
    println!(
        "  比较厚的书是: 《{}》({} pages)",
        thicker.title, thicker.pages
    );
    println!();

    // ---------- 8. 切片与借用 ----------
    println!("【8. 字符串切片 —— &str】");
    let text = String::from("Rust is fast\nRust is safe\nRust is fun\nPython is easy");
    let rust_lines = analyze_text(&text, "Rust");
    println!("  包含 'Rust' 的行:");
    for line in &rust_lines {
        println!("    - {}", line);
    }
    // rust_lines 借用了 text，所以 text 在此期间不能被修改
    println!("  原文仍可访问: {} 行", text.lines().count());
    println!();

    // ---------- 9. 所有权总结 ----------
    println!("{}", "=".repeat(50));
    println!("📋  关键概念回顾");
    println!("{}", "=".repeat(50));
    println!(
        "\
1. 所有权规则：
   - 每个值有且只有一个所有者
   - 所有权可以转移（move）
   - 所有者离开作用域，值被 drop

2. 借用规则：
   - 要么有任意多个不可变引用（&T）
   - 要么有且只有一个可变引用（&mut T）
   - 两者不能同时存在

3. 生命周期：
   - 每个引用都有生命周期
   - 生命周期标注 'a 确保引用不会悬空
   - 结构体中的引用需要生命周期标注

4. &str 是字符串切片 —— 对 String 的借用"
    );
}
