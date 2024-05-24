use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::DirEntry;

#[allow(deprecated)]
pub fn 展示文件及修改时间(v: &[(DirEntry, SystemTime)]) {
    v.iter().enumerate().for_each(|(x, (a, b))| {
        println!(
            "{x}: {}, \t最后修改日期: {}",
            a.file_name().to_str().unwrap(),
            DateTime::<Local>::from_utc(
                NaiveDateTime::from_timestamp(
                    b.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                    0,
                ),
                FixedOffset::east_opt(28800).unwrap(),
            )
        )
    });
}

use crate::data_manage::config::Config;
use std::io::BufRead;

pub fn 选择文件(v: &[(DirEntry, SystemTime)]) -> DirEntry {
    println!("编号为0的是正确文件吗(回车/编号)");
    let mut num = 0;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let parse_result = line.parse::<usize>();
        if let Err(e) = parse_result {
            eprintln!("输入错误, 请重新输入 原因: {e}");
            continue;
        }
        let parse_result = parse_result.unwrap();
        if parse_result > v.len() - 1 {
            println!("超出范围，请重新输入,回车使用最近的选择");
            continue;
        }
        num = parse_result;
        println!("选择的文件{}", v[num].0.file_name().to_str().unwrap());
        println!("重新选择？回车下一步");
    }
    println!("最终确定的文件{}", v[num].0.file_name().to_str().unwrap());
    v[num].0.clone()
}

pub fn 启动服务器前展示配置(config: &Config) {
    println!("正在启动web server 端口: {}", config.proxy());
    match config.times() {
        None => {
            println!("不设服务次数上限");
        }
        Some(times) => {
            println!("服务{times}次");
        }
    }
    if config.ip().is_ipv4() {
        println!("http://{}:{}", config.ip(), config.proxy());
    } else {
        println!("http://[{}]:{}", config.ip(), config.proxy())
    }
}
