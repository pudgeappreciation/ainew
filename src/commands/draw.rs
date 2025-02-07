use serenity::all::MessageBuilder;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

#[derive(Debug, Default)]
struct DrawRequest {
    prompt: String,
    negative_prompt: String,
}

impl<'a> Into<DrawRequest> for &[ResolvedOption<'a>] {
    fn into(self) -> DrawRequest {
        let mut request = DrawRequest::default();

        for option in self.iter() {
            match option {
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "prompt",
                    ..
                } => request.prompt = value.to_string(),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "negative_prompt",
                    ..
                } => request.negative_prompt = value.to_string(),
                _ => {}
            }
        }

        request
    }
}

pub fn run(options: &[ResolvedOption]) -> String {
    println!("{options:?}");

    let request: DrawRequest = options.into();

    MessageBuilder::new()
        .push_bold_line_safe("Prompt:")
        .push_codeblock_safe(request.prompt, None)
        .push_bold_line("Negative prompt:")
        .push_codeblock_safe(request.negative_prompt, None)
        .build()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("draw")
        .description("draw an image")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "prompt", "The prompt to use")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "negative_prompt",
                "The negative_prompt to use",
            )
            .required(false),
        )
}
