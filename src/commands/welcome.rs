use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::application_command::{
        ApplicationCommandOptionType,
    },
};

pub fn create(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("welcome")
        .description("Welcome a user")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to welcome")
                .kind(ApplicationCommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("message")
                .description("The message to send")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
                .add_string_choice(
                    "Welcome to our cool server! Ask me if you need help",
                    "pizza",
                )
                .add_string_choice("Hey, do you want a coffee?", "coffee")
                .add_string_choice(
                    "Welcome to the club, you're now a good person. Well, I hope.",
                    "club",
                )
                .add_string_choice(
                    "I hope that you brought a controller to play together!",
                    "game",
                )
        })
}

// pub async fn handle(command: &ApplicationCommandInteraction) -> String {
//     unimplemented!();
// }
