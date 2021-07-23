use clap::{App, Arg};
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::{mpsc, oneshot};

mod read;
mod write;

#[tokio::main]
async fn main() {
    let matches = App::new("replacer")
        .version("0.1.0")
        .author("jcsora. <jcsora@outlook.com>")
        .about("A tool that replaces the content of a source file with new content with different weights.")
        .arg(
            Arg::with_name("path")
            .index(1)
            .help("源文件路径")
            .takes_value(true)
            )
        .get_matches();

    let path = matches.value_of("path").unwrap_or_else(|| {
        println!("未指定源文件路径");
        std::process::exit(64);
    });
    let now = Instant::now();
    let (tx, rx) = mpsc::channel::<String>(32 * 1024);
    let (f_tx, f_rx) = oneshot::channel::<String>();
    let (l_tx, l_rx) = oneshot::channel::<HashMap<String, u64>>();
    let read = tokio::spawn(read::read_chunk(path.to_string(), tx, l_tx));
    let write = tokio::spawn(write::write_chunk(rx, f_tx));

    read.await.unwrap();
    match f_rx.await {
        Ok(v) => match l_rx.await {
            Ok(log) => {
                println!(
                    "替换成功，共耗时:{:?}秒\n新文件为: {:?}\n替换日志:\n{:?}",
                    now.elapsed().as_secs(),
                    v,
                    log
                )
            }
            Err(_) => println!(
                "替换成功, 共耗时:{:?}\n新文件为: {:?}",
                now.elapsed().as_secs(),
                v
            ),
        },
        Err(_) => println!("替换失败"),
    }
    write.await.unwrap();
}
