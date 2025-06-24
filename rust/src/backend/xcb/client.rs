use xcb::{x, Connection};
use crate::types::Rect;

// Helper struct to initialize xcb atoms.
// This helps initialize the atoms taking advantage of xcb concurrency.
xcb::atoms_struct! {
    #[derive(Copy, Clone, Debug)]
    pub struct Atoms {
        pub _net_active_window           => b"_NET_ACTIVE_WINDOW",
        pub _net_client_list             => b"_NET_CLIENT_LIST",
        pub _net_supported               => b"_NET_SUPPORTED",
        pub _net_wm_state                => b"_NET_WM_STATE",
        pub _net_wm_state_hidden         => b"_NET_WM_STATE_HIDDEN",
        pub _net_wm_state_maximized_horz => b"_NET_WM_STATE_MAXIMIZED_HORZ",
        pub _net_wm_state_maximized_vert => b"_NET_WM_STATE_MAXIMIZED_VERT",
        pub wm_state                     => b"WM_STATE",
        pub wm_state_withdrawn           => b"WM_STATE_WITHDRAWN",
    }
}

pub struct Client {
    conn: Connection,
    root: x::Window,
    atoms: Atoms,
}

impl Client {
    pub fn new() -> Self {
        // Connect to the X server
        let (conn, screen_num) = Connection::connect(None)
            .expect("Failed to connect to X server");

        // Get the default screen
        // Modern setups use a single screen and distribute windows using XRandR
        let screen = conn.get_setup().roots().nth(screen_num as usize)
            .expect("Failed to get screen");

        // Get the root window of the screen
        let root = screen.root();

        // Get all required atoms
        let atoms = Atoms::intern_all(&conn)
            .expect("Failed to intern required atoms");

        Client { conn, root, atoms }
    }

    pub fn verify_required_atoms(&self) -> Result<(), String> {
        // Get the _NET_SUPPORTED property from the root window
        let cookie = self.conn.send_request(&x::GetProperty {
            delete: false,
            window: self.root,
            property: self.atoms._net_supported,
            r#type: x::ATOM_ATOM,
            long_offset: 0,
            long_length: 1024, // Number of atoms to fetch
        });
        let supported_atoms = self.conn.wait_for_reply(cookie)
            .expect("Failed to get _NET_SUPPORTED property");
        let supported_atoms = supported_atoms.value::<x::Atom>().to_vec();

        // Check if all required atoms are supported
        for atom in [
            &self.atoms._net_active_window,
            &self.atoms._net_client_list,
            &self.atoms._net_wm_state,
            &self.atoms._net_wm_state_maximized_horz,
            &self.atoms._net_wm_state_maximized_vert,
        ] {
            if !supported_atoms.contains(&atom) {
                let cookie = self.conn.send_request(&x::GetAtomName {
                    atom: *atom,
                });
                let atom_name = self.conn.wait_for_reply(cookie)
                    .map(|reply| reply.name().to_string())
                    .unwrap_or_else(|_| "unknown".to_string());
                return Err(format!("Required atom '{}' is not supported", atom_name));
            }
        }
        Ok(())
    }

    pub fn get_client_list(&self) -> Vec<x::Window> {
        // Get the list of client windows from the root window
        let cookie = self.conn.send_request(&x::GetProperty {
            delete: false,
            window: self.root,
            property: self.atoms._net_client_list,
            r#type: x::ATOM_WINDOW,
            long_offset: 0,
            long_length: 1024, // Number of windows to fetch
        });

        let reply = self.conn.wait_for_reply(cookie)
            .expect("Failed to get _NET_CLIENT_LIST property");

        reply.value::<x::Window>().to_vec()
    }

    pub fn get_active_window(&self) -> Option<x::Window> {
        // Get the active window from the root window
        let cookie = self.conn.send_request(&x::GetProperty {
            delete: false,
            window: self.root,
            property: self.atoms._net_active_window,
            r#type: x::ATOM_WINDOW,
            long_offset: 0,
            long_length: 1, // Only one active window
        });

        match self.conn.wait_for_reply(cookie) {
            Ok(reply) => {
                if reply.length() > 0 {
                    Some(reply.value::<x::Window>()[0])
                } else {
                    None
                }
            },
            Err(_) => None,
        }
    }

    pub fn get_normalized_windows_rects(&self, window_ids: &[x::Window]) -> Vec<Rect> {
        let mut rects = self.get_windows_rects(window_ids);
        // Translate rects to root window coordinates
        let mut cookies = Vec::new();
        for &window_id in window_ids {
            cookies.push(self.conn.send_request(&x::TranslateCoordinates {
                src_window: window_id,
                dst_window: self.root,
                src_x: 0,
                src_y: 0,
            }));
        }
        // Wait for all replies
        let mut i: usize = 0;
        for cookie in cookies {
            match self.conn.wait_for_reply(cookie) {
                Ok(reply) => {
                    // Adjust the geometry based on the translation
                    rects[i].x += reply.dst_x() as i32;
                    rects[i].y += reply.dst_y() as i32;
                },
                Err(err) => {
                    eprintln!("Failed to translate coordinates: {}", err);
                }
            }
            i += 1;
        }
        rects
    }

    fn get_windows_rects(&self, window_ids: &[x::Window]) -> Vec<Rect> {
        // Build requests for all window rects
        let mut cookies = Vec::new();
        for &window_id in window_ids {
            cookies.push(self.conn.send_request(&x::GetGeometry {
                drawable: x::Drawable::Window(window_id),
            }));
        }
        // Wait for all replies
        let mut rects = Vec::new();
        for cookie in cookies {
            match self.conn.wait_for_reply(cookie) {
                Ok(reply) => {
                    rects.push(Rect {
                        x: reply.x() as i32,
                        y: reply.y() as i32,
                        w: reply.width() as i32,
                        h: reply.height() as i32,
                    });
                },
                Err(err) => {
                    eprintln!("Failed to get geometry: {}", err);
                }
            }
        }
        rects
    }

    pub fn get_windows_withdrawn(&self, window_ids: &[x::Window]) -> Vec<bool> {
        let cookies = window_ids.iter().map(|&window_id| {
            self.conn.send_request(&x::GetProperty {
                delete: false,
                window: window_id,
                property: self.atoms.wm_state,
                r#type: x::ATOM_ANY,
                long_offset: 0,
                long_length: 1024,
            })
        }).collect::<Vec<_>>();

        let mut withdrawns = Vec::new();
        for cookie in cookies {
            let reply = self.conn.wait_for_reply(cookie)
                .expect("Failed to get WM_STATE property");
            println!("Reply length: {}", reply.length());
            if reply.length() > 0 {
                let state = reply.value::<x::Atom>()[0];
                println!("Window state: {:?}", state);
                withdrawns.push(state == self.atoms.wm_state_withdrawn);
            } else {
                withdrawns.push(false);
            }
        }
        withdrawns
    }

    pub fn get_windows_properties(&self, window_ids: &[x::Window]) -> Vec<Vec<x::Atom>> {
        // Build requests to get properties for each window
        let mut cookies = Vec::new();
        for &window_id in window_ids {
            cookies.push(self.conn.send_request(&x::GetProperty {
                delete: false,
                window: window_id,
                property: self.atoms._net_wm_state,
                r#type: x::ATOM_ATOM,
                long_offset: 0,
                long_length: 1024, // Number of properties to fetch
            }));
        }

        // Wait for all replies and collect properties
        let mut properties = Vec::new();
        for cookie in cookies {
            match self.conn.wait_for_reply(cookie) {
                Ok(reply) => {
                    properties.push(reply.value().to_vec());
                },
                Err(err) => {
                    eprintln!("Failed to get properties: {}", err);
                    properties.push(vec![]);
                }
            }
        }
        properties
    }

    pub fn is_floating(&self, properties: &Vec<x::Atom>) -> bool {
        // Check if the window is floating
        ! (properties.contains(&self.atoms._net_wm_state_maximized_horz.into()) ||
            properties.contains(&self.atoms._net_wm_state_maximized_vert.into()))
    }

    pub fn is_visible(&self, properties: &Vec<x::Atom>) -> bool {
        // Check if the window is visible
        !properties.contains(&self.atoms._net_wm_state_hidden.into())
    }

    pub fn set_focus(&self, window_id: x::Window) -> Result<(), String> {
        // Set focus to the specified window
        let cookie = self.conn.send_request_checked(&x::SetInputFocus {
            focus: window_id,
            revert_to: x::InputFocus::None,
            time: x::CURRENT_TIME,
        });

        match self.conn.check_request(cookie) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to set focus: {}", err)),
        }
    }
}
