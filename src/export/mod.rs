use crate::data::*;
use crate::storage;
use anyhow::Result;
use std::path::PathBuf;
use chrono::Local;
use std::fs;

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

pub fn export_pdf(resume: &Resume) -> Result<Vec<u8>> {
    // Try to find a Japanese-capable font on the system
    let _font_path = find_japanese_font_path()?;

    // Build text content for PDF
    let mut content = format!("{}\n\n", resume.personal_info.full_name);

    // Contact Info
    content.push_str(&format!("Email: {}\n", resume.personal_info.email));
    if let Some(phone) = &resume.personal_info.phone {
        content.push_str(&format!("Phone: {}\n", phone));
    }
    if let Some(location) = &resume.personal_info.location {
        content.push_str(&format!("Location: {}\n", location));
    }
    if let Some(website) = &resume.personal_info.website {
        content.push_str(&format!("Website: {}\n", website));
    }
    content.push_str("\n");

    // Professional Summary
    if let Some(summary) = &resume.professional_summary {
        content.push_str("PROFESSIONAL SUMMARY\n");
        content.push_str(&format!("{}\n\n", summary));
    }

    // Work Experience
    if !resume.experience.is_empty() {
        content.push_str("WORK EXPERIENCE\n");
        for exp in &resume.experience {
            let end_date = exp
                .end_date
                .as_ref()
                .map(|d| d.to_string_display())
                .unwrap_or_else(|| "Present".to_string());

            content.push_str(&format!("{} at {}\n", exp.position, exp.company));
            content.push_str(&format!(
                "{} - {}\n",
                exp.start_date.to_string_display(),
                end_date
            ));
            if let Some(location) = &exp.location {
                content.push_str(&format!("{}\n", location));
            }
            content.push_str(&format!("{}\n\n", exp.description));
        }
    }

    // Education
    if !resume.education.is_empty() {
        content.push_str("EDUCATION\n");
        for edu in &resume.education {
            content.push_str(&format!("{} in {}\n", edu.degree, edu.field_of_study));
            content.push_str(&format!("{}\n", edu.institution));
            content.push_str(&format!("Graduated: {}\n", edu.graduation_date.to_string_display()));
            if let Some(gpa) = &edu.gpa {
                content.push_str(&format!("GPA: {}\n", gpa));
            }
            content.push_str("\n");
        }
    }

    // Skills
    if !resume.skills.is_empty() {
        content.push_str("SKILLS\n");
        let mut by_category: std::collections::HashMap<String, Vec<&Skill>> =
            std::collections::HashMap::new();
        for skill in &resume.skills {
            by_category
                .entry(skill.category.to_string())
                .or_insert_with(Vec::new)
                .push(skill);
        }

        for (category, skills) in by_category {
            content.push_str(&format!("{}\n", category));
            for skill in skills {
                content.push_str(&format!("- {} ({})\n", skill.name, skill.proficiency));
            }
            content.push_str("\n");
        }
    }

    // Create a minimal valid PDF with the resume content
    create_minimal_pdf(&content)
}

fn create_minimal_pdf(content: &str) -> Result<Vec<u8>> {
    // Create a minimal valid PDF with UTF-8 text content
    let mut pdf = Vec::new();

    // PDF header
    pdf.extend_from_slice(b"%PDF-1.4\n");

    // Object 1: Catalog
    let obj1_start = pdf.len();
    pdf.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");

    // Object 2: Pages
    let obj2_start = pdf.len();
    pdf.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");

    // Object 3: Page
    let obj3_start = pdf.len();
    pdf.extend_from_slice(b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n");

    // Object 4: Content stream
    let obj4_start = pdf.len();
    let content_stream = format!("BT /F1 12 Tf 50 750 Td ({}) Tj ET\n", content.replace('\n', "\\n"));
    let obj4 = format!("4 0 obj\n<< /Length {} >>\nstream\n{}\nendstream\nendobj\n", content_stream.len(), content_stream);
    pdf.extend_from_slice(obj4.as_bytes());

    // Object 5: Font
    let obj5_start = pdf.len();
    pdf.extend_from_slice(b"5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n");

    // Xref table
    let xref_start = pdf.len();
    pdf.extend_from_slice(b"xref\n");
    pdf.extend_from_slice(b"0 6\n");
    pdf.extend_from_slice(format!("0000000000 65535 f \n").as_bytes());
    pdf.extend_from_slice(format!("{:010} 00000 n \n", obj1_start).as_bytes());
    pdf.extend_from_slice(format!("{:010} 00000 n \n", obj2_start).as_bytes());
    pdf.extend_from_slice(format!("{:010} 00000 n \n", obj3_start).as_bytes());
    pdf.extend_from_slice(format!("{:010} 00000 n \n", obj4_start).as_bytes());
    pdf.extend_from_slice(format!("{:010} 00000 n \n", obj5_start).as_bytes());

    // Trailer
    pdf.extend_from_slice(format!("trailer\n<< /Size 6 /Root 1 0 R >>\n").as_bytes());
    pdf.extend_from_slice(format!("startxref\n{}\n%%EOF\n", xref_start).as_bytes());

    Ok(pdf)
}

fn find_japanese_font_path() -> Result<String> {
    // Try common Japanese font locations on Linux
    let font_paths = vec![
        "/usr/share/fonts/opentype/noto/NotoSerifCJK-Regular.otc",
        "/usr/share/fonts/noto-cjk/NotoSerifCJK-Regular.otc",
        "/usr/share/fonts/truetype/noto-cjk/NotoSerifCJK-Regular.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.otc",
        "/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ];

    for path in font_paths {
        if fs::metadata(path).is_ok() {
            return Ok(path.to_string());
        }
    }

    // If no specific font found, return error with hint
    anyhow::bail!(
        "Could not find a suitable Japanese font. Please install fonts like 'fonts-noto-cjk' or 'fonts-noto-serif-cjk'"
    )
}

pub fn save_export_to_file(resume: &Resume, format: &str) -> Result<String> {
    let export_dir = storage::get_export_dir()?;

    let filename = format!(
        "resume_{}_{}.{}",
        resume.personal_info.full_name.replace(" ", "_"),
        Local::now().format("%Y%m%d_%H%M%S"),
        match format {
            "json" => "json",
            "pdf" => "pdf",
            _ => "md",
        }
    );

    let path = export_dir.join(&filename);

    match format {
        "json" => {
            let content = export_json(resume)?;
            std::fs::write(&path, &content)?;
        }
        "pdf" => {
            let pdf_bytes = export_pdf(resume)?;
            std::fs::write(&path, pdf_bytes)?;
        }
        _ => {
            let content = export_markdown(resume);
            std::fs::write(&path, &content)?;
        }
    }

    Ok(path.to_string_lossy().to_string())
}

pub fn save_export_to_file_with_path(resume: &Resume, format: &str, path: &str) -> Result<String> {
    let path_obj = PathBuf::from(path);

    // Create parent directory if it doesn't exist
    if let Some(parent) = path_obj.parent() {
        std::fs::create_dir_all(parent)?;
    }

    match format {
        "json" => {
            let content = export_json(resume)?;
            std::fs::write(&path_obj, &content)?;
        }
        "pdf" => {
            let pdf_bytes = export_pdf(resume)?;
            std::fs::write(&path_obj, pdf_bytes)?;
        }
        _ => {
            let content = export_markdown(resume);
            std::fs::write(&path_obj, &content)?;
        }
    }

    Ok(path_obj.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_resume() -> Resume {
        let mut resume = Resume::new();
        resume.personal_info.full_name = "Alice Johnson".to_string();
        resume.personal_info.email = "alice@example.com".to_string();
        resume
    }

    #[test]
    fn test_export_markdown_contains_name() {
        let resume = create_test_resume();
        let markdown = export_markdown(&resume);
        assert!(markdown.contains("# Alice Johnson"));
    }

    #[test]
    fn test_export_markdown_contains_contact_section() {
        let resume = create_test_resume();
        let markdown = export_markdown(&resume);
        assert!(markdown.contains("## Contact Information"));
        assert!(markdown.contains("alice@example.com"));
    }

    #[test]
    fn test_export_markdown_has_experience_section_when_nonempty() {
        let mut resume = create_test_resume();
        resume.experience.push(Experience {
            id: uuid::Uuid::new_v4().to_string(),
            company: "TechCorp".to_string(),
            position: "Senior Engineer".to_string(),
            start_date: YearMonth::new(2020, 1),
            end_date: Some(YearMonth::new(2024, 1)),
            is_current: false,
            description: "Led team projects".to_string(),
            location: Some("San Francisco".to_string()),
        });

        let markdown = export_markdown(&resume);
        assert!(markdown.contains("## Work Experience"));
        assert!(markdown.contains("Senior Engineer at TechCorp"));
    }

    #[test]
    fn test_export_markdown_no_experience_section_when_empty() {
        let resume = create_test_resume();
        let markdown = export_markdown(&resume);
        assert!(!markdown.contains("## Work Experience"));
    }

    #[test]
    fn test_export_json_valid() {
        let resume = create_test_resume();
        let json_result = export_json(&resume);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("Alice Johnson"));
        assert!(json_str.contains("alice@example.com"));
    }

    #[test]
    fn test_json_export_roundtrip() {
        let mut resume = create_test_resume();
        resume.personal_info.phone = Some("+1-555-0123".to_string());
        resume.professional_summary = Some("Experienced engineer".to_string());

        let json_str = export_json(&resume).unwrap();
        let loaded: Resume = serde_json::from_str(&json_str).unwrap();

        assert_eq!(loaded.personal_info.full_name, "Alice Johnson");
        assert_eq!(loaded.personal_info.email, "alice@example.com");
        assert_eq!(loaded.personal_info.phone, Some("+1-555-0123".to_string()));
        assert_eq!(loaded.professional_summary, Some("Experienced engineer".to_string()));
    }
}
