use axum::Extension;
use node_monitor::{
    data::{filfox::update::miner_info_updater, history::db::init_history_db},
    router,
};

#[static_init::dynamic]
static STATIC_HANDLER: () = {
    dotenv::dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }

    tracing_subscriber::fmt::init();
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    loop {
        if let Err(e) = run().await {
            tracing::error!("mail server error: {}", e.to_string())
        }
    }
}

async fn run() -> anyhow::Result<()> {
    let port = std::env::var("PORT")
        .expect("Port must be given!")
        .parse::<u16>()
        .expect("Port must be a number!");
    let address = std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, port));
    tracing::info!("Running on: {}", address);

    let app = router::init_router().await?;

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
