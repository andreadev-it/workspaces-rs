use directories::ProjectDirectories;
use std::fs;


pub struct Config {
    pub workspaces: Vec<Workspace>,
    pub config_file: String,
    pub result_file: String,
}

impl Config {
    pub fn build() -> Config {
        let mut workspaces = vec![];

        let project_dirs = ProjectDirectories::from_project_name("workspaces-rs");
        let config_dir = project_dirs.project_config_dir();
        if !config_dir.exists() {
            fs::create_dir(&config_dir).expect("Should have been able to create configuration directory.");
        }

        let workspaces_file_path = config_dir.join("workspaces.txt");
        if !workspaces_file_path.exists() {
            fs::File::create(&workspaces_file_path).expect("Should have been able to create workspaces file.");
        }

        let workspaces_txt = fs::read_to_string(&workspaces_file_path).expect("Should have been able to read the workspaces file.");
        for line in workspaces_txt.lines() {
            if line.is_empty() { continue; }

            let mut sections = line.split(",");
            let name = sections.next().unwrap();
            let path = sections.next().unwrap();
            workspaces.push(
                Workspace {
                    name: name.to_string(),
                    path: path.to_string() 
                }
            );
        }

        let config_file = workspaces_file_path.to_str().unwrap().to_string();
        let result_file = config_dir.join("result.txt").to_str().unwrap().to_string();

        workspaces.sort_by(|a, b| a.name.cmp(&b.name));

        Config {
            workspaces,
            config_file,
            result_file
        }
    }

    pub fn write_to_file(&mut self) -> Result<(), std::io::Error> {
        let workspaces_txt = self.workspaces
            .iter()
            .map(|workspace| format!("{},{}", workspace.name, workspace.path))
            .collect::<Vec<String>>()
            .join("\n");
        
        fs::write(&self.config_file, workspaces_txt)?;

        Ok(())
    }
}

pub struct Workspace {
    pub name: String,
    pub path: String
}
