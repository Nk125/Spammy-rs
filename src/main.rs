use ntex::web;
use spammyrs::{discord::client, env, rest};

async fn start_client() {
    let discord_client = client::Client::from_bot_token(
        &std::env::var(env::identifier::DISCORD_TOKEN_STR_ENV).unwrap(),
    )
    .await
    .expect("Client should be able to be constructed at this point");

    let bot_profile = discord_client
        .get_app_info()
        .await
        .expect("App info must be retrievable");

    log::info!(
        "Logged in as: {} ({}), app name: {}",
        bot_profile["bot"]["username"]
            .as_str()
            .expect("Bot profile must have username field"),
        bot_profile["id"]
            .as_str()
            .expect("Bot profile must have id field"),
        bot_profile["name"]
            .as_str()
            .expect("App object must have a name field")
    );

    log::info!(
        "Bot owner: {} ({})",
        bot_profile["owner"]["username"]
            .as_str()
            .expect("Owner profile must have username field"),
        bot_profile["owner"]["id"]
            .as_str()
            .expect("Owner profile must have id field")
    );

    let cmd_name = std::env::var(env::identifier::COMMAND_NAME_STR_ENV).unwrap();

    discord_client
        .submit_command(&cmd_name, "Some fun tho :)")
        .await
        .expect("Command must be registereable");

    log::info!("Overwritten all commands with: /{}", &cmd_name);
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if !env::check::all_vars_defined() {
        panic!("Some variables are missing in the environment, closing...");
    }

    let msg = env::load_msg::load_message();

    log::info!("Message to show up: {}", msg);

    let listen_port = std::env::var(env::identifier::LISTEN_PORT_STR_ENV)
        .unwrap()
        .parse::<u16>()
        .map_err(|e| log::error!("{:?}", e))
        .and_then(|port| match port {
            1..=65535 => Ok(port),
            _ => Err(()),
        })
        .expect("Listening port must be in the range of 1..65535");

    start_client().await;

    log::info!("Server listen port configured at: {}", listen_port);

    web::HttpServer::new(|| {
        web::App::new()
            .service(
                web::resource("/interactions")
                    .route(web::post().to(rest::discord::endpoint::interactions)),
            )
            .wrap(web::middleware::Compress::default())
            .wrap(web::middleware::Logger::new(
                r#"%a "%r" %s %b %Dms "%{User-Agent}i" "%{Content-Type}i" "%{Content-Type}o""#,
            ))
    })
    .bind(("0.0.0.0", listen_port))?
    .run()
    .await
}
