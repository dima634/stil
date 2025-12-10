use tracing::warn;

use crate::{
    hyprland::{self, Devices, GetDevicesCmd},
    keyboard,
};

#[derive(Debug)]
pub struct Keyboard {
    name: String,
    active_keymap: String,
    is_caps_lock_on: bool,
    is_num_lock_on: bool,
    is_main: bool,
}

impl Keyboard {
    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn active_keymap(&self) -> &String {
        &self.active_keymap
    }

    #[inline]
    pub fn set_active_keymap(&mut self, keymap: String) {
        self.active_keymap = keymap;
    }

    #[inline]
    pub fn is_caps_lock_on(&self) -> bool {
        self.is_caps_lock_on
    }

    #[inline]
    pub fn is_num_lock_on(&self) -> bool {
        self.is_num_lock_on
    }

    #[inline]
    pub fn is_main(&self) -> bool {
        self.is_main
    }
}

impl From<hyprland::Keyboard> for Keyboard {
    fn from(value: hyprland::Keyboard) -> Self {
        Self {
            name: value.name,
            active_keymap: value.active_keymap,
            is_caps_lock_on: value.caps_lock,
            is_num_lock_on: value.num_lock,
            is_main: value.main,
        }
    }
}

#[derive(Debug)]
pub struct KeyboardManager {
    keyboards: Vec<Keyboard>,
}

impl KeyboardManager {
    pub fn init(&mut self) {
        let Some(devices) = hyprland::HyprCtl::default().run(GetDevicesCmd) else {
            warn!("Failed to get keyboard state from Hyprland");
            return;
        };
        self.keyboards = devices.keyboards.into_iter().map(Keyboard::from).collect();
    }

    pub fn get_main_keyboard(&self) -> Option<&Keyboard> {
        self.keyboards.iter().find(|kb| kb.is_main())
    }

    pub fn set_keyboard_keymap(&mut self, keyboard_name: &str, keymap: String) {
        if let Some(kb) = self.keyboards.iter_mut().find(|kb| kb.name() == keyboard_name) {
            kb.set_active_keymap(keymap);
        }
    }
}

impl Default for KeyboardManager {
    #[inline]
    fn default() -> Self {
        Self { keyboards: Vec::new() }
    }
}
