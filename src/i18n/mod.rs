use crate::data::Language;

pub struct I18n {
    language: Language,
}

impl I18n {
    pub fn new(language: Language) -> Self {
        I18n { language }
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn get_string(&self, key: &str) -> String {
        match self.language {
            Language::English => self.get_en(key),
            Language::Japanese => self.get_ja(key),
        }
    }

    fn get_en(&self, key: &str) -> String {
        match key {
            "app.title" => "Resume Creator",
            "menu.new" => "New Resume",
            "menu.open" => "Open Resume",
            "menu.export" => "Export",
            "menu.edit" => "Edit",
            "menu.delete" => "Delete",
            "menu.add" => "Add",
            "menu.save" => "Save",
            "menu.quit" => "Quit",
            "section.personal" => "Personal Information",
            "section.summary" => "Professional Summary",
            "section.experience" => "Work Experience",
            "section.education" => "Education",
            "section.skills" => "Skills",
            "field.name" => "Full Name",
            "field.email" => "Email",
            "field.phone" => "Phone",
            "field.location" => "Location",
            "field.company" => "Company",
            "field.position" => "Position",
            "field.degree" => "Degree",
            "field.institution" => "Institution",
            "skill.name" => "Skill Name",
            "skill.proficiency" => "Proficiency",
            "unsaved_changes" => "You have unsaved changes",
            "export.success" => "Resume exported successfully",
            "export.error" => "Failed to export resume",
            _ => key,
        }
        .to_string()
    }

    fn get_ja(&self, key: &str) -> String {
        match key {
            "app.title" => "レジュメ クリエーター",
            "menu.new" => "新規作成",
            "menu.open" => "開く",
            "menu.export" => "エクスポート",
            "menu.edit" => "編集",
            "menu.delete" => "削除",
            "menu.add" => "追加",
            "menu.save" => "保存",
            "menu.quit" => "終了",
            "section.personal" => "個人情報",
            "section.summary" => "プロフェッショナル概要",
            "section.experience" => "職務経歴",
            "section.education" => "教育",
            "section.skills" => "スキル",
            "field.name" => "名前",
            "field.email" => "メール",
            "field.phone" => "電話",
            "field.location" => "所在地",
            "field.company" => "会社",
            "field.position" => "職位",
            "field.degree" => "学位",
            "field.institution" => "機関",
            "skill.name" => "スキル名",
            "skill.proficiency" => "習得度",
            "unsaved_changes" => "保存されていない変更があります",
            "export.success" => "レジュメが正常にエクスポートされました",
            "export.error" => "レジュメのエクスポートに失敗しました",
            _ => key,
        }
        .to_string()
    }
}
