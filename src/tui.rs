use crate::email::Inbox;
use dialoguer::{theme::ColorfulTheme, Select};
use termion::{clear, cursor, screen, input::TermRead};
use std::io::{Write, stdin, stdout};

pub fn render_inbox(inbox: &Inbox) {
    let mut screen = screen::AlternateScreen::from(stdout());

    loop {
        clear_screen(&mut screen);

        let theme = &ColorfulTheme::default();
        let mut select = Select::with_theme(theme);
        for message in inbox.messages() {
            select.item(&format!("{}: {}", message.sender(), message.subject().unwrap()));
        }

        let selection = select.with_prompt(&format!("{} messages", inbox.size()))
            .default(0)
            .paged(true)
            .interact_opt()
            .unwrap();

        match selection {
            Some(index) => {
                clear_screen(&mut screen);
                println!("{}", inbox.messages().get(index).unwrap());

                for c in stdin().keys() {
                    if c.is_ok() {
                        break;
                    }
                }
            },
            None => break
        }
    }

    write!(screen, "{}", cursor::Show).unwrap()
}

fn clear_screen<W: Write>(screen: &mut screen::AlternateScreen<W>) {
    write!(screen, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide).unwrap();
    screen.flush().unwrap();
}
