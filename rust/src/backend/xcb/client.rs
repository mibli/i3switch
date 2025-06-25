use xcb::{x, Connection};
use xcb::Xid;
use crate::types::Rect;
use crate::types::Window;

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
        pub wm_state_normal              => b"WM_STATE_NORMAL",
        pub wm_state_iconic              => b"WM_STATE_ICONIC",
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
            &self.atoms._net_wm_state_hidden,
        ] {
            if !supported_atoms.contains(&atom) {
                return Err(format!(
                    "Required atom '{}' is not supported",
                    self.get_atom_name(*atom)?
                ));
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

    pub fn get_active_window(&self) -> Result<x::Window, String> {
        // Get the active window from the root window
        let cookie = self.conn.send_request(&x::GetProperty {
            delete: false,
            window: self.root,
            property: self.atoms._net_active_window,
            r#type: x::ATOM_WINDOW,
            long_offset: 0,
            long_length: 1, // Only one active window
        });

        let x_windows = self.conn.wait_for_reply(cookie)
            .expect("Failed to get _NET_ACTIVE_WINDOW property")
            .value::<x::Window>().to_vec();
        match x_windows.first() {
            Some(window) => Ok(*window),
            None => Err("No active window found".to_string()),
        }
    }

    pub fn fetch_window_info(&self, window_id: &x::Window) -> Result<Window, String> {
        // Request all necessary information about the window
        // asynchronously to use xcb properly.
        let cookies = (
            self.request_geometry(window_id.clone()),
            self.request_normalized_offset(window_id.clone()),
            self.request_wm_state(window_id.clone()),
            self.request_ewmh_state(window_id.clone()),
        );

        // Wait for all requests to complete
        let replies = (
            self.conn.wait_for_reply(cookies.0),
            self.conn.wait_for_reply(cookies.1),
            self.conn.wait_for_reply(cookies.2),
            self.conn.wait_for_reply(cookies.3),
        );

        // Get geometry of the window
        let mut rect = match replies.0 {
            Ok(reply) => Rect::from(&reply),
            Err(err) => return Err(format!("Failed to get window geometry: {}", err)),
        };

        // Get the normalized offset of the window
        match replies.1 {
            Ok(reply) => translate_rect(&mut rect, &reply),
            Err(err) => return Err(format!("Failed to translate coordinates: {}", err)),
        };

        // Match WM state
        let wm_state = match replies.2 {
            Ok(reply) => reply.value::<x::Atom>().to_vec(),
            Err(err) => return Err(format!("Failed to get WM state: {}", err)),
        };

        // Match EWMH state
        let ewmh_state = match replies.3 {
            Ok(reply) => reply.value::<x::Atom>().to_vec(),
            Err(err) => return Err(format!("Failed to get EWMH state: {}", err)),
        };

        if self.is_hidden(&wm_state, &ewmh_state) {
            return Err("Skipping invisible window".to_string());
        }

        Ok(Window {
            id: window_id.resource_id().into(),
            rect: rect,
            floating: self.is_floating(),
            focused: false, // Focus state will be set later
        })
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

    fn get_atom_name(&self, atom: x::Atom) -> Result<String, String> {
        // Get the name of an atom
        let cookie = self.conn.send_request(&x::GetAtomName {
            atom: atom,
        });
        match self.conn.wait_for_reply(cookie) {
            Ok(reply) => Ok(reply.name().to_string()),
            Err(err) => Err(format!("Failed to get atom name: {}", err)),
        }
    }

    fn request_normalized_offset(&self, src_window: x::Window)
        -> x::TranslateCoordinatesCookie {
        // Request to translate coordinates from one window to another
        self.conn.send_request(&x::TranslateCoordinates {
            src_window: src_window,
            dst_window: self.root,
            src_x: 0,
            src_y: 0,
        })
    }

    fn request_geometry(&self, window_id: x::Window)
        -> x::GetGeometryCookie {
        // Request to get the geometry of a window
        self.conn.send_request(&x::GetGeometry {
            drawable: x::Drawable::Window(window_id),
        })
    }

    fn request_wm_state(&self, window_id: x::Window)
        -> x::GetPropertyCookie {
        // Request to get properties of a window
        self.conn.send_request(&x::GetProperty {
            delete: false,
            window: window_id,
            property: self.atoms.wm_state,
            r#type: x::ATOM_ANY,
            long_offset: 0,
            long_length: 1024, // Number of properties to fetch
        })
    }

    fn request_ewmh_state(&self, window_id: x::Window)
        -> x::GetPropertyCookie {
        // Request to get EWMH states of a window
        self.conn.send_request(&x::GetProperty {
            delete: false,
            window: window_id,
            property: self.atoms._net_wm_state,
            r#type: x::ATOM_ATOM,
            long_offset: 0,
            long_length: 1024, // Number of properties to fetch
        })
    }

    fn is_floating(&self) -> bool {
        return false; // Placeholder for floating logic
    }

    fn is_hidden(&self, wm_state: &Vec<x::Atom>, ewmh_state: &Vec<x::Atom>) -> bool {
        wm_state.first().expect("WM_STATE is missing window state")
            == &self.atoms.wm_state_withdrawn.into() ||
            ewmh_state.contains(&self.atoms._net_wm_state_hidden.into())
    }
}

impl From<&x::GetGeometryReply> for Rect {
    fn from(reply: &x::GetGeometryReply) -> Self {
        Rect {
            x: reply.x() as i32,
            y: reply.y() as i32,
            w: reply.width() as i32,
            h: reply.height() as i32,
        }
    }
}

fn translate_rect(rect: &mut Rect, translation: &x::TranslateCoordinatesReply) {
    // Translate the rectangle coordinates based on the translation reply
    rect.x += translation.dst_x() as i32;
    rect.y += translation.dst_y() as i32;
}
