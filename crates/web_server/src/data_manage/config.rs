use crate::data_manage::config::ReadError::Custom;
use crate::pause;
use serde::de::{Error, StdError, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use std::fs;
use std::net::IpAddr;

pub async fn 初始化配置文件() {
    let f = fs::File::create("./config.json");
    if let Err(ref e) = f {
        eprintln!("打开或创建文件错误: {e}");
        pause();
        std::process::exit(2)
    }
    let mut f = f.unwrap();
    let config = Config::default();
    let result = serde_json::to_writer_pretty(&mut f, &config);
    if result.is_err() {
        eprintln!("初始化失败，退出程序");
        pause();
        std::process::exit(1)
    }
    eprintln!("初始化成功！");
}
pub async fn 将配置文件读取到结构体中() -> Result<Config, ReadError> {
    let f = fs::OpenOptions::new().read(true).open("./config.json");

    let f = match f {
        Ok(f) => f,
        Err(e) => return Err(ReadError::IoE(e)),
    };
    let config: serde_json::Result<Config> = serde_json::from_reader(f);
    match config {
        Ok(v) => Ok(v),
        Err(e) => Err(ReadError::SerdeJsonE(e)),
    }
}
#[derive(Debug)]
pub enum ReadError {
    SerdeJsonE(serde_json::Error),
    IoE(std::io::Error),
    Custom(String),
}
impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::SerdeJsonE(e) => {
                write!(f, "{}", e)
            }
            ReadError::IoE(e) => {
                write!(f, "{}", e)
            }
            Custom(e) => {
                write!(f, "{}", e)
            }
        }
    }
}
impl StdError for ReadError {}
impl Error for ReadError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        eprintln!("ReadError msg: {msg}");
        Custom(format!("{msg}"))
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    ip: IpAddr,
    proxy: Proxy,
    times: Option<usize>,
    suffix: String,
}

impl Config {
    pub fn ip(&self) -> IpAddr {
        self.ip
    }
    #[allow(private_interfaces)]
    pub fn proxy(&self) -> u16 {
        self.proxy.0
    }
    pub fn times(&self) -> Option<usize> {
        self.times
    }
    pub fn suffix(&self) -> &str {
        &self.suffix
    }
}
#[derive(Serialize, Debug, Copy, Clone)]
struct Proxy(u16);
struct ProxyVisitor;

impl Default for Proxy {
    fn default() -> Self {
        Proxy(7878)
    }
}

impl Display for Proxy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: IpAddr::from([127, 0, 0, 1]),
            proxy: Proxy(7878),
            times: None,
            suffix: ".wav".into(),
        }
    }
}
impl Visitor<'_> for ProxyVisitor {
    type Value = Proxy;
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("端口需在1~65535之间")
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u16(v as u16)
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v == 0 {
            return Err(E::custom("端口不能是0"));
        }
        Ok(Proxy(v))
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v >= 65536 {
            return Err(E::custom("端口过大"));
        }
        self.visit_u16(v as u16)
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v >= 65536 {
            return Err(E::custom("端口过大"));
        }
        self.visit_u16(v as u16)
    }
}

impl<'de> Deserialize<'de> for Proxy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ProxyVisitor)
    }
}
