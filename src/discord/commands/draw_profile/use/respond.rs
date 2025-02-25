use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};

pub async fn cleared_active_profile(ctx: &Context, interaction: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new()
        .content("Cleared active profile")
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Message(message);
    if let Err(why) = interaction
        .create_response(&ctx.http, initial_response)
        .await
    {
        println!("Cannot respond to slash command: {why}");
    }
}

pub async fn set_profile(profile_name: &str, ctx: &Context, interaction: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new()
        .content(format!("Set profile \"{}\" as active", profile_name))
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Message(message);
    if let Err(why) = interaction
        .create_response(&ctx.http, initial_response)
        .await
    {
        println!("Cannot respond to slash command: {why}");
    }
}
