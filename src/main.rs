mod db;
mod errors;
mod models;

use actix_web;
use deadpool_postgres;
use dotenv;
use env_logger;

#[actix_web::post("/register")]
async fn register_user(
    actix_web::web::Form(form): actix_web::web::Form<crate::models::UserForm>,
    pool: actix_web::web::Data<deadpool_postgres::Pool>,
) -> actix_web::Result<impl actix_web::Responder> {
    let p = pool.get().await.map_err(crate::errors::Errors::PoolError)?;
    match crate::db::register_user(&p, &form.username, &form.password).await {
        Ok(_) => Ok(actix_web::HttpResponse::Created().finish()),
        Err(e) => Err(e.into()),
    }
}

#[actix_web::post("/login")]
async fn login_user(
    actix_web::web::Form(form): actix_web::web::Form<crate::models::UserForm>,
    pool: actix_web::web::Data<deadpool_postgres::Pool>,
) -> actix_web::Result<impl actix_web::Responder> {
    let p = pool.get().await.map_err(crate::errors::Errors::PoolError)?;
    match crate::db::login_user(&p, &form.username, &form.password).await {
        Ok(_) => Ok(actix_web::HttpResponse::Accepted().finish()),
        Err(e) => Err(e.into()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::builder()
        .filter(None, log::LevelFilter::Debug)
        .default_format()
        .format_indent(Some(4))
        .format_target(true)
        .format_timestamp_secs()
        .target(env_logger::Target::Stdout)
        .init();
    let mut config = tokio_postgres::Config::new();
    config.dbname(&std::env::var("DB_NAME").expect("No DB_NAME enviroment variable found"));
    config.host(&std::env::var("DB_HOST").unwrap());
    config.user(&std::env::var("DB_USER").unwrap());
    config.password(&std::env::var("DB_PASS").unwrap());
    config.port(std::env::var("DB_PORT").unwrap().parse::<u16>().unwrap());
    let mgr = deadpool_postgres::Manager::from_config(
        config,
        tokio_postgres::NoTls,
        deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        },
    );
    let pool = deadpool_postgres::Pool::builder(mgr)
        .max_size(8)
        .build()
        .unwrap();
    let _ = pool
        .get()
        .await
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS users(username TEXT, hashed_password TEXT)",
            &[],
        )
        .await
        .unwrap();
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(register_user)
            .service(login_user)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind("localhost:3020")
    .unwrap()
    .run();
    log::info!("http://localhost:3020");
    server.await
}
