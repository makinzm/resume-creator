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

pub fn load_resume_from_path(path: &str) -> Result<Resume> {
    let data = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

pub fn save_resume_to_path(resume: &Resume, path: &str) -> Result<()> {
    let parent = std::path::Path::new(path).parent();
    if let Some(p) = parent {
        std::fs::create_dir_all(p)?;
    }
    let content = serde_json::to_string_pretty(resume)?;
    std::fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_roundtrip() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_resume.json");

        let mut resume = Resume::new();
        resume.personal_info.full_name = "John Doe".to_string();
        resume.personal_info.email = "john@example.com".to_string();

        save_resume_to_path(&resume, file_path.to_str().unwrap()).unwrap();

        let loaded = load_resume_from_path(file_path.to_str().unwrap()).unwrap();

        assert_eq!(loaded.personal_info.full_name, "John Doe");
        assert_eq!(loaded.personal_info.email, "john@example.com");
        assert_eq!(loaded.id, resume.id);
    }

    #[test]
    fn test_load_invalid_path() {
        let result = load_resume_from_path("/nonexistent/path/to/resume.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_invalid_json() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("invalid.json");

        std::fs::write(&file_path, "{ invalid json }").unwrap();

        let result = load_resume_from_path(file_path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_save_resume_to_path_creates_parent_dirs() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("nested/dir/resume.json");

        let resume = Resume::new();
        save_resume_to_path(&resume, file_path.to_str().unwrap()).unwrap();

        assert!(file_path.exists());
    }

    #[test]
    fn test_roundtrip_preserves_all_fields() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("full_resume.json");

        let mut resume = Resume::new();
        resume.personal_info.full_name = "Jane Smith".to_string();
        resume.personal_info.email = "jane@example.com".to_string();
        resume.personal_info.phone = Some("+1-555-0123".to_string());
        resume.personal_info.location = Some("New York".to_string());
        resume.professional_summary = Some("Experienced professional".to_string());

        resume.experience.push(crate::data::Experience {
            id: uuid::Uuid::new_v4().to_string(),
            company: "TechCorp".to_string(),
            position: "Engineer".to_string(),
            start_date: crate::data::YearMonth::new(2020, 1),
            end_date: None,
            is_current: true,
            description: "Great company".to_string(),
            location: Some("NYC".to_string()),
        });

        save_resume_to_path(&resume, file_path.to_str().unwrap()).unwrap();
        let loaded = load_resume_from_path(file_path.to_str().unwrap()).unwrap();

        assert_eq!(loaded.personal_info.full_name, "Jane Smith");
        assert_eq!(loaded.personal_info.phone, Some("+1-555-0123".to_string()));
        assert_eq!(loaded.professional_summary, Some("Experienced professional".to_string()));
        assert_eq!(loaded.experience.len(), 1);
        assert_eq!(loaded.experience[0].company, "TechCorp");
    }
}
