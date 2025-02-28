use serenity::all::MessageBuilder;

pub trait OptionalNameValuePair {
    fn append_to_builder(&self, name: &str, builder: &mut MessageBuilder);
}

impl<T> OptionalNameValuePair for Option<T>
where
    T: ToString,
{
    fn append_to_builder(&self, name: &str, builder: &mut MessageBuilder) {
        match self {
            Some(value) => builder
                .push_bold_safe(name)
                .push_line_safe(value.to_string()),
            None => builder.push_bold_safe(name).push_line_safe("None"),
        };
    }
}

pub trait AddOptionalNameValuePair<T>
where
    T: OptionalNameValuePair,
{
    fn append_optional_name_value(&mut self, name: &str, value: &T) -> &mut Self;
}

impl<T> AddOptionalNameValuePair<T> for MessageBuilder
where
    T: OptionalNameValuePair,
{
    fn append_optional_name_value(&mut self, name: &str, value: &T) -> &mut Self {
        value.append_to_builder(name, self);

        self
    }
}
