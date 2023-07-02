use std::borrow::Cow;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Builder, Serialize, Deserialize)]
pub struct AppliedItem<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub amount: i32,
}
