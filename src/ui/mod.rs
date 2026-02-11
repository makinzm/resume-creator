use crate::app::{App, Screen};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();

    match app.current_screen {
        Screen::Home => draw_home(f, app, size),
        Screen::PersonalInfo => draw_personal_info(f, app, size),
        Screen::Summary => draw_summary(f, app, size),
        Screen::Experience => draw_experience(f, app, size),
        Screen::Education => draw_education(f, app, size),
        Screen::Skills => draw_skills(f, app, size),
        Screen::Export => draw_export(f, app, size),
    }

    if app.show_quit_confirmation {
        draw_quit_confirmation(f, app, size);
    }
}

fn draw_home(f: &mut Frame, app: &App, size: Rect) {
    let title = format!("{} - {}", app.t("app.title"), app.t("menu.home"));

    let menu_items = app.get_home_menu_items();
    let items: Vec<ListItem> = menu_items
        .iter()
        .map(|item| ListItem::new(item.clone()))
        .collect();

    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("> ");

    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(app.selected_index));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(size);

    let unsaved_str = if app.unsaved_changes { "Yes" } else { "No" };
    let header = Paragraph::new(format!(
        "{} {} | {}: {}",
        app.t("message.language"),
        app.resume.metadata.language,
        app.t("message.unsaved"),
        unsaved_str
    ));
    f.render_widget(header, chunks[0]);
    f.render_stateful_widget(list, chunks[1], &mut list_state);
}

fn draw_personal_info(f: &mut Frame, app: &App, size: Rect) {
    let title = app.t("section.personal");

    let mut content = String::new();
    content.push_str(&format!("{}: {}\n", app.t("field.name"), app.resume.personal_info.full_name));
    content.push_str(&format!("{}: {}\n", app.t("field.email"), app.resume.personal_info.email));
    content.push_str(&format!(
        "{}: {}\n",
        app.t("field.phone"),
        app.resume.personal_info.phone.as_ref().unwrap_or(&"N/A".to_string())
    ));
    content.push_str(&format!(
        "{}: {}\n",
        app.t("field.location"),
        app.resume.personal_info.location.as_ref().unwrap_or(&"N/A".to_string())
    ));

    if app.editing_field {
        content.push_str("\n[");
        content.push_str(&app.t("action.press_i_edit"));
        content.push_str("] ");
        content.push_str(&app.edit_text);
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    let block = Block::default().title(title).borders(Borders::ALL);
    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, chunks[0]);

    let footer_text = format!(
        "{} | {} | {}",
        app.t("action.press_i_edit"),
        app.t("action.press_tab_next"),
        app.t("message.quit_key")
    );
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[1]);
}

fn draw_summary(f: &mut Frame, app: &App, size: Rect) {
    let title = app.t("section.summary");

    let summary = app.resume.professional_summary.as_ref().unwrap_or(&"".to_string()).clone();
    let mut content = summary.clone();

    if app.editing_field {
        content.push_str("\n[");
        content.push_str(&app.t("action.press_i_edit"));
        content.push_str("] ");
        content.push_str(&app.edit_text);
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    let block = Block::default().title(title).borders(Borders::ALL);
    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, chunks[0]);

    let footer_text = format!(
        "{} | {}",
        app.t("action.press_i_edit"),
        app.t("action.press_tab_next")
    );
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[1]);
}

fn draw_experience(f: &mut Frame, app: &App, size: Rect) {
    let title = app.t("section.experience");

    let mut content = String::new();
    if app.resume.experience.is_empty() {
        content.push_str(&app.t("action.press_a_add"));
    } else {
        for (idx, exp) in app.resume.experience.iter().enumerate() {
            let marker = if idx == app.selected_index { ">" } else { " " };
            content.push_str(&format!(
                "{} {} at {} ({} - {})\n",
                marker,
                exp.position,
                exp.company,
                exp.start_date.to_string_display(),
                exp.end_date.as_ref().map(|d| d.to_string_display()).unwrap_or_else(|| "Present".to_string())
            ));
        }
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(size);

    let block = Block::default().title(title).borders(Borders::ALL);
    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, chunks[0]);

    let footer_text = app.t("action.edit_mode_help");
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[1]);
}

fn draw_education(f: &mut Frame, app: &App, size: Rect) {
    let title = app.t("section.education");

    let mut content = String::new();
    if app.resume.education.is_empty() {
        content.push_str(&app.t("action.press_a_add"));
    } else {
        for (idx, edu) in app.resume.education.iter().enumerate() {
            let marker = if idx == app.selected_index { ">" } else { " " };
            content.push_str(&format!(
                "{} {} - {} ({})\n  {}\n",
                marker,
                edu.degree,
                edu.field_of_study,
                edu.institution,
                edu.graduation_date.to_string_display()
            ));
        }
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(size);

    let block = Block::default().title(title).borders(Borders::ALL);
    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, chunks[0]);

    let footer_text = app.t("action.edit_mode_help");
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[1]);
}

fn draw_skills(f: &mut Frame, app: &App, size: Rect) {
    let title = app.t("section.skills");

    let mut content = String::new();
    if app.resume.skills.is_empty() {
        content.push_str(&app.t("action.press_a_add"));
    } else {
        for (idx, skill) in app.resume.skills.iter().enumerate() {
            let marker = if idx == app.selected_index { ">" } else { " " };
            content.push_str(&format!(
                "{} {} ({})\n",
                marker,
                skill.name,
                skill.proficiency
            ));
        }
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(size);

    let block = Block::default().title(title).borders(Borders::ALL);
    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, chunks[0]);

    let footer_text = app.t("action.edit_mode_help");
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[1]);
}

fn draw_export(f: &mut Frame, app: &App, size: Rect) {
    let title = app.t("menu.export_resume");

    let formats = vec![
        app.t("export.format_markdown"),
        app.t("export.format_json"),
    ];
    let items: Vec<ListItem> = formats
        .iter()
        .map(|format| ListItem::new(format.clone()))
        .collect();

    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("> ");

    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(app.selected_index));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let footer_text = app.t("export.press_enter");
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[1]);
}

fn draw_quit_confirmation(f: &mut Frame, app: &App, size: Rect) {
    let block = Block::default()
        .title(app.t("action.unsaved_changes"))
        .borders(Borders::ALL);
    let content = Paragraph::new(app.t("action.save_prompt"))
        .block(block)
        .alignment(Alignment::Center);

    let popup_width = 40;
    let popup_height = 5;
    let x = (size.width.saturating_sub(popup_width)) / 2;
    let y = (size.height.saturating_sub(popup_height)) / 2;

    let popup = Rect {
        x,
        y,
        width: popup_width,
        height: popup_height,
    };

    f.render_widget(Clear, popup);
    f.render_widget(content, popup);
}
