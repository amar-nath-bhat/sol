use super::terminal::{Position, Size, Terminal};
use std::io::Error;

mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { col: 0, row: at })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        let Size { height, width } = self.size;

        if height == 0 || width == 0 {
            return Ok(());
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(row, truncated_line)?;
            } else if row == vertical_center && self.buffer.is_empty() {
                Self::render_line(row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(row, "~")?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

    pub fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let welcome_msg = format!("Welcome to {} -- Version {}!", NAME, VERSION);
        let len = welcome_msg.len();

        if width <= len {
            return "~".to_string();
        }

        #[allow(clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let mut full_msg = format!("~{}{}", " ".repeat(padding), welcome_msg);
        full_msg.truncate(width);
        full_msg
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
