use clap::Parser;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio::sync::Semaphore;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 't', long)]
    target: String,

    #[arg(short, long, default_value_t = 1)]
    start_port: u16,

    #[arg(short, long, default_value_t = 1024)]
    end_port: u16,

    #[arg(short = 'T', long, default_value_t = 500)]
    timeout_ms: u64,

    #[arg(short = 'c', long, default_value_t = 100)]
    concurrency: usize,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let semaphore = Arc::new(Semaphore::new(args.concurrency));
    let mut handles = vec![];

    for port in args.start_port..=args.end_port {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let addr = format!("{}:{}", args.target, port);
        let duration = Duration::from_millis(args.timeout_ms);

        let handle = tokio::spawn(async move {
            let _permit = permit;
            let conn_result = timeout(duration, TcpStream::connect(&addr)).await;

            match conn_result {
                Ok(Ok(_stream)) => {
                    println!("Port {} is open", port);
                }
                Ok(Err(_e)) => {

                }
                Err(_) => {

                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}
