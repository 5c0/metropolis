mod args;
mod city;
mod config;
mod logos;
mod theme;

use city::{MetropolisCity, Weather};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io, time::{Duration, Instant}};
use sysinfo::System;

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = args::parse()?;
    let config = config::Config::load();
    
    let weather = match cli_args.weather.as_deref().unwrap_or(&config.appearance.default_weather).to_lowercase().as_str() {
        "rain" => Weather::Rain,
        "snow" => Weather::Snow,
        _ => Weather::Clear,
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut sys = System::new();
    sys.refresh_memory();
    sys.refresh_cpu_usage();
    sys.refresh_processes();
    
    // DETECT DISTRO
    let distro = cli_args.distro.clone().unwrap_or_else(|| {
        if !config.monolith.override_distro.is_empty() {
            config.monolith.override_distro.clone()
        } else {
            System::name().unwrap_or_else(|| "linux".to_string())
        }
    }).to_lowercase();
    
    let theme_name = cli_args.theme.as_deref().unwrap_or(&config.appearance.global_theme);
    let global_theme = theme::Theme::from_str(theme_name);

    let mut city = MetropolisCity::new(
        distro, 
        weather, 
        global_theme,
        config.appearance.solid_background_color,
        config.monolith.custom_text,
        config.monolith.custom_color,
        config.simulation,
    );
    city.debug_mode = cli_args.debug;
    
    let tick_rate = Duration::from_millis(50); 
    let sysinfo_tick_rate = Duration::from_millis(1000);
    let mut last_tick = Instant::now();
    let mut last_sysinfo_tick = Instant::now();
    let mut proc_names: Vec<String> = Vec::new();
    let mut last_disk_bytes = 0u64;
    let mut needs_draw = true;

    loop {
        if needs_draw {
            let draw_start = Instant::now();
            terminal.draw(|f| {
                f.render_widget(&city, f.size());
            })?;
            city.perf.draw_us = draw_start.elapsed().as_micros() as u64;
            city.perf.push_frame();
            needs_draw = false;
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('r') => {
                                city.weather = if city.weather == Weather::Rain { Weather::Clear } else { Weather::Rain };
                                needs_draw = true;
                            },
                            KeyCode::Char('s') => {
                                city.weather = if city.weather == Weather::Snow { Weather::Clear } else { Weather::Snow };
                                needs_draw = true;
                            },
                            KeyCode::Char('d') => {
                                city.debug_mode = !city.debug_mode;
                                needs_draw = true;
                            },
                            _ => {}
                        }
                    }
                },
                Event::Resize(_, _) => {
                    needs_draw = true;
                },
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let mut cpu = sys.global_cpu_info().cpu_usage();
            let mut ram = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
            let mut disk_usage = city.disk_usage;

            if last_sysinfo_tick.elapsed() >= sysinfo_tick_rate {
                sys.refresh_memory();
                sys.refresh_cpu_usage();
                sys.refresh_processes();
                last_sysinfo_tick = Instant::now();

                cpu = sys.global_cpu_info().cpu_usage();
                ram = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;

                let mut procs: Vec<(String, f32)> = sys.processes()
                    .values()
                    .map(|p| (p.name().to_string(), p.cpu_usage()))
                    .collect();
                
                procs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                
                proc_names = procs.into_iter()
                    .filter(|(name, _)| !name.to_lowercase().contains("metropolis"))
                    .take(10)
                    .map(|(name, _)| {
                        let clean = name.split('.').next().unwrap_or(&name);
                        clean.to_uppercase().chars().take(8).collect()
                    })
                    .collect();

                let current_disk_bytes: u64 = sys.processes()
                    .values()
                    .map(|p| p.disk_usage().read_bytes + p.disk_usage().written_bytes)
                    .sum();
                let disk_delta = current_disk_bytes.saturating_sub(last_disk_bytes);
                last_disk_bytes = current_disk_bytes;
                disk_usage = (disk_delta as f32 / 250_000.0).min(100.0);
            }

            let update_start = Instant::now();
            city.update(terminal.size()?, cpu, ram, disk_usage, proc_names.clone());
            city.perf.update_us = update_start.elapsed().as_micros() as u64;
            last_tick = Instant::now();
            needs_draw = true;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}