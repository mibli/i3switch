pub mod i3;
pub mod wmctl;
pub mod xcb;
pub mod traits;
pub mod backend;

pub use i3::Backend as I3Backend;
pub use wmctl::Backend as WmctlBackend;
pub use xcb::Backend as XcbBackend;
pub use backend::Backend;
pub use backend::UsedBackend;
pub use traits::*;
