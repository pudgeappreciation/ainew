pub trait CommandOption {
    fn append_to_string(&self, name: &str, string: &mut String);
}

impl<T> CommandOption for Option<T>
where
    T: ToString,
{
    fn append_to_string(&self, name: &str, string: &mut String) {
        if let Some(value) = self {
            string.push_str("\n");
            string.push_str(name);
            string.push_str(":");
            string.push_str(&value.to_string());
        }
    }
}

pub trait AddCommandOption<T>
where
    T: CommandOption,
{
    fn append_command_option(&mut self, name: &str, value: &T) -> &mut Self;
}

impl<T> AddCommandOption<T> for String
where
    T: CommandOption,
{
    fn append_command_option(&mut self, name: &str, value: &T) -> &mut Self {
        value.append_to_string(name, self);

        self
    }
}
