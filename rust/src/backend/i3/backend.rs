use crate::backend::traits::*;
use crate::logging::ResultExt;
use crate::logging;
use crate::types::Windows;
use super::client::{Client, Request};
use super::compass;
use super::json::Node;

use serde_json as json;
use std::process;

pub struct Backend {
    client: Client,
    root: Node,
}

impl Backend {
    pub fn new() -> Self {
        // Establish a connection to the i3 IPC server and get the tree structure
        let i3_socket_path_output = process::Command::new("i3").arg("--get-socketpath").output()
            .expect_log("Failed to get i3 socket path");
        let i3_path = String::from_utf8(i3_socket_path_output.stdout)
            .expect_log("Failed to parse i3 socket path output");
        let mut client = Client::new(&i3_path.trim())
            .expect_log("Failed to connect to i3 IPC server");
        let root_string = client.request(Request::GetTree, "")
            .expect_log("Failed to get i3 tree JSON");

        // Parse the i3 tree to get the current workspace and window information
        let root_value = json::from_str::<json::Value>(&root_string)
            .expect_log("Failed to parse i3 tree JSON");
        let root: Node = json::from_value(root_value)
            .expect_log("Failed to convert i3 tree JSON to Node");
        Self {
            client,
            root,
        }
    }
}

impl GetTabs for Backend {
    fn get_tabs(&self) -> Result<Windows, String> {
        let nodes = compass::available_tabs(&self.root);
        Ok(compass::to_windows(nodes))
    }
}

impl GetVisible for Backend {
    fn get_visible(&self) -> Result<Windows, String> {
        let nodes = compass::visible_nodes(&self.root);
        Ok(compass::to_windows(nodes))
    }
}

impl SetFocus for Backend {
    fn set_focus(& mut self, window_id: &u64) {
        // Focus the window with the determined ID
        logging::info!("Focusing window with ID: {}", window_id);
        let payload = format!("[con_id={}] focus", window_id);
        self.client.request(Request::Command, &payload)
            .expect_log("Failed to send focus command");
    }
}
