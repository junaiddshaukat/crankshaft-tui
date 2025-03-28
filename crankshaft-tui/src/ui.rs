//! UI rendering for the TUI.

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Line, Text},
    widgets::{
        Block, Borders, BorderType, Cell, Gauge, List, ListItem, Paragraph, Row, Table, Tabs,
        Wrap, Padding, canvas::Canvas,
    },
    Frame,
};

use crate::app::{App, TaskStatus};

/// Renders the user interface widgets.
pub fn draw(f: &mut Frame, app: &App) {
    // Create a layered layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_tabs(f, app, main_layout[0]);
    
    match app.tab_index {
        0 => draw_tasks_tab(f, app, main_layout[1]),
        1 => draw_stats_tab(f, app, main_layout[1]),
        2 => draw_help_tab(f, app, main_layout[1]),
        _ => {}
    }
    
    draw_footer(f, main_layout[2]);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = ["Tasks", "Statistics", "Help"]
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(rest, Style::default().fg(Color::White))
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(Span::styled(" Crankshaft Monitor ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
                .title_alignment(Alignment::Center)
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray)
        )
        .select(app.tab_index)
        .divider(Span::styled("|", Style::default().fg(Color::DarkGray)));
    
    f.render_widget(tabs, area);
}

fn draw_tasks_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
        .split(area);
    
    // Task list
    let tasks: Vec<ListItem<'_>> = app.task_ids
        .iter()
        .map(|id| {
            let task = &app.tasks[id];
            let (status_color, status_icon) = match task.status {
                TaskStatus::Pending => (Color::Blue, "⏳"),
                TaskStatus::Running => (Color::Yellow, "▶️"),
                TaskStatus::Completed => (Color::Green, "✅"),
                TaskStatus::Failed => (Color::Red, "❌"),
            };
            
            let content = Line::from(vec![
                Span::styled(format!(" {} ", status_icon), Style::default()),
                Span::styled(format!("{:<8}", task.id), Style::default().fg(Color::White)),
                Span::styled(format!("{:<12}", task.status), Style::default().fg(status_color)),
                Span::styled(task.name.clone(), Style::default()),
            ]);
            
            ListItem::new(content)
        })
        .collect();
    
    let tasks_list = List::new(tasks)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(Span::styled(" Tasks ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
                .padding(Padding::new(1, 1, 0, 0))
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray)
        )
        .highlight_symbol("➤ ");
    
    let mut state = ratatui::widgets::ListState::default();
    if let Some(selected_id) = &app.selected_task_id {
        if let Some(index) = app.task_ids.iter().position(|id| id == selected_id) {
            state.select(Some(index));
        }
    }
    
    f.render_stateful_widget(tasks_list, chunks[0], &mut state);
    
    // Task details
    if let Some(selected_id) = &app.selected_task_id {
        if let Some(task) = app.tasks.get(selected_id) {
            draw_task_details(f, task, chunks[1]);
        }
    } else {
        let no_selection = Paragraph::new(Text::styled(
            "Select a task to view details",
            Style::default().fg(Color::DarkGray)
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(Span::styled(" Task Details ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        )
        .alignment(Alignment::Center);
        f.render_widget(no_selection, chunks[1]);
    }
}

fn draw_task_details(f: &mut Frame, task: &crate::app::Task, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(area);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(" Task Details ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    f.render_widget(block, area);
    
    // Task ID
    let id_text = Paragraph::new(Line::from(vec![
        Span::styled("ID: ", Style::default().fg(Color::Gray)),
        Span::styled(task.id.to_string(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
    ]));
    f.render_widget(id_text, chunks[0]);
    
    // Task Name
    let name_text = Paragraph::new(Line::from(vec![
        Span::styled("Name: ", Style::default().fg(Color::Gray)),
        Span::styled(&task.name, Style::default().fg(Color::White)),
    ]));
    f.render_widget(name_text, chunks[1]);
    
    // Task Status
    let (status_color, status_icon) = match task.status {
        TaskStatus::Pending => (Color::Blue, "⏳"),
        TaskStatus::Running => (Color::Yellow, "▶️"),
        TaskStatus::Completed => (Color::Green, "✅"),
        TaskStatus::Failed => (Color::Red, "❌"),
    };
    
    let status_text = Paragraph::new(Line::from(vec![
        Span::styled("Status: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} {}", status_icon, task.status), Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
    ]));
    f.render_widget(status_text, chunks[2]);
    
    // Progress bar
    let progress_label = format!(" {:.1}% ", task.progress * 100.0);
    let gauge = Gauge::default()
        .block(Block::default().title("Progress"))
        .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .ratio(task.progress)
        .label(progress_label)
        .use_unicode(true);
    f.render_widget(gauge, chunks[3]);
    
    // CPU Usage
    let cpu_label = format!(" {:.1}% ", task.cpu_usage * 100.0);
    let cpu_gauge = Gauge::default()
        .block(Block::default().title("CPU Usage"))
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .ratio(task.cpu_usage)
        .label(cpu_label)
        .use_unicode(true);
    f.render_widget(cpu_gauge, chunks[4]);
    
    // Additional info could be added here
    if chunks.len() > 5 && task.status == TaskStatus::Running {
        let info_text = Paragraph::new(Text::styled(
            "Task is currently running...",
            Style::default().fg(Color::Yellow)
        ))
        .alignment(Alignment::Center);
        f.render_widget(info_text, chunks[5]);
    }
}

fn draw_stats_tab(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);
    
    // Task status summary table
    let mut pending = 0;
    let mut running = 0;
    let mut completed = 0;
    let mut failed = 0;
    
    for task in app.tasks.values() {
        match task.status {
            TaskStatus::Pending => pending += 1,
            TaskStatus::Running => running += 1,
            TaskStatus::Completed => completed += 1,
            TaskStatus::Failed => failed += 1,
        }
    }
    
    let total = app.tasks.len();
    let completed_percent = if total > 0 { (completed as f64 / total as f64) * 100.0 } else { 0.0 };
    
    let rows = vec![
        Row::new(vec![
            Cell::from("Pending"),
            Cell::from(pending.to_string()).style(Style::default().fg(Color::Blue)),
            Cell::from(format!("{:.1}%", if total > 0 { (pending as f64 / total as f64) * 100.0 } else { 0.0 })),
        ]),
        Row::new(vec![
            Cell::from("Running"),
            Cell::from(running.to_string()).style(Style::default().fg(Color::Yellow)),
            Cell::from(format!("{:.1}%", if total > 0 { (running as f64 / total as f64) * 100.0 } else { 0.0 })),
        ]),
        Row::new(vec![
            Cell::from("Completed"),
            Cell::from(completed.to_string()).style(Style::default().fg(Color::Green)),
            Cell::from(format!("{:.1}%", completed_percent)),
        ]),
        Row::new(vec![
            Cell::from("Failed"),
            Cell::from(failed.to_string()).style(Style::default().fg(Color::Red)),
            Cell::from(format!("{:.1}%", if total > 0 { (failed as f64 / total as f64) * 100.0 } else { 0.0 })),
        ]),
        Row::new(vec![
            Cell::from("Total").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from(total.to_string()).style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("100.0%").style(Style::default().add_modifier(Modifier::BOLD)),
        ]),
    ];
    
    let table = Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(Span::styled(" Task Statistics ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        )
        .header(
            Row::new(vec!["Status", "Count", "Percentage"])
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        )
        .widths(&[Constraint::Percentage(40), Constraint::Percentage(30), Constraint::Percentage(30)])
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    
    f.render_widget(table, chunks[0]);
    
    // Progress overview
    let progress_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(" Overall Progress ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    
    f.render_widget(progress_block, chunks[1]);
    
    let progress_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ].as_ref())
        .split(chunks[1]);
    
    // Overall completion gauge
    let completion_gauge = Gauge::default()
        .block(Block::default().title("Completion"))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .ratio(completed_percent / 100.0)
        .label(format!(" {:.1}% ", completed_percent))
        .use_unicode(true);
    
    f.render_widget(completion_gauge, progress_chunks[0]);
    
    // Failure rate gauge
    let failure_rate = if total > 0 { (failed as f64 / total as f64) } else { 0.0 };
    let failure_gauge = Gauge::default()
        .block(Block::default().title("Failure Rate"))
        .gauge_style(Style::default().fg(Color::Red).bg(Color::Black))
        .ratio(failure_rate)
        .label(format!(" {:.1}% ", failure_rate * 100.0))
        .use_unicode(true);
    
    f.render_widget(failure_gauge, progress_chunks[1]);
}

fn draw_help_tab(f: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(" Help & Keyboard Shortcuts ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    
    let text = vec![
        Line::from(vec![
            Span::styled("Keyboard shortcuts:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("q", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" or "),
            Span::styled("Esc", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" - Quit the application"),
        ]),
        Line::from(vec![
            Span::styled("Tab", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" - Switch to next tab"),
        ]),
        Line::from(vec![
            Span::styled("Shift+Tab", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" - Switch to previous tab"),
        ]),
        Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" - Navigate through task list"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("About Crankshaft:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from("Crankshaft is a headless task execution framework that supports local, cloud, and HPC environments."),
        Line::from("It's designed to be a high-performance engine for managing and executing tasks concurrently."),
        Line::from(""),
        Line::from(vec![
            Span::styled("Task Status Icons:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("⏳ - "),
            Span::styled("Pending", Style::default().fg(Color::Blue)),
            Span::raw(" | "),
            Span::raw("▶️ - "),
            Span::styled("Running", Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::raw("✅ - "),
            Span::styled("Completed", Style::default().fg(Color::Green)),
            Span::raw(" | "),
            Span::raw("❌ - "),
            Span::styled("Failed", Style::default().fg(Color::Red)),
        ]),
    ];
    
    let help_text = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true });
    
    f.render_widget(help_text, area);
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::DarkGray)),
            Span::styled("q", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(" to quit | ", Style::default().fg(Color::DarkGray)),
            Span::styled("Tab", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(" to switch tabs | ", Style::default().fg(Color::DarkGray)),
            Span::styled("↑/↓", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(" to navigate", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .alignment(Alignment::Center);
    
    f.render_widget(paragraph, area);
}