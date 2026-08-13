use askama::Template;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Template, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[template(path = "talk_prompt.askama.md")]
pub struct TalkPrompt {}

impl TalkPrompt {}
