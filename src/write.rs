use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::{mpsc, oneshot};

pub async fn write_chunk(mut rx: mpsc::Receiver<String>, f_tx: oneshot::Sender<String>) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let f_name: String = ("r_".to_string() + &now.to_string()).into();
    let mut w_file = File::create(f_name.clone()).await.unwrap();
    while let Some(chunk) = rx.recv().await {
        w_file.write(chunk.as_bytes()).await.unwrap();
    }
    f_tx.send(f_name).unwrap();
}
