use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt, EventMask};
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::ClientMessageEvent;

use libwmctl::prelude::{windows, active, State};
use crate::backend::traits::*;
use crate::types::{Rect, Window, Windows};

pub struct Backend {
    windows: Windows,
    visibility: Vec<bool>,
}

impl Backend {
    pub fn new() -> Self {
        let show_hidden = false;
        let wm_windows = windows(show_hidden)
            .expect("Failed to connect to the window manager");

        let wm_focused = active().id;

        let mut visibility = Vec::with_capacity(wm_windows.len());
        let windows = wm_windows.iter()
            .inspect(|w| {
                visibility.push(is_visible(&w.state()
                    .expect("Failed to get window state")));
            })
            .map(|w| {
                let wm_win_geometry = w.geometry()
                    .expect("Failed to get window geometry");
                let wm_win_states = w.state()
                    .expect("Failed to get window state");
                let focused = w.id == wm_focused;
                let floating = is_floating(&wm_win_states);
                Window {
                    id: w.id as u64,
                    rect: Rect {
                        x: wm_win_geometry.0,
                        y: wm_win_geometry.1,
                        w: wm_win_geometry.2 as i32,
                        h: wm_win_geometry.3 as i32,
                    },
                    focused: focused,
                    floating: floating,
                }
            })
            .collect::<Windows>();

        Self {
            windows: windows,
            visibility: visibility,
        }
    }
}

impl GetTabs for Backend {
    fn get_tabs(&self) -> Result<Windows, String> {
        Err("Tabs not supported in this backend".to_string())
    }
}

impl GetVisible for Backend {
    fn get_visible(&self) -> Result<Windows, String> {
        Ok(self.windows.iter()
            .enumerate()
            .filter_map(|(i, window)| {
                if self.visibility[i] {
                    Some(window.clone())
                } else {
                    None
                }
            })
            .collect())
    }
}

impl SetFocus for Backend {
    fn set_focus(&mut self, window_id: &u64) {
        // Connect to the X server
        let (conn, screen_num) = RustConnection::connect(None)
            .expect("Failed to connect to the X server");
        let screen = &conn.setup().roots[screen_num];

        // Get the atom for _NET_ACTIVE_WINDOW
        let atom_name = b"_NET_ACTIVE_WINDOW";
        let net_active_window = conn.intern_atom(false, atom_name)
            .expect("Failed to intern atom")
            .reply()
            .expect("Failed to get atom reply")
            .atom;

        // Get the atom for _NET_WM_WINDOW_TYPE_NORMAL if needed (not strictly necessary for focusing)
        // let net_wm_window_type_normal = conn.intern_atom(false, b"_NET_WM_WINDOW_TYPE_NORMAL")?.reply()?.atom;

        // Construct and send the client message event
        let event = ClientMessageEvent {
            response_type: 33, // CLIENT_MESSAGE
            format: 32,
            sequence: 0,
            window: *window_id as u32,
            type_: net_active_window,
            data: x11rb::protocol::xproto::ClientMessageData::from([
                1, // source indication (1 = application)
                x11rb::CURRENT_TIME,
                0,
                0,
                0,
            ]),
        };

        conn.send_event(
            false,
            screen.root,
            EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
            event,
        ).expect("Failed to send event");
        conn.flush()
            .expect("Failed to flush connection");
    }
}

fn is_tiled(states: &Vec<State>) -> bool {
    states.iter().any(|state| matches!(state, State::MaxHorz | State::MaxVert))
}

fn is_floating(states: &Vec<State>) -> bool {
    !is_tiled(states)
}

fn is_visible(states: &Vec<State>) -> bool {
    !states.iter().any(|state| matches!(state, State::Hidden))
}
