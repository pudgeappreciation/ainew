use serenity::all::{
    ChannelId, CreateAttachment, CreateMessage, MessageId, MessageReference, MessageReferenceKind,
};

use crate::global::draw_response::DrawResponse;

pub fn create(
    response: DrawResponse,
    channel_id: ChannelId,
    message_id: MessageId,
) -> CreateMessage {
    let message_reference =
        MessageReference::new(MessageReferenceKind::Default, channel_id).message_id(message_id);
    let message = CreateMessage::new().reference_message(message_reference);
    let message = match response.images.into_iter().next() {
        Some(image_data) => message
            .content("**Your drawing is finished! :D**")
            .add_file(CreateAttachment::bytes(image_data, "image.png")),
        None => message.content("**No image returned :/**"),
    };

    message
}
