use crate::backend::traits::*;
use crate::logging::ResultExt;
use crate::logging::OptionExt;
use crate::logging;
use crate::types::Windows;
use super::client::{Client, Request};
use super::compass;

use serde_json as json;
use std::process;

pub struct Backend {
    client: Client,
    root: json::Value,
}

fn get_sock_path(executable: &str) -> Option<String> {
    process::Command::new(executable).arg("--get-socketpath").output()
        .ok().filter(|o| o.status.success()).and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_owned()).filter(|s| !s.is_empty())
}

impl Backend {
    pub fn new() -> Self {
        // Establish a connection to the i3 IPC server and get the tree structure
        let socket_path = get_sock_path("i3").or_else(|| get_sock_path("sway"))
            .wanted("Failed to get socket path from i3 or sway").unwrap_or_default();
        let mut client = Client::new(&socket_path.trim())
            .expect_log("Failed to connect to i3 IPC server");
        let root_string = client.request(Request::GetTree, "")
            .expect_log("Failed to get i3 tree JSON");

        // Parse the i3 tree to get the current workspace and window information
        let root = json::from_str::<json::Value>(&root_string)
            .expect_log("Failed to parse i3 tree JSON");
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
