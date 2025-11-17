//! Icon theme lookup implementation following the freedesktop.org specification.
//!
//! This module implements the [Icon Theme Specification](https://specifications.freedesktop.org/icon-theme/latest/)
//! for locating icon files across multiple themes with size and scale support.
//! # Examples
//!
//! Basic icon lookup using the current theme:
//!
//! ```no_run
//! use stil_core::freedesktop::icon_lookup::IconLookup;
//!
//! let mut lookup = IconLookup::default();
//!
//! // Find a 48x48 icon in the current theme (auto-detected or hicolor)
//! if let Some(path) = lookup.find_icon("firefox", 48, 1) {
//!     println!("Found icon at: {:?}", path);
//! }
//!
//! // Find a HiDPI icon (48x48 at 2x scale = 96x96 physical pixels)
//! if let Some(path) = lookup.find_icon("firefox", 48, 2) {
//!     println!("Found HiDPI icon at: {:?}", path);
//! }
//! ```
//!
//! Using a specific theme:
//!
//! ```no_run
//! use stil_core::freedesktop::icon_lookup::IconLookup;
//!
//! // Create lookup with a specific theme
//! let mut lookup = IconLookup::with_theme("Papirus");
//!
//! if let Some(path) = lookup.find_icon("folder", 48, 1) {
//!     println!("Found icon at: {:?}", path);
//! }
//!
//! // Or look up in a different theme without changing the current theme
//! if let Some(path) = lookup.find_icon_in_theme("folder", 48, 1, "Adwaita") {
//!     println!("Found icon in Adwaita: {:?}", path);
//! }
//! ```
//!
//! Finding the best icon from multiple candidates:
//!
//! ```no_run
//! use stil_core::freedesktop::icon_lookup::IconLookup;
//!
//! let mut lookup = IconLookup::default();
//!
//! // Try multiple icon names in order of preference
//! let candidates = ["text-x-python", "text-x-script", "text-plain"];
//! if let Some(path) = lookup.find_best_icon(&candidates, 48, 1) {
//!     println!("Found icon at: {:?}", path);
//! }
//! ```

use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// Type of icon directory sizing
#[derive(Debug, Clone, PartialEq)]
pub enum DirectoryType {
    /// Icon files have a fixed size
    Fixed,
    /// Icons can be scaled between MinSize and MaxSize
    Scalable,
    /// Icons can differ by the threshold value
    Threshold,
}

/// Metadata for an icon theme directory
#[derive(Debug, Clone)]
pub struct IconThemeDirectory {
    /// Nominal (unscaled) size of icons in this directory
    pub size: u32,
    /// Target scale (defaults to 1)
    pub scale: u32,
    /// Type of icon sizing
    pub directory_type: DirectoryType,
    /// Maximum size for scalable icons (defaults to size)
    pub max_size: u32,
    /// Minimum size for scalable icons (defaults to size)
    pub min_size: u32,
    /// Threshold for threshold type (defaults to 2)
    pub threshold: u32,
    /// Context of the icon directory
    pub context: Option<String>,
}

/// Represents an icon theme
#[derive(Debug)]
pub struct IconTheme {
    /// Internal name of the theme
    pub name: String,
    /// Display name
    pub display_name: Option<String>,
    /// Comment/description
    pub comment: Option<String>,
    /// Parent themes to inherit from
    pub inherits: Vec<String>,
    /// Directories in this theme with their metadata
    pub directories: HashMap<String, IconThemeDirectory>,
}

impl Default for IconThemeDirectory {
    fn default() -> Self {
        Self {
            size: 48,
            scale: 1,
            directory_type: DirectoryType::Threshold,
            max_size: 48,
            min_size: 48,
            threshold: 2,
            context: None,
        }
    }
}

impl IconTheme {
    /// Load an icon theme from a base directory
    pub fn load(base_dir: &Path, theme_name: &str) -> Option<Self> {
        let theme_path = base_dir.join(theme_name);
        let index_path = theme_path.join("index.theme");

        if !index_path.exists() {
            return None;
        }

        parse_index_theme(&index_path, theme_name.to_string())
    }
}

/// Parse an index.theme file
fn parse_index_theme(path: &Path, theme_name: String) -> Option<IconTheme> {
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);

    let mut display_name = None;
    let mut comment = None;
    let mut inherits = Vec::new();
    let mut directory_names = Vec::new();
    let mut directories = HashMap::new();
    let mut current_section = String::new();
    let mut current_dir_data = IconThemeDirectory::default();

    for line in reader.lines() {
        let line = line.ok()?;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Section header
        if line.starts_with('[') && line.ends_with(']') {
            // Save previous directory section if it was a directory
            if !current_section.is_empty() && current_section != "Icon Theme" {
                directories.insert(current_section.clone(), current_dir_data.clone());
                current_dir_data = IconThemeDirectory::default();
            }

            current_section = line[1..line.len() - 1].to_string();
            continue;
        }

        // Key=Value pair
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            if current_section == "Icon Theme" {
                match key {
                    "Name" => display_name = Some(value.to_string()),
                    "Comment" => comment = Some(value.to_string()),
                    "Inherits" => {
                        inherits = value
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    "Directories" | "ScaledDirectories" => {
                        let dirs = value
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty());
                        directory_names.extend(dirs);
                    }
                    _ => {}
                }
            } else {
                // Directory section
                match key {
                    "Size" => {
                        if let Ok(size) = value.parse::<u32>() {
                            current_dir_data.size = size;
                            if current_dir_data.max_size == 48 {
                                // default
                                current_dir_data.max_size = size;
                            }
                            if current_dir_data.min_size == 48 {
                                // default
                                current_dir_data.min_size = size;
                            }
                        }
                    }
                    "Scale" => {
                        if let Ok(scale) = value.parse::<u32>() {
                            current_dir_data.scale = scale;
                        }
                    }
                    "Type" => {
                        current_dir_data.directory_type = match value {
                            "Fixed" => DirectoryType::Fixed,
                            "Scalable" => DirectoryType::Scalable,
                            "Threshold" => DirectoryType::Threshold,
                            _ => DirectoryType::Threshold,
                        };
                    }
                    "MaxSize" => {
                        if let Ok(max_size) = value.parse::<u32>() {
                            current_dir_data.max_size = max_size;
                        }
                    }
                    "MinSize" => {
                        if let Ok(min_size) = value.parse::<u32>() {
                            current_dir_data.min_size = min_size;
                        }
                    }
                    "Threshold" => {
                        if let Ok(threshold) = value.parse::<u32>() {
                            current_dir_data.threshold = threshold;
                        }
                    }
                    "Context" => {
                        current_dir_data.context = Some(value.to_string());
                    }
                    _ => {}
                }
            }
        }
    }

    // Save last directory section
    if !current_section.is_empty() && current_section != "Icon Theme" {
        directories.insert(current_section, current_dir_data);
    }

    Some(IconTheme {
        name: theme_name,
        display_name,
        comment,
        inherits,
        directories,
    })
}

/// Check if a directory matches the requested size and scale
fn directory_matches_size(dir: &IconThemeDirectory, icon_size: u32, icon_scale: u32) -> bool {
    if dir.scale != icon_scale {
        return false;
    }

    match dir.directory_type {
        DirectoryType::Fixed => dir.size == icon_size,
        DirectoryType::Scalable => icon_size >= dir.min_size && icon_size <= dir.max_size,
        DirectoryType::Threshold => {
            icon_size >= dir.size.saturating_sub(dir.threshold) && icon_size <= dir.size.saturating_add(dir.threshold)
        }
    }
}

/// Calculate the size distance between a directory and requested size/scale
fn directory_size_distance(dir: &IconThemeDirectory, icon_size: u32, icon_scale: u32) -> u32 {
    let dir_size_scaled = dir.size * dir.scale;
    let icon_size_scaled = icon_size * icon_scale;

    match dir.directory_type {
        DirectoryType::Fixed => dir_size_scaled.abs_diff(icon_size_scaled),
        DirectoryType::Scalable => {
            let min_size_scaled = dir.min_size * dir.scale;
            let max_size_scaled = dir.max_size * dir.scale;

            if icon_size_scaled < min_size_scaled {
                min_size_scaled - icon_size_scaled
            } else if icon_size_scaled > max_size_scaled {
                icon_size_scaled - max_size_scaled
            } else {
                0
            }
        }
        DirectoryType::Threshold => {
            let min_size_scaled = (dir.size.saturating_sub(dir.threshold)) * dir.scale;
            let max_size_scaled = (dir.size.saturating_add(dir.threshold)) * dir.scale;

            if icon_size_scaled < min_size_scaled {
                min_size_scaled - icon_size_scaled
            } else if icon_size_scaled > max_size_scaled {
                icon_size_scaled - max_size_scaled
            } else {
                0
            }
        }
    }
}

/// Icon lookup context containing base directories and theme cache
pub struct IconLookup {
    base_directories: Vec<PathBuf>,
    theme_cache: HashMap<String, IconTheme>,
    current_theme: String,
}

impl IconLookup {
    /// Create with a specific theme name
    pub fn with_theme(theme_name: impl Into<String>) -> Self {
        let base_directories = get_icon_base_directories();
        Self {
            base_directories,
            theme_cache: HashMap::new(),
            current_theme: theme_name.into(),
        }
    }
    
    /// Create with custom base directories and theme
    pub fn with_base_directories(base_directories: Vec<PathBuf>, theme_name: impl Into<String>) -> Self {
        Self {
            base_directories,
            theme_cache: HashMap::new(),
            current_theme: theme_name.into(),
        }
    }
    
    /// Get the current theme name
    pub fn current_theme(&self) -> &str {
        &self.current_theme
    }
    
    /// Set the current theme
    pub fn set_current_theme(&mut self, theme_name: impl Into<String>) {
        self.current_theme = theme_name.into();
    }

    /// Find an icon file using the current theme
    pub fn find_icon(&mut self, icon_name: &str, size: u32, scale: u32) -> Option<PathBuf> {
        let theme = self.current_theme.clone();
        self.find_icon_in_theme(icon_name, size, scale, &theme)
    }

    /// Find an icon file in a specific theme
    pub fn find_icon_in_theme(&mut self, icon_name: &str, size: u32, scale: u32, theme_name: &str) -> Option<PathBuf> {
        // Try to find in the specified theme
        if let Some(path) = self.find_icon_helper(icon_name, size, scale, theme_name) {
            return Some(path);
        }

        // Fall back to hicolor theme
        if theme_name != "hicolor" {
            if let Some(path) = self.find_icon_helper(icon_name, size, scale, "hicolor") {
                return Some(path);
            }
        }

        // Fall back to unthemed icons
        self.lookup_fallback_icon(icon_name)
    }

    /// Helper function for recursive theme lookup
    fn find_icon_helper(&mut self, icon_name: &str, size: u32, scale: u32, theme_name: &str) -> Option<PathBuf> {
        // Try to lookup in the theme
        if let Some(path) = self.lookup_icon(icon_name, size, scale, theme_name) {
            return Some(path);
        }

        // Get parent themes
        let parents = self.get_theme_parents(theme_name);

        // Recursively search in parent themes
        for parent in parents {
            if let Some(path) = self.find_icon_helper(icon_name, size, scale, &parent) {
                return Some(path);
            }
        }

        None
    }

    /// Lookup an icon in a specific theme
    fn lookup_icon(&mut self, icon_name: &str, size: u32, scale: u32, theme_name: &str) -> Option<PathBuf> {
        // Load theme first to avoid borrow checker issues
        self.load_theme(theme_name)?;

        // Now get the theme and clone directories to avoid holding a reference
        let directories = &self.theme_cache.get(theme_name)?.directories;
        let extensions = ["png", "svg", "xpm"];

        // First pass: look for exact size match
        for (subdir_name, dir_data) in directories {
            if directory_matches_size(dir_data, size, scale) {
                for base_dir in &self.base_directories {
                    let theme_dir = base_dir.join(theme_name).join(subdir_name);
                    for ext in &extensions {
                        let icon_path = theme_dir.join(format!("{}.{}", icon_name, ext));
                        if icon_path.exists() {
                            return Some(icon_path);
                        }
                    }
                }
            }
        }

        // Second pass: find closest size
        let mut minimal_distance = u32::MAX;
        let mut closest_filename = None;

        for (subdir_name, dir_data) in directories {
            let distance = directory_size_distance(dir_data, size, scale);

            if distance < minimal_distance {
                for base_dir in &self.base_directories {
                    let theme_dir = base_dir.join(theme_name).join(subdir_name);
                    for ext in &extensions {
                        let icon_path = theme_dir.join(format!("{}.{}", icon_name, ext));
                        if icon_path.exists() {
                            closest_filename = Some(icon_path);
                            minimal_distance = distance;
                        }
                    }
                }
            }
        }

        closest_filename
    }

    /// Lookup fallback icon (unthemed)
    fn lookup_fallback_icon(&self, icon_name: &str) -> Option<PathBuf> {
        let extensions = ["png", "svg", "xpm"];

        for base_dir in &self.base_directories {
            for ext in &extensions {
                let icon_path = base_dir.join(format!("{}.{}", icon_name, ext));
                if icon_path.exists() {
                    return Some(icon_path);
                }
            }
        }

        None
    }

    /// Load a theme from cache or disk
    fn load_theme(&mut self, theme_name: &str) -> Option<&IconTheme> {
        if !self.theme_cache.contains_key(theme_name) {
            // Try to load from any base directory
            for base_dir in &self.base_directories {
                if let Some(theme) = IconTheme::load(base_dir, theme_name) {
                    self.theme_cache.insert(theme_name.to_string(), theme);
                    break;
                }
            }
        }

        self.theme_cache.get(theme_name)
    }

    /// Get parent themes for a given theme
    fn get_theme_parents(&mut self, theme_name: &str) -> Vec<String> {
        if let Some(theme) = self.load_theme(theme_name) {
            let mut parents = theme.inherits.clone();

            // Ensure hicolor is always in the inheritance chain
            if !parents.contains(&"hicolor".to_string()) && theme_name != "hicolor" {
                parents.push("hicolor".to_string());
            }

            parents
        } else {
            vec![]
        }
    }

    /// Find the first matching icon from a list of icon names using the current theme
    pub fn find_best_icon(&mut self, icon_list: &[&str], size: u32, scale: u32) -> Option<PathBuf> {
        let theme = self.current_theme.clone();
        self.find_best_icon_in_theme(icon_list, size, scale, &theme)
    }

    /// Find the first matching icon from a list of icon names in a specific theme
    pub fn find_best_icon_in_theme(&mut self, icon_list: &[&str], size: u32, scale: u32, theme_name: &str) -> Option<PathBuf> {
        // Try to find in the specified theme
        if let Some(path) = self.find_best_icon_helper(icon_list, size, scale, theme_name) {
            return Some(path);
        }

        // Fall back to hicolor theme
        if theme_name != "hicolor" {
            if let Some(path) = self.find_best_icon_helper(icon_list, size, scale, "hicolor") {
                return Some(path);
            }
        }

        // Fall back to unthemed icons
        for icon_name in icon_list {
            if let Some(path) = self.lookup_fallback_icon(icon_name) {
                return Some(path);
            }
        }

        None
    }

    /// Helper for finding best icon from a list
    fn find_best_icon_helper(
        &mut self,
        icon_list: &[&str],
        size: u32,
        scale: u32,
        theme_name: &str,
    ) -> Option<PathBuf> {
        // Try each icon in the list
        for icon_name in icon_list {
            if let Some(path) = self.lookup_icon(icon_name, size, scale, theme_name) {
                return Some(path);
            }
        }

        // Recursively search in parent themes
        let parents = self.get_theme_parents(theme_name);
        for parent in parents {
            if let Some(path) = self.find_best_icon_helper(icon_list, size, scale, &parent) {
                return Some(path);
            }
        }

        None
    }
}

impl Default for IconLookup {
    fn default() -> Self {
        let base_directories = get_icon_base_directories();
        let current_theme = detect_current_theme();
        Self {
            base_directories,
            theme_cache: HashMap::new(),
            current_theme,
        }
    }
}

/// Detect the current icon theme from the environment
/// 
/// Attempts to read from common environment variables or desktop-specific settings.
/// Falls back to "hicolor" if no theme is detected.
fn detect_current_theme() -> String {
    // Try GTK settings
    if let Ok(theme) = std::env::var("GTK_THEME") {
        if !theme.is_empty() {
            return theme;
        }
    }
    
    // Try reading from GTK3 settings file
    if let Some(home) = std::env::var_os("HOME") {
        let settings_path = PathBuf::from(home).join(".config/gtk-3.0/settings.ini");
        if let Ok(content) = std::fs::read_to_string(settings_path) {
            for line in content.lines() {
                if let Some(theme) = line.strip_prefix("gtk-icon-theme-name=") {
                    let theme = theme.trim().trim_matches('"');
                    if !theme.is_empty() {
                        return theme.to_string();
                    }
                }
            }
        }
    }
    
    // Default to hicolor
    "hicolor".to_string()
}

/// Get the standard icon base directories according to the XDG Base Directory spec
fn get_icon_base_directories() -> Vec<PathBuf> {
    let mut directories = Vec::new();

    // $HOME/.icons (for backwards compatibility)
    if let Some(home) = std::env::var_os("HOME") {
        directories.push(PathBuf::from(home).join(".icons"));
    }

    // $XDG_DATA_HOME/icons (defaults to $HOME/.local/share/icons)
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        directories.push(PathBuf::from(data_home).join("icons"));
    } else if let Some(home) = std::env::var_os("HOME") {
        directories.push(PathBuf::from(home).join(".local/share/icons"));
    }

    // $XDG_DATA_DIRS/icons (defaults to /usr/local/share/icons:/usr/share/icons)
    if let Some(data_dirs) = std::env::var_os("XDG_DATA_DIRS") {
        for dir in std::env::split_paths(&data_dirs) {
            directories.push(dir.join("icons"));
        }
    } else {
        directories.push(PathBuf::from("/usr/local/share/icons"));
        directories.push(PathBuf::from("/usr/share/icons"));
    }

    // /usr/share/pixmaps
    directories.push(PathBuf::from("/usr/share/pixmaps"));

    directories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directory_matches_size_fixed() {
        let dir = IconThemeDirectory {
            size: 48,
            scale: 1,
            directory_type: DirectoryType::Fixed,
            ..Default::default()
        };

        assert!(directory_matches_size(&dir, 48, 1));
        assert!(!directory_matches_size(&dir, 32, 1));
        assert!(!directory_matches_size(&dir, 48, 2));
    }

    #[test]
    fn test_directory_matches_size_scalable() {
        let dir = IconThemeDirectory {
            size: 48,
            scale: 1,
            directory_type: DirectoryType::Scalable,
            min_size: 16,
            max_size: 256,
            ..Default::default()
        };

        assert!(directory_matches_size(&dir, 48, 1));
        assert!(directory_matches_size(&dir, 32, 1));
        assert!(directory_matches_size(&dir, 16, 1));
        assert!(directory_matches_size(&dir, 256, 1));
        assert!(!directory_matches_size(&dir, 8, 1));
        assert!(!directory_matches_size(&dir, 512, 1));
    }

    #[test]
    fn test_directory_matches_size_threshold() {
        let dir = IconThemeDirectory {
            size: 48,
            scale: 1,
            directory_type: DirectoryType::Threshold,
            threshold: 2,
            ..Default::default()
        };

        assert!(directory_matches_size(&dir, 48, 1));
        assert!(directory_matches_size(&dir, 46, 1));
        assert!(directory_matches_size(&dir, 50, 1));
        assert!(!directory_matches_size(&dir, 45, 1));
        assert!(!directory_matches_size(&dir, 51, 1));
    }

    #[test]
    fn test_directory_size_distance_fixed() {
        let dir = IconThemeDirectory {
            size: 48,
            scale: 1,
            directory_type: DirectoryType::Fixed,
            ..Default::default()
        };

        assert_eq!(directory_size_distance(&dir, 48, 1), 0);
        assert_eq!(directory_size_distance(&dir, 32, 1), 16);
        assert_eq!(directory_size_distance(&dir, 64, 1), 16);
    }

    #[test]
    fn test_directory_size_distance_scalable() {
        let dir = IconThemeDirectory {
            size: 48,
            scale: 1,
            directory_type: DirectoryType::Scalable,
            min_size: 16,
            max_size: 256,
            ..Default::default()
        };

        assert_eq!(directory_size_distance(&dir, 48, 1), 0);
        assert_eq!(directory_size_distance(&dir, 32, 1), 0);
        assert_eq!(directory_size_distance(&dir, 8, 1), 8);
        assert_eq!(directory_size_distance(&dir, 512, 1), 256);
    }
}
