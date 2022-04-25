use serenity::{model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
}, builder::CreateApplicationCommand};


pub fn create(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
    .name("attachmentinput")
    .description("Test command for attachment input")
    .create_option(|option| {
        option
            .name("attachment")
            .description("A file")
            .kind(ApplicationCommandOptionType::Attachment)
            .required(true)
    })
}


pub async fn handle(command: &ApplicationCommandInteraction) -> String {
    let options = command
        .data
        .options
        .get(0)
        .expect("Expected attachment option")
        .resolved
        .as_ref()
        .expect("Expected attachment object");

    if let ApplicationCommandInteractionDataOptionValue::Attachment(attachment) = options {
        format!(
            "Attachment name: {}, attachment size: {}",
            attachment.filename, attachment.size
        )
    } else {
        "Please provide a valid attachment".to_string()
    }
}
