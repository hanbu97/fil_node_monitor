use axum::{
    routing::{on, MethodFilter},
    Router,
};

use crate::apis;

pub async fn init_router() -> anyhow::Result<Router> {
    use http::Method;
    use tower_http::cors::{Any, CorsLayer};

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/history",
                    Router::new().route("/",
                    on(
                        MethodFilter::GET,
                        apis::history::get::get_history,
                    ).on(
                        MethodFilter::POST,
                        apis::history::get::get_history,
                    )).nest(
                        "/subscribe",
                        Router::new()
                            .route(
                                "/add",
                                on(
                                    MethodFilter::POST,
                                    apis::history::subscribe::add::post_history_subscribe_add,
                                ),
                            )
                            .route(
                                "/delete",
                                on(
                                    MethodFilter::POST,
                                    apis::history::subscribe::delete::post_history_subscribe_delete,
                                ),
                            )
                            .route(
                                "/",
                                on(
                                    MethodFilter::GET,
                                    apis::history::subscribe::get_history_subscribe,
                                ),
                            ),
                    ),
                )
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
                            "/interval",
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
        )
        .layer(cors);

    Ok(app)
}
