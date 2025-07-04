use crate::types::Rect;
use crate::types::Window;
use crate::logging;

use serde;
use serde::Deserialize;

/// This enum represents the layout type of a node in a window manager's tree structure.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Layout {
    AllVisible,
    OneVisible,
    Skipped,
    Invalid,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: u64,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub layout: String,
    pub nodes: Vec<Node>,
    pub floating_nodes: Vec<Node>,
    pub rect: Rect,
    pub focus: Vec<u64>,
    pub focused: bool,
}

impl Node {

    // --------------
    // Public methods
    // --------------

    /// Finds all available tabs in the node tree, relevant for current focus.
    /// Returns a vector of most recently focused nodes for each tab.
    pub fn available_tabs(&self) -> Vec<&Node> {
        if let Some(subnode) = self.find_deepest_focused_tabbed() {
            return subnode.nodes.iter().map(|tab| tab.find_deepest_focused().unwrap_or(tab)).collect();
        }
        logging::info!("No available tabs found in the provided node.");
        vec![]
    }

    /// Finds all visible nodes in the node tree.
    /// Nodes are considered visible if they are on a visible workspace, not unfocused tab,
    /// and have a non-zero rectangle size.
    pub fn visible_nodes<'a>(&'a self) -> Vec<&'a Node> {
        logging::debug!("V Iterated {}", self.to_string());
        if self.is_leaf() {
            if self.is_invisible() {
                return vec![];
            }
            return vec![self];
        }

        let layout = self.get_layout();
        match layout {
            Layout::AllVisible => {
                let mut nodes: Vec<&'a Node> = vec![];
                nodes.extend(self.floating_nodes.iter());
                self.nodes.iter().for_each(|subnode| {
                    nodes.extend(subnode.visible_nodes());
                });
                return nodes;
            }
            Layout::OneVisible => {
                let mut nodes: Vec<&'a Node> = vec![];
                if let Some(focused_node) = self.focused_subnode() {
                    nodes.extend(focused_node.visible_nodes());
                }
                return nodes;
            }
            Layout::Skipped => vec![],
            Layout::Invalid => {
                logging::error!("Invalid layout encountered: {:?}", layout);
                return vec![]
            }
        }
    }

    // ---------------
    // Private methods
    // ---------------

    /// Checks if the node is a leaf node, meaning it has no subnodes and is a container.
    fn is_leaf(&self) -> bool {
        self.nodes.is_empty() &&
            (self.type_ == "con" || self.type_ == "floating_con")
    }

    /// Checks if the node is a content node, which is a special type of node containing
    /// workspaces.
    fn is_content(&self) -> bool {
        self.type_ == "con" &&
            self.name.is_some() &&
            self.name.as_ref().unwrap() == "content" &&
            !self.is_leaf()
    }

    /// Checks if the node is invisible, meaning it has a rectangle with zero width and height.
    fn is_invisible(&self) -> bool {
        self.rect.w == 0 &&
            self.rect.h == 0
    }

    /// Checks if the node is floating, meaning it is a floating container or has any floating
    /// subnodes that are floating.
    fn is_floating(&self) -> bool {
        self.type_ == "floating_con" ||
            self.floating_nodes.iter().any(|n| n.is_floating())
    }

    /// Returns whether the node is a tabbed layout that has multiple subnodes.
    fn is_switchable_tabbed(&self) -> bool {
        self.get_layout() == Layout::OneVisible &&
            !self.is_content() &&
            self.nodes.len() > 1
    }

    /// Returns the focused subnode of the current node, if it exists.
    /// Focused subnodes are determined by the most recent focus ID in the `focus` vector.
    fn focused_subnode(&self) -> Option<&Node> {
        if self.is_leaf() || self.focus.is_empty() {
            return None;
        }
        let focus_id = self.focus.first().unwrap_or(&0);
        self.nodes
            .iter()
            .find(|n| n.id == *focus_id)
    }

    /// Finds the deepest focused tabbed node in the tree that has multiple subnodes.
    fn find_deepest_focused_tabbed(&self) -> Option<&Node> {
        logging::debug!("T Iterated {}", self.to_string());
        let subnode = self.focused_subnode()?;
        match subnode.find_deepest_focused_tabbed() {
            Some(tabnode) => Some(tabnode),
            None if self.is_switchable_tabbed() => Some(self),
            None => None
        }
    }

    /// Finds the deepest focused node in the tree.
    fn find_deepest_focused(&self) -> Option<&Node> {
        logging::debug!("F Iterated {}", self.to_string());
        let subnode = self.focused_subnode()?;
        match subnode.find_deepest_focused() {
            Some(deepest) => Some(deepest),
            None => Some(subnode)
        }
    }

    /// Returns the layout type of the node based on its type and layout fields.
    fn get_layout(&self) -> Layout {
        if self.is_content() ||
            ["stacked", "tabbed"].contains(&self.layout.as_str())  {
            Layout::OneVisible
        } else if ["splith", "splitv", "output"].contains(&self.layout.as_str()) &&
                ["workspace", "root", "output", "con"].contains(&self.type_.as_str()) {
            Layout::AllVisible
        } else if self.layout == "dockarea" {
            Layout::Skipped
        } else {
            Layout::Invalid
        }
    }
}

impl From<&Node> for Window {
    fn from(node: &Node) -> Self {
        let id = node.id as u64;
        let rect = node.rect.clone();
        let floating = node.is_floating();
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

// -----
// Tests
// -----

#[cfg(test)]
mod tests {
    use super::*;

    fn read_json(file: &str) -> Node {
        let content = std::fs::read_to_string(file)
            .expect("Failed to read JSON file");
        serde_json::from_str::<Node>(content.as_str())
            .expect("Failed to parse JSON to Node")
    }

    /// Tests for visible nodes extraction.
    /// We expect the function to return all nodes that are not invisible.
    #[test]
    fn test_visible_nodes() {
        let root: Node = read_json("jsons/2node_splith.json");
        let nodes = root.visible_nodes();
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].id, 2);

        let node: Node = read_json("jsons/root_with_several_nodes.json");
        let nodes = node.visible_nodes();
        assert!(nodes.iter().any(|n| n.id == 5));
        assert!(nodes.iter().any(|n| n.id == 6));
        assert!(nodes.iter().any(|n| n.id == 11));
        assert!(nodes.iter().any(|n| n.id == 20));
        assert_eq!(nodes.len(), 4);
    }

    /// Tests for extracting available tabs from a node.
    /// We expect the function to return a vector of leaf nodes that are focused of a tabbed
    /// layout, or none if there are no tabs.
    #[test]
    fn test_available_tabs() {
        let node: Node = read_json("jsons/tabs_with_deep_focus.json");
        let tabs = node.available_tabs();
        assert_eq!(tabs.len(), 2);
        assert!(tabs.iter().any(|tab| tab.id == 3));
        assert!(tabs.iter().any(|tab| tab.id == 5));
    }

    /// Tests for converting JSON nodes to windows.
    #[test]
    fn test_to_windows() {
        let node: Node = read_json("jsons/2node_splith.json");
        let nodes: Vec<&Node> = node.nodes.iter().collect();
        let windows = nodes.iter().map(|n| Window::from(*n)).collect::<Vec<Window>>();
        assert_eq!(windows.len(), 2);
        assert_eq!(windows[0].id, 2);
        assert!(windows[0].focused);
        assert!(!windows[0].floating);
        assert_eq!(windows[0].rect, Rect { x: 0, y: 0, w: 100, h: 100 });
        assert_eq!(windows[1].id, 3);
        assert!(!windows[1].focused);
        assert!(!windows[1].floating);
        assert_eq!(windows[1].rect, Rect { x: 300, y: 450, w: 15, h: 200 });
    }

    /// Tests for layout extraction.
    /// We expect the function to return the correct layout type based on the "layout" and "type"
    /// fields.
    /// Workspaces are switchable,
    /// Split layouts are directional,
    /// Tabbed layouts are switchable,
    /// Stacked layouts are switchable,
    /// Dock areas are opaque to the layout system,
    /// Invalid layouts are marked as invalid.
    #[test]
    fn test_get_layout() {
        let mut node: Node = read_json("jsons/empty_node.json");
        node.name = Some("content".to_string());
        node.type_ = "con".to_string();
        node.layout = "splith".to_string();
        node.nodes.push(node.clone());
        assert_eq!(node.get_layout(), Layout::OneVisible);

        let mut node: Node = read_json("jsons/empty_node.json");
        node.type_ = "workspace".to_string();
        node.layout = "splith".to_string();
        assert_eq!(node.get_layout(), Layout::AllVisible);

        node.type_ = "con".to_string();
        node.layout = "splith".to_string();
        assert_eq!(node.get_layout(), Layout::AllVisible);

        node.type_ = "con".to_string();
        node.layout = "splitv".to_string();
        assert_eq!(node.get_layout(), Layout::AllVisible);

        node.type_ = "con".to_string();
        node.layout = "tabbed".to_string();
        assert_eq!(node.get_layout(), Layout::OneVisible);

        node.type_ = "con".to_string();
        node.layout = "stacked".to_string();
        assert_eq!(node.get_layout(), Layout::OneVisible);

        node.type_ = "dockarea".to_string();
        node.layout = "dockarea".to_string();
        assert_eq!(node.get_layout(), Layout::Skipped);

        node.type_ = "con".to_string();
        node.layout = "invalid".to_string();
        assert_eq!(node.get_layout(), Layout::Invalid);
    }

    /// Tests for invisible node detection.
    /// We expect the function to return true for nodes that have a rectangle with zero width and
    /// height.
    #[test]
    fn test_is_invisible_node() {
        let mut node: Node = read_json("jsons/empty_node.json");
        node.rect.w = 0;
        node.rect.h = 0;
        assert!(node.is_invisible());

        node.rect.w = 100;
        node.rect.h = 100;
        assert!(!node.is_invisible());
    }

    /// Tests for end node detection.
    /// We expect the function to return true for nodes that have no subnodes and are a container.
    #[test]
    fn test_is_end_node() {
        let node: Node = read_json("jsons/empty_node.json");
        assert!(node.is_leaf());

        let node: Node = read_json("jsons/2node_splith.json");
        assert!(!node.is_leaf());

        let node: Node = read_json("jsons/empty_workspace.json");
        assert!(!node.is_leaf());
    }

    /// Tests for focused subnode extraction.
    /// We expect the function to return the node that is indicated based on the "focus" field.
    #[test]
    fn test_focused_subnode() {
        let node: Node = read_json("jsons/2node_splith.json");
        assert_eq!(node.focused_subnode().unwrap().id, 2);

        let node: Node = read_json("jsons/empty_node.json");
        assert!(node.focused_subnode().is_none());
    }

    /// Tests for finding the deepest focused node.
    /// We expect the function to traverse the tree and find the deepest node that is focused.
    #[test]
    fn test_find_deepest_focused() {
        let node: Node = read_json("jsons/2node_splith.json");
        assert_eq!(node.find_deepest_focused().unwrap().id, 2);

        let node: Node = read_json("jsons/empty_node.json");
        assert!(node.find_deepest_focused().is_none());
    }

    /// Tests for finding the deepest focused tabbed node.
    /// We expect the function to find the deepest node that is focused and has focusable tabs.
    #[test]
    fn test_find_deepest_focused_tabbed() {
        // Test with a focused tabbed node.
        // Tabs require elements and focus to be recognized as switchable.
        let node: Node = read_json("jsons/ambigous_tabs.json");
        assert_eq!(node.find_deepest_focused_tabbed().unwrap().id, 1);

        // Test with a focused tabbed node that has no subnodes
        // This should return None since it has no tabs.
        let node: Node = read_json("jsons/empty_tabs.json");
        assert_eq!(node.find_deepest_focused_tabbed(), None);

        // Test with no focused tabbed node
        // This should return None since there are no tabs.
        let node: Node = read_json("jsons/empty_node.json");
        assert!(node.find_deepest_focused_tabbed().is_none());
    }

}
