use std::convert::From;
use std::fs::File;
use std::io::{read_to_string, BufReader};
use std::path::PathBuf;
use sqlite::State;
use crate::models::Project;

#[tauri::command]
pub fn find_projects(app_handle: tauri::AppHandle) -> Vec<Project> {
    let connection = sqlite::open(app_handle.path_resolver().app_local_data_dir().unwrap().join("config.db")).unwrap();
    let mut statement = connection.prepare("
        SELECT (location) FROM directories WHERE single_project IS ?
    ").unwrap();
    statement.bind((1, 1)).unwrap();

    let mut projects: Vec<Project> = vec![];

    while let Ok(State::Row) = statement.next() {
        let path = PathBuf::from(statement.read::<String, _>("location").unwrap());
        let git_path = path.join(".git");

        let git_support = git_path.exists();

        // Read .git/config to attempt to find a reference to the "origin" remote
        let origin_remote = if git_support {
            let f = File::open(git_path.join("config")).unwrap();
            let f = BufReader::new(f);

            let mut remote_url: Option<String> = None;

            let mut found_remote = false;

            for line in read_to_string(f).unwrap().lines().map(String::from).collect::<Vec<String>>() {
                println!("{}", line);
                if found_remote && line.starts_with("\turl = "){
                    remote_url = Some(line.split("=").last().unwrap().trim().to_string());
                    break;
                }
                if line.eq_ignore_ascii_case("[remote \"origin\"]") {
                    found_remote = true;
                }
            }

            remote_url
        } else { None };

        let project = Project {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            location: path.to_str().unwrap().to_string(),
            git_support,
            has_remote: match origin_remote {
                Some(_) => true,
                None => false
            },
            origin_remote,
        };
        projects.push(project);
    }

    projects
}
