use rfd::{MessageDialog, MessageLevel};

#[macro_export]
macro_rules! option_chain_fields {
    ($base:expr, $($field:ident).+) => {{
        let mut temp = Some(&$base);
        $(
            temp = match temp {
                Some(val) => val.$field.as_ref(),
                None => None,
            };
        )+
        temp
    }};
}

pub fn show_message_dialog (title: &str, description: &str , message_level: MessageLevel) {
    MessageDialog::new()
        .set_level(message_level)
        .set_title(title)
        .set_description(description)
        .show();
}