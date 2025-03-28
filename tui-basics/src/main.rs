use std::{io, thread, time::Duration};
use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph, Table, Row, Cell, Chart, Axis, Dataset},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
    symbols,
};
use rand::Rng;

struct Workflow {
    id: usize,
    name: &'static str,
    status: &'static str,
}

fn get_workflows() -> Vec<Workflow> {
    vec![
        Workflow { id: 1, name: "Genome Analysis", status: "Running" },
        Workflow { id: 2, name: "Protein Folding", status: "Completed" },
        Workflow { id: 3, name: "DNA Sequencing", status: "Failed" },
    ]
}

fn generate_cpu_usage_data() -> Vec<(f64, f64)> {
    let mut rng = rand::thread_rng();
    (0..30).map(|i| (i as f64, rng.gen_range(20.0..100.0))).collect()
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?; // Enables raw mode for input handling
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Percentage(40), // Job Table
                    Constraint::Percentage(40), // Analytics Graph
                ])
                .split(size);

            // Header Block
            let header = Paragraph::new("🚀 Crankshaft Monitoring Dashboard")
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL));

            // Table for workflow statuses
            let workflows = get_workflows();
            let rows: Vec<Row> = workflows.iter().map(|wf| {
                Row::new(vec![
                    Cell::from(format!("{}", wf.id)),
                    Cell::from(wf.name),
                    Cell::from(wf.status).style(match wf.status {
                        "Running" => Style::default().fg(Color::Yellow),
                        "Completed" => Style::default().fg(Color::Green),
                        "Failed" => Style::default().fg(Color::Red),
                        _ => Style::default(),
                    }),
                ])
            }).collect();

            let table = Table::new(rows)
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Percentage(20), // ID column
                    Constraint::Percentage(40), // Name column
                    Constraint::Percentage(40), // Status column
                ]);

            // Generate real-time CPU Usage graph data
            let cpu_data = generate_cpu_usage_data();
            let dataset = Dataset::default()
                .name("CPU Usage %")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Magenta))
                .data(&cpu_data);

            let chart = Chart::new(vec![dataset])
                .block(Block::default().title("📊 CPU Usage Over Time").borders(Borders::ALL))
                .x_axis(Axis::default().title("Time (s)").bounds([0.0, 30.0]))
                .y_axis(Axis::default().title("CPU %").bounds([0.0, 100.0]));

            frame.render_widget(header, chunks[0]);
            frame.render_widget(table, chunks[1]);
            frame.render_widget(chart, chunks[2]);
        })?;

        // Auto-refresh every second
        thread::sleep(Duration::from_secs(1));

        // Exit when 'q' is pressed
        if event::poll(Duration::from_millis(500))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?; // Restore terminal mode
    Ok(())
}
