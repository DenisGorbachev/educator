use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Slide {
    pub title: String,

    #[schemars(description = "Graphical elements that are displayed on the screen")]
    pub contents: Vec<String>,

    #[schemars(description = "Sentences that the speaker says while the slide is being displayed.")]
    pub speech: Vec<String>,
}
