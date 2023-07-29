use crate::routes::__path_ping;
use crate::routes::ping;

use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::settings::{Settings, DatabaseSettings};

pub fn get_connection_pool(
    configuration: &DatabaseSettings
) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_lazy_with(configuration.database_connection_string())
}

pub struct Application {
    port: u16,
    server: Server
}

impl Application {
    pub async fn build_app(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = start(listener, connection_pool)?;

        Ok(Self { port, server })
    }

    pub async fn run_app(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn start(
    listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    #[derive(OpenApi)]
    #[openapi(
        paths(ping),
        info(
            title = "Actix-web RESTful",
            version = "0.1.0",
            description = "My First Rust Backend with _`Actix-web`_",
            license(name = "MIT", url = "github.com"),
            contact(name = "Dev Activity", url = "github.com", email = "dev@activity.com")
        )
    )]
    struct ApiDoc;

    let db_pool_data = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/apidoc/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .app_data(db_pool_data.clone())
            .route("/ping", web::get().to(ping))
    })
    .listen(listener)?
    .run();

    Ok(server)
}