use super::client::Client;
use crate::types::{Window, Windows};
use crate::backend::traits::*;
use xcb::Xid;
use xcb::x::Window as XWindow;
use std::collections::HashMap;

pub struct Backend {
    client: Client,
    windows: Windows,
    xwindows: HashMap<u64, XWindow>,
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
        let x_windows = client.get_client_list();

        // Get the list of windows
        let rects = client.get_normalized_windows_rects(&x_windows);

        // Get the list of properties for the windows
        let properties = client.get_windows_properties(&x_windows);

        // Get window withdrawn states
        let withdrawns = client.get_windows_withdrawn(&x_windows);

        // Create a map to hold the XWindow to u64 resource ID mapping
        let mut xwindows: HashMap<u64, XWindow> = HashMap::new();

        // Create the Windows structure
        let windows: Windows = x_windows.iter()
            .zip(rects.iter())
            .zip(properties.iter())
            .zip(withdrawns.iter())
            .filter_map(|(((x_window, rect), properties), withdrawn)| {
                println!("Properties are: {:?}", properties);
                match client.is_visible(properties) && !withdrawn {
                    false => return None, // Skip invisible windows
                    true => {
                        let window_id: u64 = x_window.resource_id().into();
                        xwindows.insert(window_id.clone(), *x_window);
                        Some(Window {
                            id: window_id,
                            rect: *rect,
                            floating: client.is_floating(&properties),
                            focused: *x_window == active_window,
                        })
                    }
                }
            })
            .collect();

        // Log the windows
        for window in &windows {
            println!("Window ID: {}, Rect: {}, Floating: {}, Focused: {}",
                     window.id, window.rect.to_string(), window.floating, window.focused);
        }

        Backend { client, windows, xwindows }
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
        if !self.xwindows.contains_key(window_id) {
            eprintln!("Window ID {} does not exist", window_id);
            return;
        }
        // Set focus to the specified window
        self.client.set_focus(self.xwindows[window_id])
            .expect("Failed to set focus");
    }
}
