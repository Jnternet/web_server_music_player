use rodio::{source::Source, Decoder, OutputStream};
use player::pause;
use std::io::{Cursor, Lines, Read, StdinLock};
use std::net::{SocketAddr, TcpStream};

fn main() {
    println!("执行中...");
    let mut lines = std::io::stdin().lines();
    let ip;
    loop {
        match 解析网址(&mut lines) {
            None => {
                println!("请重新输入");
                continue;
            }
            Some(ipa) => ip = ipa,
        }
        break;
    }
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    // let file = BufReader::new(File::open("2.wav").unwrap());
    // let mut file = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut file;
    loop {
        match TcpStream::connect(ip) {
            Ok(tcp) => file = tcp,
            Err(e) => {
                println!("尝试接收失败:{e}");
                println!("回车重试");
                pause();
                continue;
            }
        };
        break;
    }
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    // Decode that sound file into a source
    let cursor = Cursor::new(buf);
    let source = Decoder::new(cursor).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    println!("回车退出");
    pause();
}
fn 解析网址(lines: &mut Lines<StdinLock>) -> Option<SocketAddr> {
    println!("输入你要链接的网址:");
    let mut http = lines.next().unwrap().unwrap();
    match http.find("//") {
        None => {}
        Some(n) => {
            let (_, _http) = http.split_at(n + 2).to_owned();
            http = _http.to_string();
        }
    }

    match http.parse::<SocketAddr>() {
        Ok(ip) => {
            println!("解析成功:{ip}");
            Some(ip)
        }
        Err(e) => {
            println!("解析失败:{e}");
            None
        }
    }
}
