mod serialize;
mod tree;

use crossterm::event::{self, Event, KeyCode};
// use log::{debug, info, trace, warn};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{
        Block, BorderType, List, ListDirection, ListState, Paragraph, StatefulWidget, Widget,
    },
};

struct ConfigEditor {
    pub list_state: ListState,
}

impl ConfigEditor {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
        }
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Text::raw("Press q to quit; arrow keys to navigate; space or enter to select")
            .on_white()
            .black()
            .bold()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items = ["Item 1", "Item 2", "Item 3"];
        let list = List::new(items)
            .block(Block::bordered().title("List"))
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        StatefulWidget::render(list, area, buf, &mut self.list_state);
    }
}

impl Widget for &mut ConfigEditor {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

        let [left_area, _right_area] =
            Layout::horizontal([Constraint::Length(25), Constraint::Fill(1)]).areas(main_area);

        self.render_list(left_area, buf);
        ConfigEditor::render_footer(footer_area, buf)
    }
}

/// Run the configuration editor. This function runs when the "config" subcommand is provided.
///
/// The config editor will be a TUI application made with ratatui.
pub fn run_config_editor() {
    let mut terminal = ratatui::init();

    let mut config_editor = ConfigEditor::new();
    loop {
        terminal
            .draw(|frame| {
                config_editor.render(frame.area(), frame.buffer_mut());
                // let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
                // let [main_area, footer] = vertical.areas(frame.area());

                // // Draw the footer
                // Text::raw("Press q to quit; arrow keys to navigate; space or enter to select")/*.on_white()*/.render(footer, frame.buffer_mut());

                // let horizontal = Layout::horizontal([Constraint::Length(25), Constraint::Fill(1)]);
                // let [left_area, right_area] = horizontal.areas(main_area);

                // let info = Paragraph::new("This is the configuration editor for mailroom. You can also change mailroom's configuration by directly editing config.toml.");

                // let block = Block::bordered().border_type(BorderType::Thick);

                // frame.render_widget(list, right_area);
                // frame.render_widget(info, left_area);
            })
            .expect("failed to draw frame");

        // Handle terminal events
        match event::read().expect("failed to read terminal event") {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Char('q') => {
                        // TODO: Save the configuration
                        // Exit the application
                        break;
                    }
                    KeyCode::Up => {
                        config_editor.list_state.select_previous();
                    }
                    KeyCode::Down => {
                        config_editor.list_state.select_next();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // Restore the terminal to its original state
    ratatui::restore();
}
