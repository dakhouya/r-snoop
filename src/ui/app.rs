use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use crate::capture::{Sniffer, device_info::DeviceInfo};

const INPUT_POLLING_MS: u64 = 100;

pub struct App {
    should_quit: bool,
    interface: String,
    devices: Vec<DeviceInfo>,
    rx: Receiver<DeviceInfo>,
}

impl App {
    pub fn new(interface: String) -> Self {
        let (tx, rx) = mpsc::channel();

        // Spawn sniffer thread
        let sniffer_interface = interface.clone();
        thread::spawn(move || {
            let sniffer = Sniffer::new(&sniffer_interface);
            if let Err(e) = sniffer.run_with_channel(tx) {
                eprintln!("Sniffer error: {}", e);
            }
        });

        Self {
            should_quit: false,
            interface,
            devices: Vec::new(),
            rx,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_loop(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            // Check for new devices from sniffer
            while let Ok(device) = self.rx.try_recv() {
                self.devices.push(device);
            }

            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3),
                    ])
                    .split(f.area());

                // Header
                let header = Paragraph::new(vec![Line::from(vec![
                    Span::styled(
                        "r-snoop",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" - Passive Network Scanner"),
                ])])
                .block(Block::default().borders(Borders::ALL));
                f.render_widget(header, chunks[0]);

                // Main content - devices list
                let title = format!(
                    "Discovered Devices ({}) - Interface: {}",
                    self.devices.len(),
                    self.interface
                );

                if self.devices.is_empty() {
                    let content = Paragraph::new(vec![
                        Line::from(""),
                        Line::from("Listening for network traffic..."),
                        Line::from(""),
                        Line::from(Span::styled(
                            "No devices discovered yet",
                            Style::default().fg(Color::DarkGray),
                        )),
                    ])
                    .block(Block::default().borders(Borders::ALL).title(title));
                    f.render_widget(content, chunks[1]);
                } else {
                    let items: Vec<ListItem> = self
                        .devices
                        .iter()
                        .map(|device| {
                            let mac_str = device
                                .mac_addr()
                                .map(|m| {
                                    format!(
                                        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                                        m[0], m[1], m[2], m[3], m[4], m[5]
                                    )
                                })
                                .unwrap_or_else(|| "unknown".to_string());

                            let ipv4_str = if device.ipv4().is_empty() {
                                String::new()
                            } else {
                                device
                                    .ipv4()
                                    .iter()
                                    .map(|ip| format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            };

                            let display =
                                format!("● {} - {} - {}", ipv4_str, mac_str, device.mac_vendor());

                            ListItem::new(Line::from(Span::styled(
                                display,
                                Style::default().fg(Color::Green),
                            )))
                        })
                        .collect();

                    let list =
                        List::new(items).block(Block::default().borders(Borders::ALL).title(title));
                    f.render_widget(list, chunks[1]);
                }

                // Footer
                let footer = Paragraph::new(Line::from(vec![
                    Span::styled("q", Style::default().fg(Color::Yellow)),
                    Span::raw(": quit"),
                ]))
                .block(Block::default().borders(Borders::ALL));
                f.render_widget(footer, chunks[2]);
            })?;

            // Handle input
            if event::poll(std::time::Duration::from_millis(INPUT_POLLING_MS))?
                && let Event::Key(key) = event::read()?
            {
                #[allow(clippy::single_match)]
                match key.code {
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                    }
                    _ => {}
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }
}
