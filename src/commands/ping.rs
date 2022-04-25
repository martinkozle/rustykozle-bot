use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::application_command::{
        ApplicationCommandInteraction,
    },
};

pub fn create(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}

pub async fn handle(_command: &ApplicationCommandInteraction) -> String {
    "Hey, I'm alive!".to_string()
}
