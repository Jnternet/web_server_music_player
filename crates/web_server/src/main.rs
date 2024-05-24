use ::web_server::data_manage::*;
use ::web_server::{pause, view};

#[tokio::main]
async fn main() {
    let config = match config::将配置文件读取到结构体中().await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("未能成功读取配置文件, 原因: {e}");
            eprintln!("正在初始化为默认状态");
            config::初始化配置文件().await;
            config::将配置文件读取到结构体中().await.unwrap()
        }
    };
    loop {
        let v = file_manage::按后缀搜索文件并按修改日期倒叙排序(config.suffix()).await;
        if v.is_none() {
            println!("无文件，请将文件放入当前文件夹或其子文件夹下再重试");
            pause();
            continue;
        }
        let v = v.unwrap();
        view::展示文件及修改时间(&v);
        let f = view::选择文件(&v);
        view::启动服务器前展示配置(&config);
        web_server::请求处理::监听端口等待并处理任务(f.path().into(), &config);
        break;
    }
    pause();
}
