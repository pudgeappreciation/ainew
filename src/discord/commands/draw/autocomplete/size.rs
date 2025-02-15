use serenity::all::CreateAutocompleteResponse;

pub fn handle() -> CreateAutocompleteResponse {
    CreateAutocompleteResponse::new()
        .add_string_choice("sm-sm", "600x600")
        .add_string_choice("sm-md", "600x840")
        .add_string_choice("md-sm", "840x600")
        .add_string_choice("md-md", "840x840")
        .add_string_choice("md-lg", "840x1080")
        .add_string_choice("lg-md", "1080x840")
        .add_string_choice("lg-lg", "1080x1080")
        .add_string_choice("lg-xl", "1080x1320")
        .add_string_choice("xl-lg", "1320x1080")
        .add_string_choice("xl-xl", "1320x1320")
}
