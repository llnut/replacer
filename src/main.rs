use bytes::BytesMut;
use rand::Rng;
use serde_derive::Deserialize;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Deserialize)]
struct Config {
    pat: Option<String>,
    to: Option<Vec<(String, u8)>>,
}

#[derive(Debug)]
struct RepStr {
    s: String,
    w_l: u8,
    w_r: u8,
}

fn build_rep_conf(v: Vec<u8>) -> (String, Vec<RepStr>) {
    let config: Config = toml::from_slice(&v[..]).unwrap();
    let mut to: Vec<RepStr> = Vec::new();
    let mut w_pos: u8 = 1;

    if let None = config.pat {
        panic!("Config.toml 内未设置 pat 项");
    }
    if let Some(t) = config.to {
        for (s, w) in t.iter() {
            to.push(RepStr {
                s: s.to_string(),
                w_l: w_pos,
                w_r: w_pos + w - 1,
            });
            w_pos += w;
            if w_pos > 101 {
                panic!("Config.toml 内的 to 总概率不能大于100");
            }
        }
    }
    (config.pat.unwrap(), to)
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        panic!("未指定源文件路径");
    }

    let (tx, mut rx) = mpsc::channel::<String>(8 * 1024);
    let (f_tx, f_rx) = oneshot::channel::<String>();

    let read = tokio::spawn(async move {
        //替换过程的配置
        let pat: String;
        let rep_str: Vec<RepStr>;
        match File::open("Config.toml").await {
            Ok(mut config) => {
                let mut conf_buf: Vec<u8> = Vec::new();
                let n = config.read_to_end(&mut conf_buf).await.unwrap();
                if n == 0 {
                    panic!("Config.toml 配置错误");
                }
                let rep_conf = build_rep_conf(conf_buf);
                pat = rep_conf.0;
                rep_str = rep_conf.1;
            }
            Err(_) => {
                panic!("未找到 Config.toml");
            }
        }

        //要替换的源文件
        let mut file = File::open(&args[1]).await.unwrap();
        let mut buf = BytesMut::with_capacity(8 * 1024);
        let mut n: usize;
        let mut chunk_tail: String = "".to_string();
        let mut chunk: String;

        loop {
            n = file.read_buf(&mut buf).await.unwrap();
            if n == 0 {
                if chunk_tail.len() != 0 {
                    //println!("send chunk:\n{:?}\n", chunk_tail);
                    tx.send(chunk_tail.clone()).await.unwrap();
                }
                break;
            }
            chunk = chunk_tail.clone() + &String::from_utf8(buf.to_vec()).unwrap();
            chunk_tail.clear();
            match chunk.rfind(&pat) {
                Some(mut pos) => {
                    //将最后一个待替换字符串之后的内容存到下次
                    pos = pos + pat.len();
                    let (first, last) = chunk.split_at_mut(pos);
                    chunk_tail = last.to_string();
                    chunk = first.to_string();
                    //改变buf大小，下次少读取一些
                    buf.resize(buf.capacity() - pat.len(), 0x0);
                    buf.clear();
                }
                None => {
                    //若本次读取的buffer内没有待替换的字符串,则直接写入
                    //println!("send chunk:\n{:?}\n", chunk);
                    tx.send(chunk.clone()).await.unwrap();
                    buf.clear();
                    continue;
                }
            }
            {
                let mut rng = rand::thread_rng();
                let mut rand: u8;
                'outer: loop {
                    rand = rng.gen_range(1..=100);
                    for rep in rep_str.iter() {
                        if rand >= rep.w_l || rand <= rep.w_r {
                            let chunk_tmp: String = chunk.replacen(&pat, &rep.s, 1);
                            if chunk == chunk_tmp {
                                break 'outer;
                            } else {
                                chunk = chunk_tmp;
                            }
                        }
                    }
                }
            }
            //println!("send chunk:\n{:?}\n", chunk);
            tx.send(chunk.clone()).await.unwrap();
        }
    });

    let write = tokio::spawn(async move {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let f_name: String = ("r_".to_string() + &now.to_string()).into();
        let mut w_file = File::create(f_name.clone()).await.unwrap();
        while let Some(chunk) = rx.recv().await {
            //println!("recv chunk:\n{:?}\n", chunk);
            w_file.write(chunk.as_bytes()).await.unwrap();
        }
        f_tx.send(f_name).unwrap();
    });

    read.await.unwrap();
    match f_rx.await {
        Ok(v) => println!("替换成功，新文件为: {:?}", v),
        Err(_) => println!("替换失败"),
    }
    write.await.unwrap();
}
