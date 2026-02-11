mod app;
mod data;
mod export;
mod i18n;
mod storage;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    // Initialize app and run
    let app = App::new()?;
    let result = run_app(terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    result
}

fn run_app<B: Backend>(mut terminal: Terminal<B>, mut app: App) -> Result<()> {
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use std::time::Duration;

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if app.should_quit {
            break;
        }

        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(KeyEvent {
                code,
                modifiers,
                ..
            }) = event::read()?
            {
                // Handle Ctrl+C first, works even in edit mode
                if code == KeyCode::Char('c') && modifiers == KeyModifiers::CONTROL {
                    app.should_quit = true;
                    continue;
                }

                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::NONE) if !app.editing_field => {
                        if app.unsaved_changes {
                            app.show_quit_confirmation = true;
                        } else {
                            app.should_quit = true;
                        }
                    }
                    (KeyCode::Char('y'), KeyModifiers::NONE) if app.show_quit_confirmation => {
                        app.should_quit = true;
                    }
                    (KeyCode::Esc, KeyModifiers::NONE) if app.show_quit_confirmation => {
                        app.show_quit_confirmation = false;
                    }
                    (KeyCode::Esc, KeyModifiers::NONE) if app.editing_field => {
                        app.finish_editing();
                    }
                    (KeyCode::Char('s'), KeyModifiers::NONE) if !app.editing_field && app.current_screen != app::Screen::SaveAs && app.current_screen != app::Screen::ExportPath => {
                        let default_path = app::App::compute_default_export_path(&app.resume.personal_info.full_name);
                        app.export_path = default_path;
                        app.toggle_screen("save_as");
                    }
                    (KeyCode::Char('e'), KeyModifiers::NONE) if !app.editing_field => {
                        app.toggle_screen("export");
                    }
                    (KeyCode::Char('l'), KeyModifiers::NONE) if !app.editing_field => {
                        app.toggle_language();
                    }
                    (KeyCode::Up, KeyModifiers::NONE) | (KeyCode::Char('k'), KeyModifiers::NONE) if !app.editing_field => {
                        app.move_selection_up();
                    }
                    (KeyCode::Down, KeyModifiers::NONE)
                    | (KeyCode::Char('j'), KeyModifiers::NONE) if !app.editing_field => {
                        app.move_selection_down();
                    }
                    (KeyCode::Enter, KeyModifiers::NONE) if !app.editing_field => {
                        app.handle_enter();
                    }
                    (KeyCode::Char('i'), KeyModifiers::NONE) if !app.editing_field => {
                        app.toggle_edit_mode();
                    }
                    (KeyCode::Char('a'), KeyModifiers::NONE) if !app.editing_field => {
                        app.add_to_current_section();
                    }
                    (KeyCode::Char('d'), KeyModifiers::NONE) if !app.editing_field => {
                        app.delete_current_item();
                    }
                    (KeyCode::Tab, KeyModifiers::NONE) if !app.editing_field => {
                        app.move_to_next_section();
                    }
                    (KeyCode::BackTab, KeyModifiers::SHIFT) if !app.editing_field => {
                        app.move_to_prev_section();
                    }
                    _ if app.editing_field || app.current_screen == app::Screen::Import || app.current_screen == app::Screen::SaveAs || app.current_screen == app::Screen::ExportPath => {
                        app.handle_text_input(code);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
