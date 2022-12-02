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
            .nest(
                "/inner",
                Router::new().route(
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
