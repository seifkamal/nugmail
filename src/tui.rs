use dialoguer::{Select, theme::ColorfulTheme};
use std::io::{stdout, Write};
use termion::{clear, cursor, screen};

use nugmail::{
    email::Address,
    generator::Service,
    storage::Store,
};

pub fn render_inbox<S: Store, C: Service>(address: Address, mut storage: S, client: C) {
    let inbox = client.inbox(&address).unwrap();
    storage.save_inbox(&inbox).unwrap();
    if inbox.size() == 0 {
        return println!("Inbox is empty");
    }

    loop {
        let mut screen = screen::AlternateScreen::from(stdout());
        write!(screen, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide).unwrap();
        screen.flush().unwrap();

        let theme = &ColorfulTheme::default();
        let mut select = Select::with_theme(theme);
        for message in inbox.messages() {
            select.item(&format!("{}: {}", message.sender(), message.subject().unwrap()));
        }

        let selection = select.with_prompt(&format!("{} messages", inbox.size()))
            .default(0)
            .interact_opt()
            .unwrap();

        write!(screen, "{}{}", cursor::Show, screen::ToMainScreen).unwrap();

        match selection {
            Some(index) => {
                let tmp_file = "nug.msg";

                std::fs::File::create(tmp_file)
                    .unwrap()
                    .write_fmt(format_args!("{}", inbox.messages().get(index).unwrap()))
                    .unwrap();

                std::process::Command::new("less")
                    .arg(tmp_file)
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                std::fs::remove_file(tmp_file).unwrap();
            }
            None => break
        }
    }
}
