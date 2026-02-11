use crate::data::Resume;
use anyhow::Result;
use std::path::PathBuf;

pub fn get_resumes_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let dir = home.join(".config/resume-creator");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

pub fn get_export_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    Ok(home)
}

pub fn save_resume(resume: &Resume) -> Result<()> {
    let dir = get_resumes_dir()?;
    let file_path = dir.join(format!("{}.json", resume.id));
    let content = serde_json::to_string_pretty(resume)?;
    std::fs::write(file_path, content)?;
    Ok(())
}

pub fn load_resume(id: &str) -> Result<Resume> {
    let dir = get_resumes_dir()?;
    let file_path = dir.join(format!("{}.json", id));
    let content = std::fs::read_to_string(file_path)?;
    let resume: Resume = serde_json::from_str(&content)?;
    Ok(resume)
}

pub fn list_resumes() -> Result<Vec<Resume>> {
    let dir = get_resumes_dir()?;
    let mut resumes = Vec::new();

    if dir.exists() {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(resume) = serde_json::from_str::<Resume>(&content) {
                        resumes.push(resume);
                    }
                }
            }
        }
    }

    Ok(resumes)
}

pub fn delete_resume(id: &str) -> Result<()> {
    let dir = get_resumes_dir()?;
    let file_path = dir.join(format!("{}.json", id));
    if file_path.exists() {
        std::fs::remove_file(file_path)?;
    }
    Ok(())
}
