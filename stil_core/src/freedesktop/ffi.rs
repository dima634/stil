#[cxx::bridge(namespace = "core")]
mod ffi {
    extern "Rust" {
        #[cxx_name = "IconLookup"]
        type IconLookupWrapper;

        #[Self = "IconLookupWrapper"]
        fn create() -> Box<IconLookupWrapper>;
        fn find_icon(&mut self, icon_name: &str, size: u32) -> String;
    }
}

#[derive(Debug, Default)]
struct IconLookupWrapper(super::IconLookup);

impl IconLookupWrapper {
    pub fn create() -> Box<Self> {
        Box::new(Self(super::IconLookup::default()))
    }

    /// Find icon by name and size, returning its path as a string. If not found, returns an empty string.
    pub fn find_icon(&mut self, icon_name: &str, size: u32) -> String {
        self.0
            .find_icon(icon_name, size, 1)
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|| String::new())
    }
}
