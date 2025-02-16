use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};

pub async fn failed(ctx: &Context, interaction: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new()
        .content("Could not delete profile")
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Message(message);
    if let Err(why) = interaction
        .create_response(&ctx.http, initial_response)
        .await
    {
        println!("Cannot respond to slash command: {why}");
    }
}

pub async fn success(profile_name: String, ctx: &Context, interaction: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new()
        .content(format!("Deleted profile \"{}\"", profile_name))
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Message(message);
    if let Err(why) = interaction
        .create_response(&ctx.http, initial_response)
        .await
    {
        println!("Cannot respond to slash command: {why}");
    }
}
