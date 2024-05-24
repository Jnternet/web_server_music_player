use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

pub async fn 按后缀搜索文件并按修改日期倒叙排序(
    suffix: &str,
) -> Option<Vec<(DirEntry, SystemTime)>> {
    let v = WalkDir::new("./")
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.file_name().to_str().unwrap().ends_with(suffix))
        .collect::<Vec<_>>();
    let mut v: Vec<_> = v
        .into_iter()
        .map(|v| {
            let time = v.metadata().unwrap().modified().unwrap();
            (v, time)
        })
        .collect();
    if v.is_empty() {
        eprintln!("未找到后缀为{suffix}的文件");
        return None;
    }
    v.sort_by(|a, b| b.1.cmp(&a.1));
    Some(v)
}
