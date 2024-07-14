use super::terminal::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View;

impl View {
    pub fn render() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        Terminal::clear_line()?;
        Terminal::print("Hello, World!\r\n")?;
        for row in 1..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if row == height / 3 {
                Self::print_welcome_msg()?;
            } else {
                Self::draw_empty_row()?;
            }
            if row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn print_welcome_msg() -> Result<(), Error> {
        let mut welcom_msg = format!("Welcome to {} -- Version {}!\r\n", NAME, VERSION);
        let width = Terminal::size()?.width as usize;
        let len = welcom_msg.len();

        #[allow(clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcom_msg = format!("~{}{}", spaces, welcom_msg);
        welcom_msg.truncate(width);
        Terminal::print(&welcom_msg)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
