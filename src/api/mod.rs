pub mod routes;

pub async fn start_server() -> anyhow::Result<()> {
    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("API Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
