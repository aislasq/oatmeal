use anyhow::Result;
use crossterm::event::Event as CrosstermEvent;
use crossterm::event::EventStream;
use futures::StreamExt;
use tokio::sync::mpsc;
use tui_textarea::Input;
use tui_textarea::Key;

use crate::domain::models::Event;

pub struct EventsService {
    crossterm_events: EventStream,
    events: mpsc::UnboundedReceiver<Event>,
}

impl EventsService {
    pub fn new(events: mpsc::UnboundedReceiver<Event>) -> EventsService {
        return EventsService {
            crossterm_events: EventStream::new(),
            events,
        };
    }

    fn handle_crossterm(&self, event: CrosstermEvent) -> Option<Event> {
        match event {
            CrosstermEvent::Resize(_, _) => {
                return Some(Event::UIResize());
            }
            CrosstermEvent::Key(keyevent) => {
                match keyevent.into() {
                    Input { key: Key::Down, .. } => {
                        return Some(Event::UIScrollDown());
                    }
                    Input { key: Key::Up, .. } => {
                        return Some(Event::UIScrollUp());
                    }
                    Input {
                        key: Key::Char('d'),
                        ctrl: true,
                        ..
                    } => {
                        return Some(Event::UIScrollPageDown());
                    }
                    Input {
                        key: Key::Char('u'),
                        ctrl: true,
                        ..
                    } => {
                        return Some(Event::UIScrollPageUp());
                    }
                    Input {
                        key: Key::Char('c'),
                        ctrl: true,
                        ..
                    } => {
                        return Some(Event::KeyboardCTRLC());
                    }
                    Input {
                        key: Key::Char('r'),
                        ctrl: true,
                        ..
                    } => {
                        return Some(Event::KeyboardCTRLR());
                    }
                    Input {
                        key: Key::Enter, ..
                    } => {
                        return Some(Event::KeyboardEnter());
                    }
                    input => {
                        return Some(Event::KeyboardCharInput(input));
                    }
                }
            }
            _ => return None,
        }
    }

    pub async fn next(&mut self) -> Result<Event> {
        loop {
            let evt = tokio::select! {
                event = self.events.recv() => event,
                event = self.crossterm_events.next() => match event {
                    Some(Ok(input)) => self.handle_crossterm(input),
                    Some(Err(_)) => None,
                    None => None
                },
            };

            if let Some(event) = evt {
                return Ok(event);
            }
        }
    }
}
