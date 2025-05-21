use crate::telemetry::publisher::test_publisher;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    println!("Starting RabbitMQ test publisher");
    test_publisher().await?;
    println!("Test completed successfully");
    Ok(())
}