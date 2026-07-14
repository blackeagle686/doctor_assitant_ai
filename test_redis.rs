use redis::Client;
async fn test(client: &Client) {
    let _ = client.get_multiplexed_async_connection().await;
    let _ = client.get_connection();
}
