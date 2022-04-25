use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
        ApplicationCommandOptionType,
    },
};

pub fn create(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("id")
        .description("Get a user id")
        .create_option(|option| {
            option
                .name("id")
                .description("The user to lookup")
                .kind(ApplicationCommandOptionType::User)
                .required(true)
        })
}

pub async fn handle(command: &ApplicationCommandInteraction) -> String {
    let user_option = command
        .data
        .options
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) = user_option {
        format!("{}'s id is {}", user.tag(), user.id)
    } else {
        "Please provide a valid user".to_string()
    }
}
