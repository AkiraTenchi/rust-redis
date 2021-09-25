use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open connection to Redis server
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set 'hello' key to value 'world'
    client.set("hello", "world".into()).await?;

    // Get the value for key 'hello'
    let result = client.get("hello").await?;

    println!(
        "value for key 'hello' has been retrieved from server; result={:?}",
        result
    );

    Ok(())
}
