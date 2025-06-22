use crate::backend::i3;
use crate::backend::wmctl;
use crate::backend::traits::*;
use crate::types::Windows;

pub enum UsedBackend {
    I3(i3::Backend),
    WmCtl(wmctl::Backend),
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
            UsedBackend::I3(ref i3) => i3.get_tabs(),
            UsedBackend::WmCtl(ref wmctl) => wmctl.get_tabs(),
        }
    }
}

impl GetVisible for Backend {
    fn get_visible(&self) -> Result<Windows, String> {
        match self.used_backend {
            UsedBackend::I3(ref i3) => i3.get_visible(),
            UsedBackend::WmCtl(ref wmctl) => wmctl.get_visible(),
        }
    }
}

impl SetFocus for Backend {
    fn set_focus(& mut self, id: &u64) {
        match self.used_backend {
            UsedBackend::I3(ref mut i3) => i3.set_focus(id),
            UsedBackend::WmCtl(ref mut wmctl) => wmctl.set_focus(id),
        }
    }
}
