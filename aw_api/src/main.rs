use aw_api::settings::get_app_mode;
use aw_api::server::Application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_setting = get_app_mode().expect("Failed to read configuration file");
    let app = Application::build_app(app_setting).await?;

    app.run_app().await?;

    Ok(())
}
