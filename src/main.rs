mod commands;

use dotenv;
use log;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use std::env;

fn log_command(command: &ApplicationCommandInteraction) {
    let guild_id = match command.guild_id {
        Some(GuildId(id)) => id.to_string(),
        None => "Direct Message".to_string(),
    };
    let author_tag = match &command.member {
        Some(member) => member.user.tag(),
        None => "Unknown".to_string(),
    };
    let command_name = &command.data.name;
    let options_string = command
        .data
        .options
        .iter()
        .map(|option| match &option.resolved {
            Some(ApplicationCommandInteractionDataOptionValue::User(user, _)) => {
                format!("{}: User(id={}, tag={})", option.name, user.id, user.tag())
            }
            Some(ApplicationCommandInteractionDataOptionValue::Channel(channel)) => {
                format!(
                    "{}: Channel(id={}, name={})",
                    option.name,
                    channel.id,
                    channel
                        .name
                        .to_owned()
                        .unwrap_or_else(|| "Unknown".to_string())
                )
            }
            Some(ApplicationCommandInteractionDataOptionValue::Role(role)) => {
                format!("{}: Role(id={}, name={})", option.name, role.id, role.name)
            }
            Some(ApplicationCommandInteractionDataOptionValue::Attachment(attatchment)) => {
                format!(
                    "{}: Attatchment(filename={}, url={}, content_type={})",
                    option.name,
                    attatchment.filename,
                    attatchment.url,
                    attatchment
                        .content_type
                        .to_owned()
                        .unwrap_or_else(|| "Unknown".to_string())
                )
            }
            Some(value) => {
                format!("{}: {:?}", option.name, value)
            }
            _ => format!("{}: Unknown", option.name),
        })
        .collect::<Vec<String>>()
        .join(", ");
    log::info!(
        "[{}] {}: /{} {}",
        guild_id,
        author_tag,
        command_name,
        options_string
    );
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            log_command(&command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::handle(&command).await,
                "id" => commands::id::handle(&command).await,
                "attachmentinput" => commands::attachmentinput::handle(&command).await,
                _ => "Command not implemented".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                log::error!("Cannot respond to slash command: {}", why);
            }
        } else {
            log::error!("Received an interaction that is not an application command");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(commands::ping::create)
                .create_application_command(commands::id::create)
                .create_application_command(commands::welcome::create)
                .create_application_command(commands::numberinput::create)
                .create_application_command(commands::attachmentinput::create)
        })
        .await;

        log::info!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );

        let guild_command =
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("wonderful_command")
                    .description("An amazing command")
            })
            .await;

        log::info!(
            "I created the following global slash command: {:#?}",
            guild_command
        );
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    log::info!("Starting bot...");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    log::info!("Discord token: {}", token);
    let mut client = Client::builder(&token, GatewayIntents::from_bits_truncate(2147483648))
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
