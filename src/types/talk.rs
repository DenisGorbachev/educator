use crate::{Definition, Slide};
use derive_more::{From, Into};
use derive_new::new;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(new, Serialize, Deserialize, JsonSchema, From, Into, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Talk {
    pub topic: String,
    pub slides: Vec<Slide>,
    pub definitions: Vec<Definition>,
}

impl Talk {
    pub fn prompt(_topic: &str) -> String {
        #[rustfmt::skip]
        let a = concat!(
            "You may use scientific and mathematical terms, but you need to introduce them first.",
            "A term introduction is a non-empty sequence of motivating examples where each motivating example uses either regular words or already introduced terms.");
        a.to_string()
    }
}
