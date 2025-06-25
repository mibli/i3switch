#[cfg(feature = "i3")]
use crate::backend::i3;
#[cfg(feature = "wmctl")]
use crate::backend::wmctl;
#[cfg(feature = "xcb")]
use crate::backend::xcb;

use crate::backend::traits::*;
use crate::types::Windows;

pub enum UsedBackend {
    #[cfg(feature = "i3")]
    I3(i3::Backend),
    #[cfg(feature = "wmctl")]
    WmCtl(wmctl::Backend),
    #[cfg(feature = "xcb")]
    Xcb(xcb::Backend),
}

pub struct Backend {
    used_backend: UsedBackend,
}

impl Backend {
    pub fn new(use_backend: UsedBackend) -> Self {
        Self {
            used_backend: use_backend,
        }
    }
}

impl GetTabs for Backend {
    fn get_tabs(&self) -> Result<Windows, String> {
        match self.used_backend {
            #[cfg(feature = "i3")]
            UsedBackend::I3(ref i3) => i3.get_tabs(),
            #[cfg(feature = "wmctl")]
            UsedBackend::WmCtl(ref wmctl) => wmctl.get_tabs(),
            #[cfg(feature = "xcb")]
            UsedBackend::Xcb(ref xcb) => xcb.get_tabs(),
        }
    }
}

impl GetVisible for Backend {
    fn get_visible(&self) -> Result<Windows, String> {
        match self.used_backend {
            #[cfg(feature = "i3")]
            UsedBackend::I3(ref i3) => i3.get_visible(),
            #[cfg(feature = "wmctl")]
            UsedBackend::WmCtl(ref wmctl) => wmctl.get_visible(),
            #[cfg(feature = "xcb")]
            UsedBackend::Xcb(ref xcb) => xcb.get_visible(),
        }
    }
}

impl SetFocus for Backend {
    fn set_focus(& mut self, id: &u64) {
        match self.used_backend {
            #[cfg(feature = "i3")]
            UsedBackend::I3(ref mut i3) => i3.set_focus(id),
            #[cfg(feature = "wmctl")]
            UsedBackend::WmCtl(ref mut wmctl) => wmctl.set_focus(id),
            #[cfg(feature = "xcb")]
            UsedBackend::Xcb(ref mut xcb) => xcb.set_focus(id),
        }
    }
}
