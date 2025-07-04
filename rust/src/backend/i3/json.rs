use crate::types::Rect;
use crate::types::Window;

use serde;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Node {
    pub id: i64,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub layout: String,
    pub nodes: Vec<Node>,
    pub floating_nodes: Vec<Node>,
    pub rect: Rect,
    pub focus: Vec<i64>,
    pub focused: bool,
}

impl Node {
    pub fn is_leaf(&self) -> bool {
        self.nodes.is_empty() &&
            (self.type_ == "con" || self.type_ == "floating_con")
    }

    pub fn is_content(&self) -> bool {
        self.type_ == "con" &&
            self.name.is_some() &&
            self.name.as_ref().unwrap() == "content" &&
            !self.is_leaf()
    }

    pub fn is_invisible(&self) -> bool {
        self.rect.w == 0 &&
            self.rect.h == 0
    }
}

impl From<&Node> for Window {
    fn from(node: &Node) -> Self {
        let id = node.id as u64;
        let rect = node.rect.clone();
        let floating = node.type_ == "floating_con";
        let focused = node.focused;

        Window { id, rect, focused, floating, }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        let result = format!("Node id={} type={} layout={}",
            self.id, self.type_, self.layout);
        result
    }
}
