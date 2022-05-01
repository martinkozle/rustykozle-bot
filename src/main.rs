mod commands;
use commands::*;
use dotenv;
use log;
use poise::serenity_prelude as serenity;
use std::env;

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn log_command(ctx: Context<'_>) {
    let guild_name = match ctx.guild() {
        Some(guild) => guild.name,
        None => "Direct message".to_string(),
    };
    let author_tag = ctx.author().tag();
    let command_name = ctx.command().name;
    let args = ctx
        .command()
        .parameters
        .iter()
        .map(|p| p.name)
        .collect::<Vec<_>>()
        .join(" ");

    log::info!(
        "[{}] {}: /{} {}",
        guild_name,
        author_tag,
        command_name,
        args
    );
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

/// Registers application commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(prefix_command, slash_command, hide_in_help)]
async fn register(
    ctx: Context<'_>,
    #[flag]
    #[description = "Register application commands globally"]
    global: bool,
) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, global).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    log::info!("Starting bot...");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    log::info!("Discord token: {}", token);

    poise::Framework::build()
        .token(token)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }))
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("r!".into()),
                ..Default::default()
            },
            commands: vec![
                register(),
                general::hello(),
                general::age(),
                general::voiceinfo(),
            ],
            on_error: |error| Box::pin(on_error(error)),
            pre_command: |ctx| Box::pin(async move { log_command(ctx).await }),
            ..Default::default()
        })
        .run()
        .await
        .unwrap();
}
