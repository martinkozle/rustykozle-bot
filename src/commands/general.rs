use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Return "Hello, {author_name}!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("Hello, {}!", ctx.author().name)).await?;
    Ok(())
}

/// Display your or another user's account creation date
#[poise::command(slash_command, track_edits)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "User"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    ctx.say(format!(
        "{}'s account was created at {}",
        user.name,
        user.created_at()
    ))
    .await?;

    Ok(())
}

/// Information about a server voice channel
#[poise::command(slash_command)]
pub async fn voiceinfo(
    ctx: Context<'_>,
    #[description = "Voice channel"]
    #[channel_types("Voice")]
    channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let response = format!(
        "\
**Name**: {}
**Bitrate**: {}
**User limit**: {}
**RTC region**: {}
**Video quality mode**: {:?}",
        channel.name,
        channel.bitrate.unwrap_or_default(),
        channel.user_limit.unwrap_or_default(),
        channel.rtc_region.unwrap_or_default(),
        channel
            .video_quality_mode
            .unwrap_or(serenity::VideoQualityMode::Unknown)
    );

    ctx.say(response).await?;
    Ok(())
}
