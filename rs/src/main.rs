#![allow(warnings)]

// 自有模块
mod algo;
mod async_await;
mod basics;
mod data_structure;
mod functional;
mod macros;
mod pointers;
mod templates;
mod threadings;
mod traits;
mod unsafe_rs;
mod design_pattern;

// 命令行
use dialoguer::Select;
use console::Style;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    // 标题
    let title = Style::new().bold().cyan();
    println!("{}", title.apply_to("请选择要运行的示例模块:"));

    // 所有示例模块文件夹列表
    let modules = &["basics", "async_await", "templates"];

    // 交互式选择
    let selection = Select::new()
        .items(modules)
        .default(0)
        .interact()
        .unwrap();

    // 扫描所有模块
    let pb = ProgressBar::new(20);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}")
            .unwrap()
            .progress_chars("#>-")
    );
    for _ in 0..20 {
        pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
    pb.finish_with_message("开始运行模块!");

    // 根据选择调用对应模块的 run 函数
    match selection {
        0 => basics::run_all(),      // basics/mod.rs 中写一个 run_all() 调用该文件夹下所有示例
        1 => async_await::run_all(),
        2 => templates::run_all(),
        _ => unreachable!(),
    }
}


