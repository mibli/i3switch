#[cfg(feature = "i3")]
pub mod i3;
#[cfg(feature = "wmctl")]
pub mod wmctl;
#[cfg(feature = "xcb")]
pub mod xcb;

pub mod traits;
pub mod backend;

#[cfg(feature = "i3")]
pub use i3::Backend as I3Backend;
#[cfg(feature = "wmctl")]
pub use wmctl::Backend as WmctlBackend;
#[cfg(feature = "xcb")]
pub use xcb::Backend as XcbBackend;

pub use backend::Backend;
pub use backend::UsedBackend;
pub use traits::*;
