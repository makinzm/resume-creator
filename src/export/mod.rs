use crate::data::*;
use crate::storage;
use anyhow::Result;
use std::path::PathBuf;
use chrono::Local;

pub fn export_markdown(resume: &Resume) -> String {
    let mut output = String::new();

    // Header
    output.push_str(&format!("# {}\n\n", resume.personal_info.full_name));

    // Contact Info
    output.push_str("## Contact Information\n\n");
    output.push_str(&format!("- **Email:** {}\n", resume.personal_info.email));
    if let Some(phone) = &resume.personal_info.phone {
        output.push_str(&format!("- **Phone:** {}\n", phone));
    }
    if let Some(location) = &resume.personal_info.location {
        output.push_str(&format!("- **Location:** {}\n", location));
    }
    if let Some(website) = &resume.personal_info.website {
        output.push_str(&format!("- **Website:** {}\n", website));
    }
    output.push_str("\n");

    // Professional Summary
    if let Some(summary) = &resume.professional_summary {
        output.push_str("## Professional Summary\n\n");
        output.push_str(&format!("{}\n\n", summary));
    }

    // Experience
    if !resume.experience.is_empty() {
        output.push_str("## Work Experience\n\n");
        for exp in &resume.experience {
            output.push_str(&format!("### {} at {}\n\n", exp.position, exp.company));
            let end_date = exp
                .end_date
                .as_ref()
                .map(|d| d.to_string_display())
                .unwrap_or_else(|| "Present".to_string());
            output.push_str(&format!(
                "**{}** - {}\n\n",
                exp.start_date.to_string_display(),
                end_date
            ));
            if let Some(location) = &exp.location {
                output.push_str(&format!("*{}*\n\n", location));
            }
            output.push_str(&format!("{}\n\n", exp.description));
        }
    }

    // Education
    if !resume.education.is_empty() {
        output.push_str("## Education\n\n");
        for edu in &resume.education {
            output.push_str(&format!(
                "### {} in {}\n\n",
                edu.degree, edu.field_of_study
            ));
            output.push_str(&format!("**{}**\n\n", edu.institution));
            output.push_str(&format!("Graduated: {}\n\n", edu.graduation_date.to_string_display()));
            if let Some(gpa) = &edu.gpa {
                output.push_str(&format!("GPA: {}\n\n", gpa));
            }
        }
    }

    // Skills
    if !resume.skills.is_empty() {
        output.push_str("## Skills\n\n");
        let mut by_category: std::collections::HashMap<String, Vec<&Skill>> =
            std::collections::HashMap::new();
        for skill in &resume.skills {
            by_category
                .entry(skill.category.to_string())
                .or_insert_with(Vec::new)
                .push(skill);
        }

        for (category, skills) in by_category {
            output.push_str(&format!("### {}\n\n", category));
            for skill in skills {
                output.push_str(&format!("- {} ({})\n", skill.name, skill.proficiency));
            }
            output.push_str("\n");
        }
    }

    output
}

pub fn export_json(resume: &Resume) -> Result<String> {
    Ok(serde_json::to_string_pretty(resume)?)
}

pub fn save_export_to_file(resume: &Resume, format: &str) -> Result<String> {
    let export_dir = storage::get_export_dir()?;

    let filename = format!(
        "resume_{}_{}.{}",
        resume.personal_info.full_name.replace(" ", "_"),
        Local::now().format("%Y%m%d_%H%M%S"),
        match format {
            "json" => "json",
            _ => "md",
        }
    );

    let path = export_dir.join(&filename);

    let content = match format {
        "json" => export_json(resume)?,
        _ => export_markdown(resume),
    };

    std::fs::write(&path, &content)?;

    Ok(path.to_string_lossy().to_string())
}
