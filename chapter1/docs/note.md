# chapter2 Note

## rustup && Cargo

可以使用 `rustup` 来安装 `Rust` 和管理 `Rust` 版本，`Cargo` 是 `Rust` 的构建系统和包管理器。

版本查询可以使用下面三个命令：

```shel
cargo --version

rustc --version

rustdoc --version
```

- cargo 是 Rust 的构建系统和包管理器,可以用Cargo来创建新项目，构建项目，运行项目，测试项目等等。
- rustc 是 Rust 的编译器，可以用来编译单个文件。 并且可以通过 `rustc --crate-type=lib lib.rs` 来编译库文件。 以及cargo 会调用 rustc 来编译项目。
- rustdoc 是 Rust 的文档生成器，可以用来生成文档。

cargo 的一些常用命令：

```shell
cargo new hello_cargo # 创建一个新的项目

cargo run # 编译并运行项目

cargo build # 编译项目

cargo test # 测试项目

cargo clean # 清理项目
```

## rust 函数

```rust
fn gcd(mut n: u64, mut n : u64) -> u64 {
    assert!(n != 0 && m !=  0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
    }
    m = m % n;
    }
    n
}
```


isize 类型和usize类型保存着一个机器的原生字长，分别是有符号和无符号的整数类型。 isize 和 usize 类型的大小取决于程序运行的计算机类型。
在32位平台上是32位长，在64位平台上是64位长。

如果一个函数体以没有尾随着分号的表达式结尾，那么这个表达式就是函数的返回值。 花括号包起来的任意代码块都是可以用作表达式。

## 测试

#[test] 标记是属性的一种形式，属性是一种元数据，可以附加到各种声明上，包括模块，函数，结构体，枚举，静态变量等等。

#[test] 将test_gcd 标记为 测试函数，在正常编译时会跳过它，但是在运行 cargo test 时会编译并运行它。
