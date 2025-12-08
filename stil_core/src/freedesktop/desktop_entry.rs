use std::{io::BufRead, path::PathBuf};
use tracing::warn;

#[derive(Debug)]
pub struct DesktopEntry {
    pub ty: Type,
    pub id: String,
    pub name: String,
    pub exec: Exec,
    pub icon: Option<String>,
    pub wm_class: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Application,
    Link,
    Directory,
}

#[derive(Debug)]
pub struct Exec(String);

#[derive(Debug)]
pub enum FileArg {
    Single(String),
    Multiple(Vec<String>),
    Url(String),
    Urls(Vec<String>),
    None,
}

impl Exec {
    pub fn with_arg(&self, arg: FileArg) -> String {
        let mut look_behind = '\0';
        let mut result = String::with_capacity(self.0.len());

        for cur in self.0.chars() {
            match (look_behind, cur) {
                ('%', 'f') => {
                    if let FileArg::Single(file) = &arg {
                        result.push_str(file);
                    }
                }
                ('%', 'F') => {
                    if let FileArg::Multiple(files) = &arg {
                        result.push_str(&files.join(" "));
                    }
                }
                ('%', 'u') => {
                    if let FileArg::Url(url) = &arg {
                        result.push_str(url);
                    }
                }
                ('%', 'U') => {
                    if let FileArg::Urls(urls) = &arg {
                        result.push_str(&urls.join(" "));
                    }
                }
                ('%', '%') => {
                    result.push('%');
                }
                ('%', _) => {
                    warn!("Unknown exec field specifier: %{}", cur);
                }
                (_, '%') => {
                    look_behind = '%';
                    continue;
                }
                (_, curr) => {
                    result.push(curr);
                    look_behind = curr;
                    continue;
                }
            }
            look_behind = '\0';
        }

        result.trim().to_string()
    }
}

pub fn find_application_desktop_entries() -> Vec<DesktopEntry> {
    let mut desktop_entries = find_desktop_entries(PathBuf::from(DESKTOP_APPLICATIONS_PATH));
    desktop_entries.retain(|entry| entry.ty == Type::Application);
    desktop_entries
}

fn find_desktop_entries(path: PathBuf) -> Vec<DesktopEntry> {
    let Ok(files) = std::fs::read_dir(path) else { return vec![] };

    let mut desktop_entries = Vec::new();
    for dir_entry in files {
        let Ok(dir_entry) = dir_entry else { continue };
        let Ok(file_type) = dir_entry.file_type() else { continue };

        if !file_type.is_file() {
            continue;
        }

        let path = dir_entry.path();
        if path.extension() != Some("desktop".as_ref()) {
            continue;
        }

        if let Some(entry) = parse_desktop_entry_file(path) {
            desktop_entries.push(entry);
        }
    }

    desktop_entries
}

fn parse_desktop_entry_file(path: PathBuf) -> Option<DesktopEntry> {
    assert!(path.is_file() && path.extension() == Some("desktop".as_ref()));
    let file = std::fs::File::open(&path).ok()?;
    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines().filter_map(|line| line.ok());

    let group_header = lines.next()?;
    if group_header != "[Desktop Entry]" {
        return None;
    }

    let mut name = None;
    let mut exec = None;
    let mut icon = None;
    let mut ty = None;
    let mut wm_class = None;

    for line in lines {
        if line.starts_with('[') {
            // New group started
            break;
        }

        let Some((key, value)) = line.split_once('=') else { continue };

        match key {
            "Name" => name = Some(value.to_string()),
            "Exec" => exec = Some(value.to_string()),
            "Icon" => icon = Some(value.to_string()),
            "Type" => {
                ty = Some(match value {
                    "Application" => Type::Application,
                    "Link" => Type::Link,
                    "Directory" => Type::Directory,
                    _ => return None,
                })
            }
            "StartupWMClass" => wm_class = Some(value.to_string()),
            _ => {}
        }
    }

    let id = path.file_stem()?.to_string_lossy().to_string();

    Some(DesktopEntry {
        id,
        icon,
        wm_class,
        ty: ty?,
        name: name?,
        exec: Exec(exec?),
    })
}

const DESKTOP_APPLICATIONS_PATH: &str = "/usr/share/applications";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_with_arg() {
        let single_file_arg = Exec("myapp %f".to_string());
        assert_eq!(
            single_file_arg.with_arg(FileArg::Single("file.txt".to_string())),
            "myapp file.txt"
        );

        let multiple_files_arg = Exec("myapp %F".to_string());
        assert_eq!(
            multiple_files_arg.with_arg(FileArg::Multiple(vec![
                "file1.txt".to_string(),
                "file2.txt".to_string()
            ])),
            "myapp file1.txt file2.txt"
        );

        let escaped_percent_arg = Exec("myapp %% --option %f".to_string());
        assert_eq!(
            escaped_percent_arg.with_arg(FileArg::Single("file.txt".to_string())),
            "myapp % --option file.txt"
        );

        let none_arg = Exec("myapp --option %f %u".to_string());
        assert_eq!(none_arg.with_arg(FileArg::None), "myapp --option");
    }
}
