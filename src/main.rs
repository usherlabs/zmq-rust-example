#[cfg(test)]
mod prover_test;

use std::io::{self};

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use zmq_rust_example::{create_proof, publish_binaries};

struct KeyEventIterator {
    should_continue: bool,
}

impl Iterator for KeyEventIterator {
    type Item = event::Event;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.should_continue {
            return None;
        }

        let ev = event::read().unwrap();
        match &ev {
            event::Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                self.should_continue = false;
            }
            _ => {}
        }
        Some(ev)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All)
    )?;
    terminal::enable_raw_mode()?;

    let key_event_iterator = KeyEventIterator {
        should_continue: true,
    };

    // create a socket at ./test_data
    let socket = zmq_rust_example::create_socket("test_data", zmq::PUB);
    let mut server = zmq_rust_example::SubscriptionServer::new(socket);

    let mut publish_n = 0;

    for event in key_event_iterator {
        match event {
            event::Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                terminal::disable_raw_mode().unwrap();
                let data = format!("test_data {}", publish_n);
                let data_binary = create_proof(&data);
                println!("sending {:?}", data_binary);
                server.subscribe_to_proofs(data_binary)?;
                terminal::enable_raw_mode().unwrap();
                publish_n += 1;
            }
            _ => {}
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
