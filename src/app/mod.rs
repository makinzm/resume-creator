use crate::data::*;
use crate::storage;
use crate::i18n::I18n;
use anyhow::Result;
use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Home,
    PersonalInfo,
    Summary,
    Experience,
    Education,
    Skills,
    Export,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub resume: Resume,
    pub current_screen: Screen,
    pub current_section: Section,
    pub selected_index: usize,
    pub field_index: usize,
    pub input_mode: InputMode,
    pub unsaved_changes: bool,
    pub show_quit_confirmation: bool,
    pub editing_field: bool,
    pub edit_text: String,
    pub export_format: ExportFormat,
    pub status_message: Option<String>,
    pub i18n: I18n,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Section {
    Personal,
    Summary,
    Experience,
    Education,
    Skills,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Markdown,
    Json,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(App {
            resume: Resume::new(),
            current_screen: Screen::Home,
            current_section: Section::Personal,
            selected_index: 0,
            field_index: 0,
            input_mode: InputMode::Normal,
            unsaved_changes: false,
            show_quit_confirmation: false,
            editing_field: false,
            edit_text: String::new(),
            export_format: ExportFormat::Markdown,
            status_message: None,
            i18n: I18n::new(Language::English),
        })
    }

    pub fn toggle_screen(&mut self, screen: &str) {
        self.current_screen = match screen {
            "home" => Screen::Home,
            "personal" => {
                self.current_section = Section::Personal;
                Screen::PersonalInfo
            }
            "summary" => {
                self.current_section = Section::Summary;
                Screen::Summary
            }
            "experience" => {
                self.current_section = Section::Experience;
                Screen::Experience
            }
            "education" => {
                self.current_section = Section::Education;
                Screen::Education
            }
            "skills" => {
                self.current_section = Section::Skills;
                Screen::Skills
            }
            "export" => Screen::Export,
            _ => self.current_screen,
        };
        self.selected_index = 0;
        self.field_index = 0;
        self.editing_field = false;
        self.edit_text.clear();
    }

    pub fn toggle_language(&mut self) {
        self.resume.metadata.language = match self.resume.metadata.language {
            Language::English => Language::Japanese,
            Language::Japanese => Language::English,
        };
        self.i18n.set_language(self.resume.metadata.language);
        self.unsaved_changes = true;
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        let max = match self.current_section {
            Section::Experience => self.resume.experience.len(),
            Section::Education => self.resume.education.len(),
            Section::Skills => self.resume.skills.len(),
            _ => 1,
        };
        if self.selected_index < max {
            self.selected_index += 1;
        }
    }

    pub fn handle_enter(&mut self) {
        match self.current_screen {
            Screen::Home => {
                if self.selected_index == 0 {
                    self.toggle_screen("personal");
                } else if self.selected_index == 1 {
                    self.toggle_screen("experience");
                } else if self.selected_index == 2 {
                    self.status_message = Some("Load Resume not yet implemented in UI".to_string());
                } else if self.selected_index == 3 {
                    self.toggle_screen("export");
                }
            }
            Screen::Export => {
                self.handle_export();
                self.toggle_screen("home");
            }
            _ => {
                self.toggle_edit_mode();
            }
        }
    }

    pub fn toggle_edit_mode(&mut self) {
        if !self.editing_field {
            self.editing_field = true;
            self.edit_text.clear();
            self.load_current_field_text();
        } else {
            self.apply_edit_field();
            self.move_to_next_field();
        }
    }

    pub fn finish_editing(&mut self) {
        if self.editing_field {
            self.apply_edit_field();
            self.editing_field = false;
            self.field_index = 0;
            self.edit_text.clear();
        }
    }

    fn load_current_field_text(&mut self) {
        self.edit_text.clear();
        match self.current_section {
            Section::Personal => {
                match self.field_index {
                    0 => self.edit_text = self.resume.personal_info.full_name.clone(),
                    1 => self.edit_text = self.resume.personal_info.email.clone(),
                    2 => self.edit_text = self.resume.personal_info.phone.clone().unwrap_or_default(),
                    3 => self.edit_text = self.resume.personal_info.location.clone().unwrap_or_default(),
                    4 => self.edit_text = self.resume.personal_info.website.clone().unwrap_or_default(),
                    _ => {}
                }
            }
            Section::Experience => {
                if let Some(exp) = self.resume.experience.get(self.selected_index) {
                    match self.field_index {
                        0 => self.edit_text = exp.company.clone(),
                        1 => self.edit_text = exp.position.clone(),
                        2 => self.edit_text = exp.start_date.year.to_string(),
                        3 => self.edit_text = exp.end_date.as_ref().map(|d| d.year.to_string()).unwrap_or_default(),
                        4 => self.edit_text = exp.description.clone(),
                        _ => {}
                    }
                }
            }
            Section::Education => {
                if let Some(edu) = self.resume.education.get(self.selected_index) {
                    match self.field_index {
                        0 => self.edit_text = edu.institution.clone(),
                        1 => self.edit_text = edu.degree.clone(),
                        2 => self.edit_text = edu.field_of_study.clone(),
                        3 => self.edit_text = edu.graduation_date.year.to_string(),
                        4 => self.edit_text = edu.gpa.clone().unwrap_or_default(),
                        _ => {}
                    }
                }
            }
            Section::Skills => {
                if let Some(skill) = self.resume.skills.get(self.selected_index) {
                    match self.field_index {
                        0 => self.edit_text = skill.name.clone(),
                        _ => {}
                    }
                }
            }
            Section::Summary => {
                self.edit_text = self.resume.professional_summary.clone().unwrap_or_default();
            }
        }
    }

    fn apply_edit_field(&mut self) {
        match self.current_section {
            Section::Personal => {
                match self.field_index {
                    0 => self.resume.personal_info.full_name = self.edit_text.clone(),
                    1 => self.resume.personal_info.email = self.edit_text.clone(),
                    2 => {
                        self.resume.personal_info.phone = if self.edit_text.is_empty() {
                            None
                        } else {
                            Some(self.edit_text.clone())
                        };
                    }
                    3 => {
                        self.resume.personal_info.location = if self.edit_text.is_empty() {
                            None
                        } else {
                            Some(self.edit_text.clone())
                        };
                    }
                    4 => {
                        self.resume.personal_info.website = if self.edit_text.is_empty() {
                            None
                        } else {
                            Some(self.edit_text.clone())
                        };
                    }
                    _ => {}
                }
            }
            Section::Summary => {
                self.resume.professional_summary = if self.edit_text.is_empty() {
                    None
                } else {
                    Some(self.edit_text.clone())
                };
            }
            Section::Experience => {
                if let Some(exp) = self.resume.experience.get_mut(self.selected_index) {
                    match self.field_index {
                        0 => exp.company = self.edit_text.clone(),
                        1 => exp.position = self.edit_text.clone(),
                        2 => {
                            if let Ok(year) = self.edit_text.parse::<u16>() {
                                exp.start_date.year = year;
                            }
                        }
                        3 => {
                            if let Ok(year) = self.edit_text.parse::<u16>() {
                                if let Some(end) = &mut exp.end_date {
                                    end.year = year;
                                }
                            }
                        }
                        4 => exp.description = self.edit_text.clone(),
                        _ => {}
                    }
                }
            }
            Section::Education => {
                if let Some(edu) = self.resume.education.get_mut(self.selected_index) {
                    match self.field_index {
                        0 => edu.institution = self.edit_text.clone(),
                        1 => edu.degree = self.edit_text.clone(),
                        2 => edu.field_of_study = self.edit_text.clone(),
                        3 => {
                            if let Ok(year) = self.edit_text.parse::<u16>() {
                                edu.graduation_date.year = year;
                            }
                        }
                        4 => {
                            edu.gpa = if self.edit_text.is_empty() {
                                None
                            } else {
                                Some(self.edit_text.clone())
                            };
                        }
                        _ => {}
                    }
                }
            }
            Section::Skills => {
                if let Some(skill) = self.resume.skills.get_mut(self.selected_index) {
                    match self.field_index {
                        0 => skill.name = self.edit_text.clone(),
                        _ => {}
                    }
                }
            }
        }
        self.unsaved_changes = true;
    }

    fn move_to_next_field(&mut self) {
        let max_fields = match self.current_section {
            Section::Personal => 5,
            Section::Summary => 1,
            Section::Experience => 5,
            Section::Education => 5,
            Section::Skills => 1,
        };

        self.field_index += 1;
        if self.field_index >= max_fields {
            self.field_index = 0;
            self.editing_field = false;
            self.edit_text.clear();
        } else {
            self.load_current_field_text();
        }
    }

    pub fn add_to_current_section(&mut self) {
        match self.current_section {
            Section::Experience => {
                self.resume.experience.push(Experience {
                    id: uuid::Uuid::new_v4().to_string(),
                    company: String::new(),
                    position: String::new(),
                    start_date: YearMonth::new(2024, 1),
                    end_date: None,
                    is_current: false,
                    description: String::new(),
                    location: None,
                });
                self.unsaved_changes = true;
            }
            Section::Education => {
                self.resume.education.push(Education {
                    id: uuid::Uuid::new_v4().to_string(),
                    institution: String::new(),
                    degree: String::new(),
                    field_of_study: String::new(),
                    graduation_date: YearMonth::new(2024, 5),
                    gpa: None,
                });
                self.unsaved_changes = true;
            }
            Section::Skills => {
                self.resume.skills.push(Skill {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: String::new(),
                    proficiency: ProficiencyLevel::Beginner,
                    category: SkillCategory::Technical,
                });
                self.unsaved_changes = true;
            }
            _ => {}
        }
    }

    pub fn delete_current_item(&mut self) {
        match self.current_section {
            Section::Experience => {
                if self.selected_index < self.resume.experience.len() {
                    self.resume.experience.remove(self.selected_index);
                    self.unsaved_changes = true;
                }
            }
            Section::Education => {
                if self.selected_index < self.resume.education.len() {
                    self.resume.education.remove(self.selected_index);
                    self.unsaved_changes = true;
                }
            }
            Section::Skills => {
                if self.selected_index < self.resume.skills.len() {
                    self.resume.skills.remove(self.selected_index);
                    self.unsaved_changes = true;
                }
            }
            _ => {}
        }
    }

    pub fn move_to_next_section(&mut self) {
        match self.current_section {
            Section::Personal => self.toggle_screen("summary"),
            Section::Summary => self.toggle_screen("experience"),
            Section::Experience => self.toggle_screen("education"),
            Section::Education => self.toggle_screen("skills"),
            Section::Skills => self.toggle_screen("home"),
        }
    }

    pub fn move_to_prev_section(&mut self) {
        match self.current_section {
            Section::Personal => self.toggle_screen("home"),
            Section::Summary => self.toggle_screen("personal"),
            Section::Experience => self.toggle_screen("summary"),
            Section::Education => self.toggle_screen("experience"),
            Section::Skills => self.toggle_screen("education"),
        }
    }

    pub fn handle_text_input(&mut self, key: KeyCode) {
        if !self.editing_field {
            return;
        }
        match key {
            KeyCode::Char(c) => self.edit_text.push(c),
            KeyCode::Backspace => {
                self.edit_text.pop();
            }
            KeyCode::Tab => {
                self.toggle_edit_mode();
            }
            KeyCode::Esc => {
                self.finish_editing();
            }
            _ => {}
        }
    }

    pub fn save_resume(&mut self) -> Result<()> {
        self.resume.update_timestamp();
        storage::save_resume(&self.resume)?;
        self.unsaved_changes = false;
        Ok(())
    }

    fn handle_export(&mut self) {
        use crate::export;

        let format = if self.selected_index == 0 { "markdown" } else { "json" };

        match export::save_export_to_file(&self.resume, format) {
            Ok(path) => {
                self.status_message = Some(format!("Exported to: {}", path));
            }
            Err(e) => {
                self.status_message = Some(format!("Export failed: {}", e));
            }
        }
    }

    pub fn load_resume(&mut self, id: &str) -> Result<()> {
        self.resume = storage::load_resume(id)?;
        self.unsaved_changes = false;
        Ok(())
    }

    pub fn list_resumes(&self) -> Result<Vec<Resume>> {
        storage::list_resumes()
    }

    pub fn get_language_str(&self) -> &str {
        match self.resume.metadata.language {
            Language::English => "en",
            Language::Japanese => "ja",
        }
    }

    pub fn t(&self, key: &str) -> String {
        self.i18n.get_string(key)
    }

    pub fn get_home_menu_items(&self) -> Vec<String> {
        vec![
            self.t("menu.edit_personal"),
            self.t("menu.add_experience"),
            self.t("menu.load_resume"),
            self.t("menu.export_resume"),
            self.t("menu.exit"),
        ]
    }
}
