use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub id: String,
    pub personal_info: PersonalInfo,
    pub professional_summary: Option<String>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub skills: Vec<Skill>,
    pub metadata: ResumeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub id: String,
    pub company: String,
    pub position: String,
    pub start_date: YearMonth,
    pub end_date: Option<YearMonth>,
    pub is_current: bool,
    pub description: String,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub id: String,
    pub institution: String,
    pub degree: String,
    pub field_of_study: String,
    pub graduation_date: YearMonth,
    pub gpa: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub proficiency: ProficiencyLevel,
    pub category: SkillCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeMetadata {
    pub created_at: String,
    pub updated_at: String,
    pub version: u32,
    pub language: Language,
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearMonth {
    pub year: u16,
    pub month: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProficiencyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl std::fmt::Display for ProficiencyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProficiencyLevel::Beginner => write!(f, "Beginner"),
            ProficiencyLevel::Intermediate => write!(f, "Intermediate"),
            ProficiencyLevel::Advanced => write!(f, "Advanced"),
            ProficiencyLevel::Expert => write!(f, "Expert"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillCategory {
    Technical,
    Language,
    SoftSkill,
    Tool,
}

impl std::fmt::Display for SkillCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkillCategory::Technical => write!(f, "Technical"),
            SkillCategory::Language => write!(f, "Language"),
            SkillCategory::SoftSkill => write!(f, "Soft Skill"),
            SkillCategory::Tool => write!(f, "Tool"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "ja")]
    Japanese,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::English => write!(f, "English"),
            Language::Japanese => write!(f, "日本語"),
        }
    }
}

impl Resume {
    pub fn new() -> Self {
        let now = Utc::now().to_rfc3339();
        Resume {
            id: Uuid::new_v4().to_string(),
            personal_info: PersonalInfo {
                full_name: String::new(),
                email: String::new(),
                phone: None,
                location: None,
                website: None,
            },
            professional_summary: None,
            experience: Vec::new(),
            education: Vec::new(),
            skills: Vec::new(),
            metadata: ResumeMetadata {
                created_at: now.clone(),
                updated_at: now,
                version: 1,
                language: Language::English,
                template: "default".to_string(),
            },
        }
    }

    pub fn update_timestamp(&mut self) {
        self.metadata.updated_at = Utc::now().to_rfc3339();
    }
}

impl YearMonth {
    pub fn new(year: u16, month: u8) -> Self {
        YearMonth { year, month }
    }

    pub fn to_string_display(&self) -> String {
        let month_names = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun",
            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let month_name = if self.month >= 1 && (self.month as usize) <= 12 {
            month_names[(self.month - 1) as usize]
        } else {
            "---"
        };
        format!("{} {}", month_name, self.year)
    }
}

impl Default for Resume {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resume_new_has_empty_fields() {
        let resume = Resume::new();
        assert_eq!(resume.personal_info.full_name, "");
        assert_eq!(resume.personal_info.email, "");
        assert!(resume.personal_info.phone.is_none());
        assert!(resume.personal_info.location.is_none());
        assert!(resume.professional_summary.is_none());
        assert!(resume.experience.is_empty());
        assert!(resume.education.is_empty());
        assert!(resume.skills.is_empty());
    }

    #[test]
    fn test_yearmonth_display() {
        let ym = YearMonth::new(2024, 1);
        assert_eq!(ym.to_string_display(), "Jan 2024");

        let ym = YearMonth::new(2023, 12);
        assert_eq!(ym.to_string_display(), "Dec 2023");

        let ym = YearMonth::new(2024, 6);
        assert_eq!(ym.to_string_display(), "Jun 2024");
    }

    #[test]
    fn test_yearmonth_invalid_month() {
        let ym = YearMonth::new(2024, 13);
        assert_eq!(ym.to_string_display(), "--- 2024");
    }

    #[test]
    fn test_proficiency_level_display() {
        assert_eq!(ProficiencyLevel::Beginner.to_string(), "Beginner");
        assert_eq!(ProficiencyLevel::Intermediate.to_string(), "Intermediate");
        assert_eq!(ProficiencyLevel::Advanced.to_string(), "Advanced");
        assert_eq!(ProficiencyLevel::Expert.to_string(), "Expert");
    }

    #[test]
    fn test_skill_category_display() {
        assert_eq!(SkillCategory::Technical.to_string(), "Technical");
        assert_eq!(SkillCategory::Language.to_string(), "Language");
        assert_eq!(SkillCategory::SoftSkill.to_string(), "Soft Skill");
        assert_eq!(SkillCategory::Tool.to_string(), "Tool");
    }

    #[test]
    fn test_language_display() {
        assert_eq!(Language::English.to_string(), "English");
        assert_eq!(Language::Japanese.to_string(), "日本語");
    }

    #[test]
    fn test_resume_update_timestamp() {
        let mut resume = Resume::new();
        let original_updated_at = resume.metadata.updated_at.clone();

        // Small delay to ensure timestamps differ
        std::thread::sleep(std::time::Duration::from_millis(10));

        resume.update_timestamp();
        assert_ne!(resume.metadata.updated_at, original_updated_at);
    }

    #[test]
    fn test_resume_has_unique_id() {
        let resume1 = Resume::new();
        let resume2 = Resume::new();
        assert_ne!(resume1.id, resume2.id);
    }
}
