use std::{io::BufRead, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Application,
    Link,
    Directory,
}

#[derive(Debug)]
pub struct DesktopEntry {
    pub ty: Type,
    pub id: String,
    pub name: String,
    pub exec: String,
    pub icon: Option<PathBuf>,
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

    for line in lines {
        if line.starts_with('[') {
            // New group started
            break;
        }

        let Some((key, value)) = line.split_once('=') else { continue };

        match key {
            "Name" => name = Some(value.to_string()),
            "Exec" => exec = Some(value.to_string()),
            "Icon" => icon = Some(PathBuf::from(value)),
            "Type" => {
                ty = Some(match value {
                    "Application" => Type::Application,
                    "Link" => Type::Link,
                    "Directory" => Type::Directory,
                    _ => return None,
                })
            }
            _ => {}
        }
    }

    let id = path.file_stem()?.to_string_lossy().to_string();

    Some(DesktopEntry {
        id,
        icon,
        ty: ty?,
        name: name?,
        exec: exec?,
    })
}

const DESKTOP_APPLICATIONS_PATH: &str = "/usr/share/applications";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_desktop_entry_file() {
        for ent in find_application_desktop_entries() {
            println!("{:?}\n", ent);// TODO: remove
        }
    }
}
