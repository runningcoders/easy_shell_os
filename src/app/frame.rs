use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Frame {
    id: usize,
    name: String,
    root: Value,
    style: Value,
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            r#"
Frame: {}_{}
elements: {}
style: {}
"#,
            self.name, self.id, self.root, self.style
        ))
    }
}
