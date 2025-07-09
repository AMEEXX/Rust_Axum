use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        println!("Task A: start");
        sleep(Duration::from_secs(2)).await;
        println!("Task A: end");
    });

    tokio::spawn(async {
        println!("Task B: start");
        sleep(Duration::from_secs(1)).await;
        println!("Task B: end");
    });

    sleep(Duration::from_secs(3)).await;
}
