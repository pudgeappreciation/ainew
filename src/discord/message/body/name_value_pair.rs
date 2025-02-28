use serenity::all::MessageBuilder;

pub trait AddNameValuePair<T>
where
    T: ToString,
{
    fn append_name_value(&mut self, name: &str, value: &T) -> &mut Self;
}

impl<T> AddNameValuePair<T> for MessageBuilder
where
    T: ToString,
{
    fn append_name_value(&mut self, name: &str, value: &T) -> &mut Self {
        self.push_bold_safe(name).push_line_safe(value.to_string())
    }
}
