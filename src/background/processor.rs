use tokio::sync::broadcast::Receiver;

pub async fn init_processor(mut rx: Receiver<String>) {
    loop {
        let r = rx.recv().await;
        match r {
            Ok(r) => {
                println!("BG Thread Recv: {}",r);
            },
            Err(e) => {
                println!("Error: {}",e);
            }
        }
    }
}