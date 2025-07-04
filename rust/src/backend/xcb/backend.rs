use super::client::Client;
use crate::types::Windows;
use crate::backend::traits::*;
use xcb::Xid;
use xcb::x::Window as XWindow;
use std::collections::HashMap;
use crate::logging;

pub struct Backend {
    client: Client,
    windows: Windows,
    xid_map: HashMap<u64, XWindow>,
}

impl Backend {
    pub fn new() -> Self {
        // Initialize the X client
        let client = Client::new();

        // Verify required atoms
        client.verify_required_atoms()
            .expect("Window manager does not support required atoms");

        // Get the active window
        let active_window = client.get_active_window()
            .expect("Failed to get active window");

        // Get the list of windows
        let xwindows = client.get_client_list();

        // Get the full window properties for each window
        let mut xid_map: HashMap<u64, XWindow> = HashMap::new();
        let mut windows = xwindows.into_iter()
            .filter_map(|xwindow| {
                // Fetch window info and add to the windows vector
                let window = match client.fetch_window_info(&xwindow) {
                    Ok(info) => info,
                    Err(e) => {
                        logging::error!("Failed to fetch window info for {}: {}", xwindow.resource_id(), e);
                        return None;
                    }
                };
                xid_map.insert(xwindow.resource_id().into(), xwindow);
                Some(window)
            })
            .collect::<Windows>();

        // Find the focused window
        windows.iter_mut().for_each(|window| {
            if xid_map[&window.id] == active_window {
                window.focused = true;
            }
            logging::debug!("Window ID: {}, Rect: {}, Floating: {}, Focused: {}",
                     window.id, window.rect.to_string(), window.floating, window.focused);
        });

        Backend { client, windows, xid_map }
    }
}

impl GetVisible for Backend {
    fn get_visible(&self) -> Result<Windows, String> {
        Ok(self.windows.clone())
    }
}

impl GetTabs for Backend {
    fn get_tabs(&self) -> Result<Windows, String> {
        Err("Not implemented".to_string())
    }
}

impl SetFocus for Backend {
    fn set_focus(&mut self, window_id: &u64) {
        // Check if the window ID exists in the map
        if !self.xid_map.contains_key(window_id) {
            logging::critical!("Window ID {} does not exist", window_id);
        }
        // Set focus to the specified window
        self.client.set_focus(self.xid_map[window_id])
            .expect("Failed to set focus");
    }
}
