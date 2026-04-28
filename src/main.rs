use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, BorderType, Paragraph, Row, Table, Sparkline, Gauge},
    Terminal,
};
use std::{io, process::Command, time::{Duration, Instant}, thread};
use sysinfo::{System, Disks};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // === 1. STARTUP BANNER & PERFORMANCE INITIALIZATION ===
    print!("{}[2J", 27 as char); 
    println!("{}", "====================================================".cyan().bold());
    println!("{}", "       🚀 NEXO-TECH ULTIMATE OVERDRIVE (WIN)       ".green().bold());
    println!("{}", "           CONTINUOUS REAL-TIME BOOST              ".blue().bold());
    println!("{}", "====================================================".cyan().bold());
    
    // Force Unlock & Activate Windows Ultimate Performance Plan
    let _ = Command::new("cmd").args(["/C", "powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 && powercfg /setactive e9a42b02-d5df-448d-aa00-03f14749eb61"]).output();
    println!("Power State: [ULTIMATE PERFORMANCE ACTIVATED]");

    // === 2. CONTINUOUS OPTIMIZATION THREAD (Background Hammer) ===
    thread::spawn(|| {
        let mut sys = System::new_all();
        loop {
            sys.refresh_all();
            
            // Priority Lock: Force Rust to High Priority if it's open
            for process in sys.processes_by_exact_name("RustClient.exe") {
                let pid = process.pid();
                // Set to 128 (High Priority) using wmic
                let _ = Command::new("cmd")
                    .args(["/C", &format!("wmic process where ProcessId={} CALL setpriority 128", pid)])
                    .spawn();
            }

            // Continuous RAM Flushing & Standby List Clear
            let _ = Command::new("powershell")
                .args(["-Command", "Clear-Variable -Name * -ErrorAction SilentlyContinue; [System.GC]::Collect()"])
                .spawn();

            thread::sleep(Duration::from_secs(2)); // Hammer interval
        }
    });

    crossterm::terminal::enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut sys = System::new_all();
    let mut fps_history = vec![0u64; 100];
    let mut last_tick = Instant::now();

    // === 3. THE UI LOOP ===
    loop {
        sys.refresh_all();
        let elapsed = last_tick.elapsed().as_secs_f64();
        let current_fps = if elapsed > 0.0 { (1.0 / elapsed) as u64 } else { 0 };
        last_tick = Instant::now();
        fps_history.remove(0);
        fps_history.push(current_fps);

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(8), // ANALYTICS
                    Constraint::Length(4), // RAM GAUGE
                    Constraint::Length(7), // STABILITY
                    Constraint::Min(5),    // HOGS
                ])
                .split(f.size());

            // Render 1: Live Analytics
            let report_text = format!(
                "NEXO-OVERDRIVE STATUS: [ACTIVE]\n\
                 SYSTEM FPS: {}\n\
                 CPU USAGE: {}%\n\
                 POWER MODE: ULTIMATE PERFORMANCE",
                current_fps, sys.global_cpu_info().cpu_usage() as u64
            );
            f.render_widget(Paragraph::new(report_text)
                .alignment(Alignment::Center)
                .block(Block::default().title(" ⚡ LIVE OVERDRIVE ").borders(Borders::ALL).border_type(BorderType::Double).fg(Color::Yellow)), chunks[0]);

            // Render 2: RAM Load
            f.render_widget(Gauge::default()
                .block(Block::default().title(" TOTAL SYSTEM RAM LOAD ").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::LightRed))
                .percent(((sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0) as u16), chunks[1]);

            // Render 3: Stability Chart
            f.render_widget(Sparkline::default()
                .block(Block::default().title(" FRAME STABILITY (CONTINUOUS) ").borders(Borders::ALL))
                .data(&fps_history)
                .style(Style::default().fg(Color::Cyan)), chunks[2]);

            // Render 4: Top Processes (Hogs)
            let mut procs: Vec<_> = sys.processes().values().collect();
            procs.sort_by(|a, b| b.memory().cmp(&a.memory()));
            let rows: Vec<Row> = procs.iter().take(5).map(|p| {
                Row::new(vec![p.name().to_string(), format!("{} MB", p.memory()/1024/1024)])
            }).collect();
            
            let table = Table::new(rows, [Constraint::Percentage(70), Constraint::Percentage(30)])
                .header(Row::new(vec!["PROCESS", "MEMORY"]).style(Style::default().bold().blue()))
                .block(Block::default().title(" [ HIGH RESOURCE PROCESSES ] ").borders(Borders::ALL));
            f.render_widget(table, chunks[3]);
        })?;

        // Handle Keyboard Events
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') { break; }
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
