use crate::logging;
use crate::types::{Window, Windows};
use crate::backend::i3::json::Node;

/// This enum represents the layout type of a node in a window manager's tree structure.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Layout {
    AllVisible,
    OneVisible,
    Skipped,
    Invalid,
}

/// Returns a collection of visible nodes in the provided JSON node.
/// This function traverses the node structure and collects nodes that are not invisible
/// or end nodes.
pub fn visible_nodes<'a>(node: &'a Node) -> Vec<&'a Node> {
    logging::debug!("V Node iterated {}", node.to_string());
    if node.is_leaf() {
        if node.is_invisible() {
            return vec![];
        }
        return vec![node];
    }

    let layout = get_layout(node);
    match layout {
        Layout::AllVisible => {
            let mut nodes: Vec<&'a Node> = vec![];
            nodes.extend(node.floating_nodes.iter());
            node.nodes.iter().for_each(|subnode| {
                nodes.extend(visible_nodes(subnode));
            });
            return nodes;
        }
        Layout::OneVisible => {
            let mut nodes: Vec<&'a Node> = vec![];
            if let Some(focused_node) = focused_subnode(node) {
                nodes.extend(visible_nodes(focused_node));
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

/// Finds deepest focused tabbed container in the provided node, and then for each tab,
/// returns the deepest focused node within that tab, this is to preserve the
/// focused state within tabs of a tabbed layout.
pub fn available_tabs(node: &Node) -> Vec<&Node> {
    if let Some(subnode) = find_deepest_focused_tabbed(node) {
        return subnode.nodes.iter().map(|tab| find_deepest_focused(tab).unwrap_or(tab)).collect();
    }
    logging::info!("No available tabs found in the provided node.");
    vec![]
}

/// Converts a JSON node to a `Windows`.
pub fn to_windows(nodes: Vec<&Node>) -> Windows {
    nodes.into_iter()
        .filter(|node| !node.is_invisible())
        .map(Window::from)
        .collect()
}

/// Returns the subnode that is currently focused, if any. If the node is an end node or has no
/// focus, it returns `None`.
/// Requires an array "focus" field and a "nodes" field containing subnodes.
fn focused_subnode(node: &Node) -> Option<&Node> {
    if node.is_leaf() || node.focus.is_empty() {
        return None;
    }
    let focus_id = node.focus.first().unwrap_or(&0);
    node.nodes
        .iter()
        .find(|n| n.id == *focus_id)
}

/// Determines the layout type of a node based on its properties.
/// Requires a "layout" field and a "type" field in the node.
fn get_layout(node: &Node) -> Layout {
    if node.is_content() ||
        ["stacked", "tabbed"].contains(&node.layout.as_str())  {
        Layout::OneVisible
    } else if ["splith", "splitv", "output"].contains(&node.layout.as_str()) ||
            ["workspace", "root", "con"].contains(&node.type_.as_str()) {
        Layout::AllVisible
    } else if node.layout == "dockarea" {
        Layout::Skipped
    } else {
        Layout::Invalid
    }
}

/// Finds the deepest focused node in a tree structure, starting from the given node.
fn find_deepest_focused(node: &Node) -> Option<&Node> {
    logging::debug!("F Node iterated {}", node.to_string());
    let subnode = focused_subnode(node);
    if subnode.is_some() {
        let deepest = find_deepest_focused(subnode?);
        if deepest.is_some() {
            return deepest;
        }
    }
    subnode
}

/// Finds the deepest focused node that is tabbed, meaning it has a layout of `tabbed` or
/// `stacked`.
fn find_deepest_focused_tabbed(node: &Node) -> Option<&Node> {
    logging::debug!("T Node iterated {}", node.to_string());
    if let Some(subnode) = focused_subnode(node) {
        let endnode = find_deepest_focused_tabbed(subnode);
        if endnode.is_some() {
            return endnode;
        } else if get_layout(node) == Layout::OneVisible {
            return Some(node);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Tests for visible nodes extraction.
    /// We expect the function to return all nodes that are not invisible.
    #[test]
    fn test_visible_nodes() {
        let node = json!({
            "id": 1,
            "type": "con",
            "layout": "splith",
            "nodes": [
                {
                    "id": 2,
                    "type": "con",
                    "rect": {"width": 100, "height": 100},
                    "focused": true,
                    "nodes": []
                },
                {
                    "id": 3,
                    "type": "con",
                    "rect": {"width": 0, "height": 0},
                    "focused": false,
                    "nodes": []
                }
            ],
            "rect": {"width": 100, "height": 100}
        });
        let nodes = visible_nodes(&node);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0]["id"], 2);

        let node = json!({
            "id": 0,
            "type": "root",
            "layout": "splith",
            "nodes": [
                {
                    "id": 1,
                    "type": "output",
                    "layout": "output",
                    "nodes": [
                        {
                            "id": 2,
                            "type": "con",
                            "layout": "splith",
                            "name": "content",
                            "nodes": [
                                {
                                    "id": 3,
                                    "type": "workspace",
                                    "layout": "splith",
                                    "nodes": [
                                        {
                                            "id": 4,
                                            "type": "con",
                                            "layout": "splith",
                                            "nodes": [
                                                {
                                                    "id": 5,
                                                    "focused": false,
                                                    "nodes": [],
                                                    "type": "con",
                                                    "focus": [],
                                                    "rect": {"width": 100, "height": 100}
                                                },
                                                {
                                                    "id": 6,
                                                    "focused": true,
                                                    "nodes": [],
                                                    "type": "con",
                                                    "focus": [],
                                                    "rect": {"width": 100, "height": 100}
                                                }
                                            ],
                                            "focus": [6, 5],
                                            "focused": false,
                                            "rect": {"width": 100, "height": 100}
                                        }
                                    ],
                                    "focus": [4],
                                    "focused": false,
                                    "rect": {"width": 100, "height": 100}
                                }
                            ],
                            "focus": [3],
                            "focused": false,
                            "rect": {"width": 100, "height": 100}
                        }
                    ],
                    "focus": [2],
                    "focused": false,
                    "rect": {"width": 100, "height": 100}
                },
                {
                    "id": 7,
                    "type": "output",
                    "layout": "output",
                    "nodes": [
                        {
                            "id": 8,
                            "type": "con",
                            "layout": "splith",
                            "name": "content",
                            "nodes": [
                                {
                                    "id": 12,
                                    "type": "workspace",
                                    "layout": "splith",
                                    "nodes": [
                                        {
                                            "id": 13,
                                            "focused": false,
                                            "nodes": [],
                                            "type": "con",
                                            "focus": [],
                                            "rect": {"width": 100, "height": 100}
                                        }
                                    ],
                                    "focus": [13],
                                    "focused": false,
                                    "rect": {"width": 100, "height": 100}
                                },
                                {
                                    "id": 9,
                                    "type": "workspace",
                                    "layout": "splith",
                                    "nodes": [
                                        {
                                            "id": 10,
                                            "type": "con",
                                            "layout": "tabbed",
                                            "nodes": [
                                                {
                                                    "id": 14,
                                                    "focused": false,
                                                    "nodes": [],
                                                    "type": "con",
                                                    "focus": [],
                                                    "rect": {"width": 100, "height": 100}
                                                },
                                                {
                                                    "id": 11,
                                                    "focused": false,
                                                    "nodes": [],
                                                    "type": "con",
                                                    "focus": [],
                                                    "rect": {"width": 100, "height": 100}
                                                }
                                            ],
                                            "focus": [11, 14],
                                            "focused": false,
                                        }
                                    ],
                                    "floating_nodes": [
                                        {
                                            "id": 20,
                                            "focused": false,
                                            "nodes": [],
                                            "type": "con",
                                            "focus": [],
                                            "rect": {"width": 100, "height": 100}
                                        }
                                    ],
                                    "focus": [10, 20],
                                    "focused": false,
                                    "rect": {"width": 100, "height": 100}
                                }
                            ],
                            "focus": [9, 12],
                            "focused": false,
                            "rect": {"width": 100, "height": 100}
                        }
                    ],
                    "focus": [8],
                    "focused": false,
                    "rect": {"width": 100, "height": 100}
                }
            ],
            "focus": [1, 7],
            "focused": false,
            "rect": {"width": 100, "height": 100}
        });
        let nodes = visible_nodes(&node);
        assert!(nodes.iter().any(|n| n["id"] == 5));
        assert!(nodes.iter().any(|n| n["id"] == 6));
        assert!(nodes.iter().any(|n| n["id"] == 11));
        assert!(nodes.iter().any(|n| n["id"] == 20));
        assert_eq!(nodes.len(), 4);
    }

    /// Tests for extracting available tabs from a node.
    /// We expect the function to return a vector of leaf nodes that are focused of a tabbed
    /// layout, or none if there are no tabs.
    #[test]
    fn test_available_tabs() {
        let node = json!({
            "id": 1,
            "type": "con",
            "layout": "splith",
            "nodes": [
                {
                    "id": 2,
                    "type": "con",
                    "layout": "tabbed",
                    "nodes": [
                        {"id": 3, "focused": true, "nodes": [], "type": "con", "focus": []},
                        {
                            "id": 4,
                            "type": "con",
                            "layout": "splith",
                            "nodes": [
                                {"id": 5, "focused": true, "nodes": [], "type": "con", "focus": []},
                                {"id": 6, "focused": false, "nodes": [], "type": "con", "focus": []}
                            ],
                            "focus": [5],
                            "focused": true,
                            "rect": {"width": 100, "height": 100}
                        }
                    ],
                    "focus": [3],
                    "focused": true,
                    "rect": {"width": 100, "height": 100}
                }
            ],
            "focus": [2],
            "focused": true,
            "rect": {"width": 100, "height": 100}
        });
        let tabs = available_tabs(&node);
        assert_eq!(tabs.len(), 2);
        assert!(tabs.iter().any(|tab| tab["id"] == 3));
        assert!(tabs.iter().any(|tab| tab["id"] == 5));
    }

    /// Tests for converting JSON nodes to windows.
    #[test]
    fn test_to_windows() {
        let nodes = vec![
            json!({"id": 1, "rect": {"x": 0, "y": 0, "width": 100, "height": 100}, "focused": true, "type": "con", "nodes": []}),
            json!({"id": 2, "rect": {"x": 300, "y": 450, "width": 15, "height": 200}, "focused": false, "type": "con", "nodes": []}),
        ];
        let node_refs: Vec<&Node> = nodes.iter().collect();
        let windows = to_windows(node_refs);
        assert_eq!(windows.len(), 2);
        assert_eq!(windows[0].id, 1);
        assert!(windows[0].focused);
        assert!(!windows[0].floating);
        assert_eq!(windows[0].rect, Rect { x: 0, y: 0, w: 100, h: 100 });
        assert_eq!(windows[1].id, 2);
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
        let node = json!({
            "layout": "splith",
            "name": "content",
            "type": "con",
            "nodes": [
                {"id": 1, "type": "workspace", "layout": "splith", "nodes": []},
            ],
        });
        assert_eq!(get_layout(&node), Layout::OneVisible);

        let node = json!({
            "layout": "splith",
            "type": "workspace"
        });
        assert_eq!(get_layout(&node), Layout::AllVisible);

        let node = json!({
            "layout": "splith",
            "type": "con"
        });
        assert_eq!(get_layout(&node), Layout::AllVisible);

        let node = json!({
            "layout": "splitv",
            "type": "con"
        });
        assert_eq!(get_layout(&node), Layout::AllVisible);

        let node = json!({
            "layout": "tabbed",
            "type": "con"
        });
        assert_eq!(get_layout(&node), Layout::OneVisible);

        let node = json!({
            "layout": "stacked",
            "type": "con"
        });
        assert_eq!(get_layout(&node), Layout::OneVisible);

        let node = json!({
            "layout": "dockarea",
            "type": "con"
        });
        assert_eq!(get_layout(&node), Layout::Skipped);

        let node = json!({
            "layout": "invalid",
            "type": "con"
        });
        assert_eq!(get_layout(&node), Layout::Invalid);
    }

    /// Tests for invisible node detection.
    /// We expect the function to return true for nodes that have a rectangle with zero width and
    /// height.
    #[test]
    fn test_is_invisible_node() {
        let node = json!({
            "rect": {"width": 0, "height": 0}
        });
        assert!(is_invisible_node(&node));

        let node = json!({
            "rect": {"width": 100, "height": 100}
        });
        assert!(!is_invisible_node(&node));
    }

    /// Tests for end node detection.
    /// We expect the function to return true for nodes that have no subnodes and are a container.
    #[test]
    fn test_is_end_node() {
        let node = json!({
            "nodes": [],
            "type": "con"
        });
        assert!(is_end_node(&node));

        let node = json!({
            "nodes": [{"id": 1}],
            "type": "con"
        });
        assert!(!is_end_node(&node));

        let node = json!({
            "nodes": [],
            "type": "workspace"
        });
        assert!(!is_end_node(&node));
    }

    /// Tests for focused subnode extraction.
    /// We expect the function to return the node that is indicated based on the "focus" field.
    #[test]
    fn test_focused_subnode() {
        let node = json!({
            "nodes": [
                {"id": 1, "focused": true, "nodes": []},
                {"id": 2, "focused": false, "nodes": []}
            ],
            "focus": [1]
        });
        assert_eq!(focused_subnode(&node).unwrap()["id"].as_u64().unwrap(), 1);

        let node = json!({
            "nodes": [],
            "focus": []
        });
        assert!(focused_subnode(&node).is_none());
    }

    /// Tests for finding the deepest focused node.
    /// We expect the function to traverse the tree and find the deepest node that is focused.
    #[test]
    fn test_find_deepest_focused() {
        let node = json!({
            "nodes": [
                {"id": 1, "focused": true, "nodes": []},
                {"id": 2, "focused": false, "nodes": []}
            ],
            "focus": [1]
        });
        assert_eq!(find_deepest_focused(&node).unwrap()["id"], 1);

        let node = json!({
            "nodes": [],
            "focus": []
        });
        assert!(find_deepest_focused(&node).is_none());
    }

    /// Tests for finding the deepest focused tabbed node.
    /// We expect the function to find the deepest node that is focused and has focusable tabs.
    #[test]
    fn test_find_deepest_focused_tabbed() {
        // Test with a focused tabbed node.
        // Tabs require elements and focus to be recognized as switchable.
        let node = json!({
            "nodes": [
                {"id": 1, "layout": "tabbed", "focused": true, "nodes": [
                        {"id": 3, "focused": true, "nodes": [], "focus": []},
                        {"id": 4, "focused": false, "nodes": [], "focus": []}
                    ], "focus": [3]},
                {"id": 2, "layout": "tabbed", "focused": false, "nodes": [], "focus": []}
            ],
            "focus": [1, 3]
        });
        assert_eq!(find_deepest_focused_tabbed(&node).unwrap()["id"], 1);

        // Test with a focused tabbed node that has no subnodes
        // This should return None since it has no tabs.
        let node = json!({
            "nodes": [
                {"id": 1, "layout": "tabbed", "focused": true, "nodes": [], "focus": []}
            ],
            "focus": [1]
        });
        assert_eq!(find_deepest_focused_tabbed(&node), None);

        // Test with no focused tabbed node
        // This should return None since there are no tabs.
        let node = json!({
            "nodes": [],
            "focus": []
        });
        assert!(find_deepest_focused_tabbed(&node).is_none());
    }

}
