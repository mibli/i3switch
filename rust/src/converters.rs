/// This file provides functions to convert JSON nodes representing windows in a window manager's
/// tree structure. It includes functions to extract visible nodes, available tabs, floating and
/// tiled windows, and to convert these nodes into a structured format for further processing.
/// It also includes functions to determine the layout of nodes and to find focused windows in
/// the tree structure.
///
/// This module is part of a window manager's arrangement system, which allows for
/// manipulating and querying the layout of windows in a graphical user interface.
// TODO: Use references instead of cloning values where possible.

use serde_json::Value;

use crate::linear;
use crate::planar;
use crate::planar::Rect;
use crate::logging;
use crate::logging::OptionExt;

/// This enum represents a window in a window manager's tree structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub id: u64,
    pub rect: Rect,
    pub focused: bool,
    pub floating: bool,
}

/// This enum represents the layout type of a node in a window manager's tree structure.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Layout {
    AllVisible,
    OneVisible,
    Skipped,
    Invalid,
}

/// A collection of windows, represented as a vector of `Window` structs.
pub type Windows = Vec<Window>;

/// Returns a collection of visible nodes in the provided JSON node.
/// This function traverses the node structure and collects nodes that are not invisible
/// or end nodes.
pub fn visible_nodes<'a>(node: &'a Value) -> Vec<&'a Value> {
    logging::debug!("V Node iterated id:{} type:{} layout:{}, name:{}", 
        node.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
        node.get("type").and_then(|v| v.as_str()).unwrap_or("null"),
        node.get("layout").and_then(|v| v.as_str()).unwrap_or("null"),
        node.get("name").and_then(|v| v.as_str()).unwrap_or("null"));
    if is_end_node(node) {
        if is_invisible_node(node) {
            return vec![];
        }
        return vec![node];
    }

    let layout = get_layout(node);
    match layout {
        Layout::AllVisible => {
            let mut nodes: Vec<&'a Value> = vec![];
            if let Some(floating_nodes) = node.get("floating_nodes").unwrap_or(&Value::Null).as_array() {
                nodes.extend(floating_nodes.iter());
            }
            node.get("nodes").unwrap().as_array().unwrap().iter().for_each(|subnode| {
                nodes.extend(visible_nodes(subnode));
            });
            return nodes;
        }
        Layout::OneVisible => {
            let mut nodes: Vec<&'a Value> = vec![];
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
pub fn available_tabs(node: &Value) -> Vec<&Value> {
    if let Some(subnode) = find_deepest_focused_tabbed(node) {
        let tabs_node = subnode.get("nodes").unwrap_or(&Value::Null);
        if let Some(tabs_node) = tabs_node.as_array() {
            return tabs_node.iter().map(|tab| find_deepest_focused(tab).unwrap_or(tab)).collect();
        }
    }
    logging::info!("No available tabs found in the provided node.");
    vec![]
}

/// Returns a collection of windows that are floating, i.e., those that are not tiled.
pub fn floating(windows: &Windows) -> Windows {
    windows.iter()
        .filter(|w| w.floating)
        .cloned()
        .collect()
}

/// Returns a collection of windows that are not floating, i.e., those that are tiled.
pub fn tiled(windows: &Windows) -> Windows {
    windows.iter()
        .filter(|w| !w.floating)
        .cloned()
        .collect()
}

/// Returns whether any window in the provided `Windows` is focused.
pub fn any_focused(windows: &Windows) -> bool {
    windows.iter().any(|w| w.focused)
}

/// Converts a JSON node to a `Windows`.
pub fn to_windows(nodes: Vec<&Value>) -> Windows {
    nodes.into_iter()
        .filter(|node| !is_invisible_node(node))
        .map(to_window)
        .collect()
}

/// Converts a JSON node to a `planar::Arrangement`.
/// This function assumes that the node represents a workspace or root node
/// and contains a list of windows.
pub fn as_arrangement(windows: &Windows, relation: planar::Relation) -> planar::Arrangement {
    let current = focused_index(windows).unwrap_or(0);
    let windows: Vec<planar::Window> = windows.iter().map(to_planar).collect();
    planar::Arrangement::new(windows, Some(current), Some(relation))
}

/// Converts a collection of `Windows` to a `linear::Sequence`.
/// This function creates a sequence of window IDs and marks the focused window by its index.
pub fn as_sequence(windows: &Windows) -> linear::Sequence {
    let focused = focused_index(windows).unwrap_or(0);
    return linear::Sequence::new(windows.iter().map(|w| w.id).collect(), focused);
}

/// Converts a JSON node to a `Window`.
/// This function extracts the window's ID, rectangle dimensions, and focus state
/// from the JSON structure.
fn to_window(node: &Value) -> Window {
    let id = node["id"].as_u64().unwrap_or(0);
    let rect = Rect {
        x: node["rect"]["x"].as_i64().unwrap_or(0) as i32,
        y: node["rect"]["y"].as_i64().unwrap_or(0) as i32,
        w: node["rect"]["width"].as_i64().unwrap_or(0) as i32,
        h: node["rect"]["height"].as_i64().unwrap_or(0) as i32,
    };
    let floating = node["type"].as_str().map_or(false, |t| t == "floating_con");
    let focused: bool;
    if floating {
        focused = node["nodes"].get(0).and_then(|n| n["focused"].as_bool()).unwrap_or(false);
    } else {
        focused = node["focused"].as_bool().unwrap_or(false);
    };

    Window { id, rect, focused, floating, }
}

/// Converts a `Window` to a `planar::Window`.
fn to_planar(window: &Window) -> planar::Window {
    planar::Window {
        id: window.id,
        rect: window.rect.clone(),
    }
}

/// Returns the index of the currently focused window, if any.
fn focused_index(windows: &Windows) -> Option<usize> {
    windows.iter().position(|w| w.focused).wanted(
        format!("No focused window found in windows: {:?}", windows).as_str())
}

/// Checks if a node is an invisible node, which is defined as having a rectangle with zero width
/// and height.
fn is_invisible_node(node: &Value) -> bool {
    node["rect"]["width"].as_i64().unwrap_or(0) == 0
        && node["rect"]["height"].as_i64().unwrap_or(0) == 0
}

/// Checks if a node is an end node, which is defined as having no subnodes and being of type
/// "con".
fn is_end_node(node: &Value) -> bool {
    let is_empty = node.get("nodes").and_then(|n| n.as_array()).unwrap_or(&vec![]).is_empty();
    let type_str = node.get("type").and_then(|t| t.as_str()).unwrap_or("");
    is_empty && ( type_str == "con" || type_str == "floating_con" )
}

/// Checks if a node is a workspace, which is defined as having a type of "workspace".
fn is_content_node(node: &Value) -> bool {
    node["name"].as_str().unwrap_or("") == "content" && !is_end_node(node)
}

/// Returns the subnode that is currently focused, if any. If the node is an end node or has no
/// focus, it returns `None`.
/// Requires an array "focus" field and a "nodes" field containing subnodes.
fn focused_subnode(node: &Value) -> Option<&Value> {
    let focus: &Value = node.get("focus").unwrap_or(&Value::Null);
    if is_end_node(node) || focus.is_null() || focus.as_array().unwrap().is_empty() {
        return None;
    }
    let focus_id = focus[0].as_u64().unwrap_or(0);
    node.get("nodes")?
        .as_array()?
        .iter()
        .find(|n| n["id"].as_u64() == Some(focus_id))
}

/// Determines the layout type of a node based on its properties.
/// Requires a "layout" field and a "type" field in the node.
fn get_layout(node: &Value) -> Layout {
    let layout = node.get("layout").and_then(|l| l.as_str()).unwrap_or("");
    let node_type = node.get("type").and_then(|t| t.as_str()).unwrap_or("");
    if is_content_node(node) || layout == "stacked" || layout == "tabbed" {
        Layout::OneVisible
    } else if layout == "splith" || layout == "splitv" || layout == "output" || node_type == "workspace" || node_type == "root" {
        Layout::AllVisible
    } else if layout == "dockarea" {
        Layout::Skipped
    } else {
        Layout::Invalid
    }
}

/// Finds the deepest focused node in a tree structure, starting from the given node.
fn find_deepest_focused(node: &Value) -> Option<&Value> {
    logging::debug!("F Node iterated id:{} type:{} layout:{}",
        node.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
        node.get("type").and_then(|v| v.as_str()).unwrap_or("null"),
        node.get("layout").and_then(|v| v.as_str()).unwrap_or("null"));
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
fn find_deepest_focused_tabbed(node: &Value) -> Option<&Value> {
    logging::debug!("T Node iterated id:{} type:{} layout:{}",
        node.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
        node.get("type").and_then(|v| v.as_str()).unwrap_or("null"),
        node.get("layout").and_then(|v| v.as_str()).unwrap_or("null"));
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
    use crate::logging;
    use serde_json::json;
    use ctor::ctor;

    #[ctor]
    fn setup() {
        logging::init();
    }

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

    /// Tests for floating and tiled windows.
    #[test]
    fn test_floating_and_tiled() {
        let windows = vec![
            Window { id: 1, rect: Rect {x: 0, y: 0, w: 100, h: 100}, focused: true, floating: false },
            Window { id: 2, rect: Rect {x: 100, y: 100, w: 200, h: 200}, focused: false, floating: true },
        ];
        let floating_windows = floating(&windows);
        let tiled_windows = tiled(&windows);
        assert_eq!(floating_windows.len(), 1);
        assert_eq!(tiled_windows.len(), 1);
    }

    /// Tests for checking if any window is focused.
    #[test]
    fn test_any_focused() {
        let windows = vec![
            Window { id: 1, rect: Rect {x: 0, y: 0, w: 100, h: 100}, focused: true, floating: false },
            Window { id: 2, rect: Rect {x: 100, y: 100, w: 200, h: 200}, focused: false, floating: true },
        ];
        assert!(any_focused(&windows));
    }

    /// Tests for converting JSON nodes to windows.
    #[test]
    fn test_to_windows() {
        let nodes = vec![
            json!({"id": 1, "rect": {"x": 0, "y": 0, "width": 100, "height": 100}, "focused": true, "type": "con", "nodes": []}),
            json!({"id": 2, "rect": {"x": 300, "y": 450, "width": 15, "height": 200}, "focused": false, "type": "con", "nodes": []}),
        ];
        let node_refs: Vec<&Value> = nodes.iter().collect();
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

    /// Tests for visible nodes extraction.
    #[test]
    fn test_as_arrangement() {
        let windows = vec![
            Window { id: 1, rect: Rect { x: 0, y: 0, w: 100, h: 100 }, focused: true, floating: false },
            Window { id: 2, rect: Rect { x: 100, y: 100, w: 200, h: 200 }, focused: false, floating: true },
        ];
        let arrangement = as_arrangement(&windows, planar::Relation::Border);
        assert_eq!(arrangement.windows.len(), 2);
        assert_eq!(arrangement.current, 0);
    }

    /// Tests for window conversion.
    #[test]
    fn test_to_planar() {
        let window = Window { id: 1, rect: Rect { x: 0, y: 0, w: 100, h: 100 }, focused: true, floating: false };
        let planar_window = to_planar(&window);
        assert_eq!(planar_window.id, 1);
        assert_eq!(planar_window.rect, Rect { x: 0, y: 0, w: 100, h: 100 });
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

    /// Tests for focused index extraction.
    /// We expect the function to return the index of the first focused window in the provided
    #[test]
    fn test_focused_index() {
        let windows = vec![
            Window { id: 1, rect: Rect { x: 0, y: 0, w: 100, h: 100 }, focused: true, floating: false },
            Window { id: 2, rect: Rect { x: 100, y: 100, w: 200, h: 200 }, focused: false, floating: true },
        ];
        assert_eq!(focused_index(&windows), Some(0));

        let windows = vec![
            Window { id: 1, rect: Rect { x: 0, y: 0, w: 100, h: 100 }, focused: false, floating: false },
            Window { id: 2, rect: Rect { x: 100, y: 100, w: 200, h: 200 }, focused: false, floating: true },
        ];
        assert_eq!(focused_index(&windows), None);
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

    /// Tests sequence conversion.
    /// We expect the function to return a sequence of window IDs and the index of the focused
    /// window.
    #[test]
    fn test_as_sequence() {
        let windows = vec![
            Window { id: 1, rect: Rect { x: 0, y: 0, w: 100, h: 100 }, focused: true, floating: false },
            Window { id: 2, rect: Rect { x: 100, y: 100, w: 200, h: 200 }, focused: false, floating: true },
        ];
        let sequence = as_sequence(&windows);
        assert_eq!(sequence[0], 1);
        assert_eq!(sequence[1], 2);
    }
}
