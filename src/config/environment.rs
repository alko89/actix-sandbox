use env_logger::Env;
use once_cell::sync::Lazy;

pub struct Environment {
    pub mongodb_uri: String,
    pub mongodb_database: String,
}

pub static ENV: Lazy<Environment> = Lazy::new(|| {
    dotenv::dotenv().ok();
    // std::env::set_var(
    //     "RUST_LOG",
    //     "simple-auth-server=debug,actix_web=info,actix_server=info",
    // );
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let mongodb_database = std::env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE must be set");

    Environment {
        mongodb_uri,
        mongodb_database,
    }
});
