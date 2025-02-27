use log::{debug, error, info};
use regex::Regex;
use savefile::prelude::*;
use serde::Serialize;
use std::{cmp::Ordering, collections::HashMap, env, fs, path::PathBuf};

use super::handler::{get_config_dir, make_file_system_safe_name};
use crate::{
    app::{kanban::Board, state::UiMode, AppConfig},
    constants::{
        CONFIG_DIR_NAME, CONFIG_FILE_NAME, SAVE_DIR_NAME, SAVE_FILE_NAME, THEME_DIR_NAME,
        THEME_FILE_NAME,
    },
    inputs::key::Key,
    io::handler::prepare_config_dir,
    ui::Theme,
};

extern crate savefile;

pub fn get_config(ignore_overlapped_keybinds: bool) -> Result<AppConfig, String> {
    let config_dir_status = get_config_dir();
    let config_dir = if let Ok(config_dir) = config_dir_status {
        config_dir
    } else {
        return Err(config_dir_status.unwrap_err());
    };
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    let config = match fs::read_to_string(config_path) {
        Ok(config) => AppConfig {
            // if config file has been found, parse it, if an error occurs, use default config and write it to file
            ..serde_json::from_str(&config).unwrap_or_else(|e| {
                error!("Error parsing config file: {}", e);
                let write_config_status = write_config(&AppConfig::default());
                if write_config_status.is_err() {
                    error!("{}", write_config_status.unwrap_err());
                }
                AppConfig::default()
            })
        },
        Err(_) => {
            // if config file has not been found, use default config and write it to file
            let config = AppConfig::default();
            let write_config_status = write_config(&config);
            if write_config_status.is_err() {
                error!("{}", write_config_status.unwrap_err());
            }
            AppConfig::default()
        }
    };
    let config_keybinds = config.keybindings.clone();
    // make sure there is no overlap between keybinds
    if ignore_overlapped_keybinds {
        return Ok(config);
    }
    let mut key_count_map: HashMap<Key, u16> = HashMap::new();
    for (_, value) in config_keybinds.iter() {
        for key in value.iter() {
            let key_count = key_count_map.entry(*key).or_insert(0);
            *key_count += 1;
        }
    }
    let mut overlapped_keys: Vec<Key> = Vec::new();
    for (key, count) in key_count_map.iter() {
        if *count > 1 {
            overlapped_keys.push(*key);
        }
    }
    if !overlapped_keys.is_empty() {
        let mut overlapped_keys_str = String::new();
        for key in overlapped_keys.iter() {
            overlapped_keys_str.push_str(&format!("{:?}, ", key));
        }
        return Err(format!(
            "Overlapped keybinds found: {}",
            overlapped_keys_str
        ));
    }
    Ok(config)
}

pub fn write_config(config: &AppConfig) -> Result<(), String> {
    let config_str = serde_json::to_string_pretty(&config).unwrap();
    prepare_config_dir()?;
    let config_dir = get_config_dir()?;
    let write_result = fs::write(config_dir.join(CONFIG_FILE_NAME), config_str);
    match write_result {
        Ok(_) => Ok(()),
        Err(e) => {
            debug!("Error writing config file: {}", e);
            Err("Error writing config file".to_string())
        }
    }
}

pub fn get_default_ui_mode() -> UiMode {
    let get_config_status = get_config(false);
    let config = if let Ok(config) = get_config_status {
        config
    } else {
        debug!("Error getting config: {}", get_config_status.unwrap_err());
        AppConfig::default()
    };
    config.default_view
}

pub fn reset_config() {
    let config = AppConfig::default();
    let write_config_status = write_config(&config);
    if write_config_status.is_err() {
        error!(
            "Error writing config file: {}",
            write_config_status.unwrap_err()
        );
    }
}

pub fn save_kanban_state_locally(boards: Vec<Board>) -> Result<(), SavefileError> {
    let get_config_status = get_config(false);
    let config = if let Ok(config) = get_config_status {
        config
    } else {
        debug!("Error getting config: {}", get_config_status.unwrap_err());
        AppConfig::default()
    };
    // check config.save_directory for previous versions of the boards
    // versioning style is: SAVE_FILE_NAME_27-12-2020_v1
    // if the file exists, increment the version number
    // if the file does not exist, version number is 1
    let files = fs::read_dir(&config.save_directory)?;
    let mut version = 1;
    for file in files {
        let file = file?;
        let file_name = file.file_name().into_string().unwrap();
        if file_name.contains(SAVE_FILE_NAME)
            && file_name.contains(chrono::Local::now().format("%d-%m-%Y").to_string().as_str())
        {
            let file_version = file_name.split('_').last();
            if let Some(file_version) = file_version {
                // remove v from version number and find max of version numbers
                let file_version = file_version.replace('v', "");
                let file_version = file_version.parse::<u32>();
                if let Ok(file_version) = file_version {
                    match file_version.cmp(&version) {
                        Ordering::Greater => {
                            version = file_version;
                            version += 1;
                        }
                        Ordering::Equal => {
                            version += 1;
                        }
                        Ordering::Less => {}
                    }
                } else {
                    debug!(
                        "Error parsing version number: {}",
                        file_version.unwrap_err()
                    );
                    continue;
                }
            }
        }
    }
    let file_name = format!(
        "{}_{}_v{}",
        SAVE_FILE_NAME,
        chrono::Local::now().format("%d-%m-%Y"),
        version
    );
    let file_path = config.save_directory.join(file_name);
    let save_status = save_file(file_path, version, &boards);
    match save_status {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn get_local_kanban_state(
    file_name: String,
    version: u32,
    preview_mode: bool,
) -> Result<Vec<Board>, SavefileError> {
    let get_config_status = get_config(false);
    let config = if let Ok(config) = get_config_status {
        config
    } else {
        debug!("Error getting config: {}", get_config_status.unwrap_err());
        AppConfig::default()
    };
    let file_path = config.save_directory.join(file_name);
    if !preview_mode {
        info!("Loading local save file: {:?}", file_path);
    }
    let boards = load_file(file_path, version)?;
    Ok(boards)
}

pub fn get_available_local_savefiles() -> Option<Vec<String>> {
    let get_config_status = get_config(false);
    let config = if let Ok(config) = get_config_status {
        config
    } else {
        debug!("Error getting config: {}", get_config_status.unwrap_err());
        AppConfig::default()
    };
    let read_dir_status = fs::read_dir(&config.save_directory);
    match read_dir_status {
        Ok(files) => {
            let mut savefiles = Vec::new();
            for file in files {
                let file = file.unwrap();
                let file_name = file.file_name().into_string().unwrap();
                savefiles.push(file_name);
            }
            // keep only the files which have follow the pattern SAVEFILE_NAME_<NaiveDate in format DD-MM-YYYY>_v<version number>
            // example kanban_02-12-2022_v7
            // use regex to match the pattern
            let re = Regex::new(r"^kanban_\d{2}-\d{2}-\d{4}_v\d+$").unwrap();
            savefiles.retain(|file| re.is_match(file));
            // order the files by date and version
            savefiles.sort_by(|a, b| {
                let a_date = a.split('_').nth(1).unwrap();
                let b_date = b.split('_').nth(1).unwrap();
                let a_version = a.split('_').nth(2).unwrap();
                let b_version = b.split('_').nth(2).unwrap();
                let a_date = chrono::NaiveDate::parse_from_str(a_date, "%d-%m-%Y").unwrap();
                let b_date = chrono::NaiveDate::parse_from_str(b_date, "%d-%m-%Y").unwrap();
                let a_version = a_version.split('v').nth(1).unwrap().parse::<u32>().unwrap();
                let b_version = b_version.split('v').nth(1).unwrap().parse::<u32>().unwrap();
                if a_date > b_date {
                    std::cmp::Ordering::Greater
                } else if a_date < b_date {
                    std::cmp::Ordering::Less
                } else if a_version > b_version {
                    std::cmp::Ordering::Greater
                } else if a_version < b_version {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            Some(savefiles)
        }
        Err(_) => {
            // try to create the save directory
            let default_save_path = env::temp_dir().join(SAVE_DIR_NAME);
            let dir_creation_status = fs::create_dir_all(&default_save_path);
            match dir_creation_status {
                Ok(_) => {
                    info!(
                        "Could not find save directory, created default save directory at: {:?}",
                        default_save_path
                    );
                }
                Err(e) => {
                    error!("Could not find save directory and could not create default save directory at: {:?}, error: {}", default_save_path, e);
                }
            }
            None
        }
    }
}

pub fn export_kanban_to_json(boards: &[Board]) -> Result<String, String> {
    #[derive(Serialize)]
    struct ExportStruct {
        kanban_version: String,
        export_date: String,
        boards: Vec<Board>,
    }
    // use serde serialization
    let get_config_status = get_config(false);
    let config = if let Ok(config) = get_config_status {
        config
    } else {
        debug!("Error getting config: {}", get_config_status.unwrap_err());
        AppConfig::default()
    };
    // make json with the keys Version, Date, Boards
    // get version from cargo.toml
    let version = env!("CARGO_PKG_VERSION");
    let date = chrono::Local::now().format("%d-%m-%Y");
    // make sure boards list is not converted to string but is a list in json
    let export_struct = ExportStruct {
        kanban_version: version.to_string(),
        export_date: date.to_string(),
        boards: boards.to_vec(),
    };
    let file_path = config.save_directory.join("kanban_export.json");
    // check if file exists if so add a number to the end of the file name with _<number>
    let file_path = if file_path.exists() {
        let mut i = 1;
        let mut new_file_path = config
            .save_directory
            .join(format!("kanban_export_{}.json", i));
        while new_file_path.exists() {
            i += 1;
            new_file_path = config
                .save_directory
                .join(format!("kanban_export_{}.json", i));
        }
        new_file_path
    } else {
        file_path
    };
    // write to file
    let write_status = fs::write(
        file_path.clone(),
        serde_json::to_string_pretty(&export_struct).unwrap(),
    );
    match write_status {
        Ok(_) => Ok(file_path.to_str().unwrap().to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_default_save_directory() -> PathBuf {
    let mut default_save_path = env::temp_dir();
    default_save_path.push(SAVE_DIR_NAME);
    default_save_path
}

fn get_theme_dir() -> Result<PathBuf, String> {
    let home_dir = home::home_dir();
    if home_dir.is_none() {
        return Err(String::from("Error getting home directory"));
    }
    let mut theme_dir = home_dir.unwrap();
    // check if windows or unix
    if cfg!(windows) {
        theme_dir.push("AppData");
        theme_dir.push("Roaming");
    } else {
        theme_dir.push(".config");
    }
    theme_dir.push(CONFIG_DIR_NAME);
    theme_dir.push(THEME_DIR_NAME);
    Ok(theme_dir)
}

pub fn get_saved_themes() -> Option<Vec<Theme>> {
    let theme_dir = get_theme_dir();
    if theme_dir.is_err() {
        return None;
    }
    let theme_dir = theme_dir.unwrap();
    let read_dir_status = fs::read_dir(&theme_dir);
    // we are looking for .json files with THEME_FILE_NAME as prefix
    let file_prefix = format!("{}_", THEME_FILE_NAME);
    let regex_str = format!("^{}.*\\.json$", file_prefix);
    let re = Regex::new(&regex_str).unwrap();
    match read_dir_status {
        Ok(files) => {
            let mut themes = Vec::new();
            for file in files {
                let file = file.unwrap();
                let file_name = file.file_name().into_string().unwrap();
                if re.is_match(&file_name) {
                    let file_path = theme_dir.join(file_name);
                    let read_status = fs::read_to_string(file_path);
                    if read_status.is_err() {
                        continue;
                    }
                    let read_status = read_status.unwrap();
                    let theme: Theme = serde_json::from_str(&read_status).unwrap();
                    themes.push(theme);
                }
            }
            Some(themes)
        }
        Err(_) => None,
    }
}

pub fn save_theme(theme: Theme) -> Result<String, String> {
    let theme_dir = get_theme_dir()?;
    let create_dir_status = fs::create_dir_all(&theme_dir);
    if let Err(e) = create_dir_status {
        return Err(e.to_string());
    }
    // export the theme to json using serde prefix the file name with THEME_FILE_NAME and put the theme.name next then .json
    let theme_name = format!(
        "{}_{}.json",
        THEME_FILE_NAME,
        make_file_system_safe_name(&theme.name)
    );
    let theme_path = theme_dir.join(theme_name);
    let write_status = fs::write(
        theme_path.clone(),
        serde_json::to_string_pretty(&theme).unwrap(),
    );
    if let Err(write_status) = write_status {
        return Err(write_status.to_string());
    }
    Ok(theme_path.to_str().unwrap().to_string())
}
