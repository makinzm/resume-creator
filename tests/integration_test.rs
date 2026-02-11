use resume_creator::data::*;
use resume_creator::storage;
use resume_creator::export;
use tempfile::tempdir;

fn create_test_resume() -> Resume {
    let mut resume = Resume::new();
    resume.personal_info.full_name = "Bob Smith".to_string();
    resume.personal_info.email = "bob@example.com".to_string();
    resume.personal_info.phone = Some("+1-555-1234".to_string());
    resume.professional_summary = Some("Software engineer".to_string());

    resume.experience.push(Experience {
        id: uuid::Uuid::new_v4().to_string(),
        company: "TechCorp".to_string(),
        position: "Senior Engineer".to_string(),
        start_date: YearMonth::new(2020, 1),
        end_date: None,
        is_current: true,
        description: "Building great software".to_string(),
        location: Some("San Francisco".to_string()),
    });

    resume.education.push(Education {
        id: uuid::Uuid::new_v4().to_string(),
        institution: "University".to_string(),
        degree: "BS".to_string(),
        field_of_study: "Computer Science".to_string(),
        graduation_date: YearMonth::new(2020, 5),
        gpa: Some("3.8".to_string()),
    });

    resume.skills.push(Skill {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Rust".to_string(),
        proficiency: ProficiencyLevel::Expert,
        category: SkillCategory::Technical,
    });

    resume
}

#[test]
fn test_storage_roundtrip_full_resume() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("full_resume.json");

    let resume = create_test_resume();
    storage::save_resume_to_path(&resume, file_path.to_str().unwrap()).unwrap();

    let loaded = storage::load_resume_from_path(file_path.to_str().unwrap()).unwrap();

    assert_eq!(loaded.personal_info.full_name, "Bob Smith");
    assert_eq!(loaded.experience.len(), 1);
    assert_eq!(loaded.education.len(), 1);
    assert_eq!(loaded.skills.len(), 1);
    assert_eq!(loaded.experience[0].company, "TechCorp");
}

#[test]
fn test_export_to_file_markdown() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("resume.md");

    let resume = create_test_resume();
    export::save_export_to_file_with_path(&resume, "markdown", file_path.to_str().unwrap()).unwrap();

    assert!(file_path.exists());
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("Bob Smith"));
    assert!(content.contains("TechCorp"));
}

#[test]
fn test_export_to_file_json() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("resume.json");

    let resume = create_test_resume();
    export::save_export_to_file_with_path(&resume, "json", file_path.to_str().unwrap()).unwrap();

    assert!(file_path.exists());
    let content = std::fs::read_to_string(&file_path).unwrap();
    let loaded: Resume = serde_json::from_str(&content).unwrap();
    assert_eq!(loaded.personal_info.full_name, "Bob Smith");
}

#[test]
fn test_markdown_export_includes_all_sections() {
    let resume = create_test_resume();
    let markdown = export::export_markdown(&resume);

    assert!(markdown.contains("# Bob Smith"));
    assert!(markdown.contains("## Contact Information"));
    assert!(markdown.contains("bob@example.com"));
    assert!(markdown.contains("## Work Experience"));
    assert!(markdown.contains("Senior Engineer at TechCorp"));
    assert!(markdown.contains("## Education"));
    assert!(markdown.contains("BS in Computer Science"));
    assert!(markdown.contains("## Skills"));
    assert!(markdown.contains("Rust"));
}

#[test]
fn test_full_workflow_create_save_export() {
    let temp_dir = tempdir().unwrap();
    let save_path = temp_dir.path().join("resume_save.json");
    let export_path = temp_dir.path().join("resume_export.md");

    // Create and save
    let resume = create_test_resume();
    storage::save_resume_to_path(&resume, save_path.to_str().unwrap()).unwrap();

    // Load it back
    let loaded = storage::load_resume_from_path(save_path.to_str().unwrap()).unwrap();

    // Export as markdown
    export::save_export_to_file_with_path(&loaded, "markdown", export_path.to_str().unwrap()).unwrap();

    // Verify export file exists and has content
    assert!(export_path.exists());
    let export_content = std::fs::read_to_string(&export_path).unwrap();
    assert!(export_content.contains("Bob Smith"));
    assert!(export_content.contains("bob@example.com"));
}

#[test]
fn test_export_json_then_reload() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("test.json");

    let original = create_test_resume();
    export::save_export_to_file_with_path(&original, "json", file_path.to_str().unwrap()).unwrap();

    let content = std::fs::read_to_string(&file_path).unwrap();
    let reloaded: Resume = serde_json::from_str(&content).unwrap();

    assert_eq!(reloaded.personal_info.full_name, original.personal_info.full_name);
    assert_eq!(reloaded.experience.len(), original.experience.len());
    assert_eq!(reloaded.skills.len(), original.skills.len());
}
