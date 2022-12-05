use axum::{
    routing::{on, MethodFilter},
    Router,
};

use crate::apis;

pub async fn init_router() -> anyhow::Result<Router> {
    let app = Router::new().nest(
        "/api",
        Router::new()
            .nest(
                "/subscribe",
                Router::new()
                    .route("/", on(MethodFilter::GET, apis::subscribe::get_subscribe))
                    .route(
                        "/add",
                        on(MethodFilter::POST, apis::subscribe::add::post_subscribe_add),
                    )
                    .route(
                        "/delete",
                        on(
                            MethodFilter::POST,
                            apis::subscribe::delete::post_subscribe_delete,
                        ),
                    ),
            )
            .route("/info", on(MethodFilter::GET, apis::info::get_info))
            .nest(
                "/inner",
                Router::new()
                    .route(
                        "interval",
                        on(MethodFilter::GET, apis::inner::interval::get_interval)
                            .on(MethodFilter::POST, apis::inner::interval::post_interval),
                    )
                    .route(
                        "/version",
                        on(
                            MethodFilter::GET,
                            han_utils::apis::inner::version::get_version,
                        ),
                    ),
            ),
    );

    Ok(app)
}
