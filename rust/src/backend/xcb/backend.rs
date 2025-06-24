use super::client::Client;
use crate::types::{Window, Windows};
use crate::backend::traits::*;
use xcb::Xid;
use xcb::XidNew;
use xcb::x::Window as XWindow;

struct Backend {
    client: Client,
    windows: Windows,
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
        let rects = client.get_windows_rects(&x_windows);

        // Get the list of properties for the windows
        let properties = client.get_windows_properties(&x_windows);

        // Create the Windows structure
        let windows = x_windows.iter()
            .zip(rects.iter())
            .zip(properties.iter())
            .map(|((x_window, rect), properties)| {
                Window {
                    id: x_window.resource_id().into(),
                    rect: rect.clone(),
                    floating: client.is_floating(&properties),
                    focused: *x_window == active_window,
                }
            })
            .collect();

        Backend { client, windows }
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
        // Set focus to the specified window
        unsafe {
            self.client.set_focus(XWindow::new(*window_id as u32))
                .expect("Failed to set focus");
        }
    }
}
