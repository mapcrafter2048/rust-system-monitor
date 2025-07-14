use crate::app::{App, SystemInfo};
use crate::system_info::{format_bytes, format_uptime};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Gauge, Paragraph, Row,
        Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

const TABS: &[&str] = &["Overview", "Processes", "Network", "Disks"];

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.size());

    // Header
    render_header(f, chunks[0], app);

    // Main content
    match app.current_tab {
        0 => render_overview(f, chunks[1], app),
        1 => render_processes(f, chunks[1], app),
        2 => render_network(f, chunks[1], app),
        3 => render_disks(f, chunks[1], app),
        _ => render_overview(f, chunks[1], app),
    }

    // Footer
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let tabs = Tabs::new(TABS.to_vec())
        .block(Block::default().borders(Borders::ALL).title("System Monitor"))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .select(app.current_tab);
    f.render_widget(tabs, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![
            Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Quit | "),
            Span::styled("h/l", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Switch tabs | "),
            Span::styled("j/k", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Navigate | "),
            Span::styled("r", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Refresh | "),
            Span::styled("s", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Sort | "),
            Span::styled("Del", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(": Kill Process"),
        ]),
    ];
    
    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(help, area);
}

fn render_overview(f: &mut Frame, area: Rect, app: &App) {
    let system_info = app.get_system_info();
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Min(8),
        ])
        .split(area);

    // System Information
    render_system_info(f, chunks[0], &system_info);
    
    // Resource Usage
    render_resource_usage(f, chunks[1], app, &system_info);
    
    // Charts
    render_charts(f, chunks[2], app);
}

fn render_system_info(f: &mut Frame, area: Rect, system_info: &SystemInfo) {
    let info_text = vec![
        Line::from(vec![
            Span::styled("System: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(&system_info.system_name),
        ]),
        Line::from(vec![
            Span::styled("Host: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(&system_info.host_name),
        ]),
        Line::from(vec![
            Span::styled("Kernel: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(&system_info.kernel_version),
        ]),
        Line::from(vec![
            Span::styled("OS: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(&system_info.os_version),
        ]),
        Line::from(vec![
            Span::styled("Uptime: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(format_uptime(system_info.uptime)),
        ]),
        Line::from(vec![
            Span::styled("CPUs: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}", system_info.cpu_count)),
        ]),
    ];

    let system_block = Paragraph::new(info_text)
        .block(
            Block::default()
                .title("System Information")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(system_block, area);
}

fn render_resource_usage(f: &mut Frame, area: Rect, app: &App, system_info: &SystemInfo) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // CPU Usage
    let cpu_usage = if let Some(&last_cpu) = app.cpu_history.last() {
        last_cpu as u16
    } else {
        0
    };
    
    let cpu_gauge = Gauge::default()
        .block(
            Block::default()
                .title("CPU Usage")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .gauge_style(
            Style::default()
                .fg(if cpu_usage > 80 { Color::Red } else if cpu_usage > 60 { Color::Yellow } else { Color::Green })
        )
        .percent(cpu_usage)
        .label(format!("{}%", cpu_usage));
    f.render_widget(cpu_gauge, chunks[0]);

    // Memory Usage
    let memory_usage = ((system_info.used_memory as f64 / system_info.total_memory as f64) * 100.0) as u16;
    
    let memory_gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!("Memory ({}/{})", 
                    format_bytes(system_info.used_memory), 
                    format_bytes(system_info.total_memory)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .gauge_style(
            Style::default()
                .fg(if memory_usage > 80 { Color::Red } else if memory_usage > 60 { Color::Yellow } else { Color::Green })
        )
        .percent(memory_usage)
        .label(format!("{}%", memory_usage));
    f.render_widget(memory_gauge, chunks[1]);
}

fn render_charts(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // CPU History Sparkline
    let cpu_data: Vec<u64> = app.cpu_history.iter().map(|&x| x as u64).collect();
    let cpu_sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("CPU History")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .data(&cpu_data)
        .style(Style::default().fg(Color::Green));
    f.render_widget(cpu_sparkline, chunks[0]);

    // Memory History Sparkline
    let memory_data: Vec<u64> = app.memory_history.iter().map(|&x| x as u64).collect();
    let memory_sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Memory History")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .data(&memory_data)
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(memory_sparkline, chunks[1]);
}

fn render_processes(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Process count and sort info
    let process_info = Paragraph::new(format!(
        "Total Processes: {} | Sort by: {:?} | Selected: {}/{}",
        app.processes.len(),
        app.sort_by,
        app.selected_process + 1,
        app.processes.len()
    ))
    .block(
        Block::default()
            .title("Process Information")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue)),
    )
    .style(Style::default().fg(Color::White));
    f.render_widget(process_info, chunks[0]);

    // Process table
    let header_cells = ["PID", "Name", "CPU%", "Memory", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).style(Style::default().bg(Color::Blue));

    let rows = app.processes.iter().enumerate().map(|(i, process)| {
        let cells = vec![
            Cell::from(process.pid.to_string()),
            Cell::from(process.name.clone()),
            Cell::from(format!("{:.1}%", process.cpu_usage)),
            Cell::from(format_bytes(process.memory)),
            Cell::from(process.status.clone()),
        ];
        
        let style = if i == app.selected_process {
            Style::default().bg(Color::Yellow).fg(Color::Black)
        } else {
            Style::default().fg(Color::White)
        };
        
        Row::new(cells).style(style)
    });

    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Processes")
                .border_style(Style::default().fg(Color::Green)),
        )
        .widths(&[
            Constraint::Length(8),
            Constraint::Min(20),
            Constraint::Length(8),
            Constraint::Length(12),
            Constraint::Min(10),
        ]);

    f.render_widget(table, chunks[1]);
}

fn render_network(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(0)])
        .split(area);

    // Network stats
    let (total_received, total_transmitted) = if let Some(&(rx, tx)) = app.network_history.last() {
        (rx, tx)
    } else {
        (0, 0)
    };

    let network_info = vec![
        Line::from(vec![
            Span::styled("Total Received: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(format_bytes(total_received), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Total Transmitted: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(format_bytes(total_transmitted), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::styled("Network Interfaces: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}", app.networks.len())),
        ]),
    ];

    let network_block = Paragraph::new(network_info)
        .block(
            Block::default()
                .title("Network Statistics")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(network_block, chunks[0]);

    // Network history chart
    let rx_history: Vec<u64> = app.network_history.iter().map(|(rx, _)| *rx / 1024 / 1024).collect(); // Convert to MB
    let tx_history: Vec<u64> = app.network_history.iter().map(|(_, tx)| *tx / 1024 / 1024).collect(); // Convert to MB

    let chart_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let rx_sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Received (MB)")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .data(&rx_history)
        .style(Style::default().fg(Color::Green));
    f.render_widget(rx_sparkline, chart_chunks[0]);

    let tx_sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Transmitted (MB)")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        )
        .data(&tx_history)
        .style(Style::default().fg(Color::Red));
    f.render_widget(tx_sparkline, chart_chunks[1]);
}

fn render_disks(f: &mut Frame, area: Rect, app: &App) {
    let rows = app.disk_usage.iter().map(|(name, used, total)| {
        let usage_percent = if *total > 0 {
            (*used as f64 / *total as f64 * 100.0) as u16
        } else {
            0
        };
        
        let cells = vec![
            Cell::from(name.clone()),
            Cell::from(format_bytes(*used)),
            Cell::from(format_bytes(*total)),
            Cell::from(format_bytes(*total - *used)),
            Cell::from(format!("{}%", usage_percent)),
        ];
        
        let style = if usage_percent > 90 {
            Style::default().fg(Color::Red)
        } else if usage_percent > 75 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Green)
        };
        
        Row::new(cells).style(style)
    });

    let header_cells = ["Drive", "Used", "Total", "Available", "Usage%"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).style(Style::default().bg(Color::Blue));

    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Disk Usage")
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .widths(&[
            Constraint::Length(15),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(8),
        ]);

    f.render_widget(table, area);
}