#[async_std::main]
async fn main() {
    say_hello().await;
}

async fn say_hello() {
    println!("Hello, world!");
}
