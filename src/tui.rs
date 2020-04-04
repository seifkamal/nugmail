use crate::email::Address;
use crate::generator::Service;
use crate::storage::Store;
use dialoguer::{theme::ColorfulTheme, Select};
use termion::{clear, cursor, screen, input::TermRead};
use std::io::{Write, stdin, stdout};

pub fn render_inbox<S: Store, C: Service>(address: Address, mut storage: S, client: C) {
    let updated_inbox = client.inbox(&address).unwrap();
    storage.save_inbox(&updated_inbox).unwrap();
    if updated_inbox.size() == 0 {
        return println!("Inbox is empty");
    }

    let mut screen = screen::AlternateScreen::from(stdout());
    let inbox = storage.inbox(&address).unwrap();

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
            }
            None => break
        }
    }

    write!(screen, "{}", cursor::Show).unwrap()
}

fn clear_screen<W: Write>(screen: &mut screen::AlternateScreen<W>) {
    write!(screen, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide).unwrap();
    screen.flush().unwrap();
}
