pub mod weather;
pub mod vehicles;
pub mod people;
pub mod buildings;
pub mod utils;

use rand::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::Widget,
};
use crate::theme::Theme;

// Re-export common types for the rest of the app
pub use weather::Weather;
use weather::{Raindrop, Splash};
use vehicles::{Vehicle, VehicleType};
use people::Person;
use utils::*;
use buildings::*;

const PERF_HISTORY_LEN: usize = 60;

pub struct PerfStats {
    pub update_us: u64,
    pub draw_us: u64,
    history_update: [u64; PERF_HISTORY_LEN],
    history_draw: [u64; PERF_HISTORY_LEN],
    head: usize,
    count: usize,
}

impl PerfStats {
    fn new() -> Self {
        Self {
            update_us: 0,
            draw_us: 0,
            history_update: [0; PERF_HISTORY_LEN],
            history_draw: [0; PERF_HISTORY_LEN],
            head: 0,
            count: 0,
        }
    }

    pub fn push_frame(&mut self) {
        self.history_update[self.head] = self.update_us;
        self.history_draw[self.head] = self.draw_us;
        self.head = (self.head + 1) % PERF_HISTORY_LEN;
        if self.count < PERF_HISTORY_LEN {
            self.count += 1;
        }
    }

    pub fn avg_update_us(&self) -> u64 {
        if self.count == 0 { return 0; }
        self.history_update[..self.count].iter().sum::<u64>() / self.count as u64
    }

    pub fn avg_draw_us(&self) -> u64 {
        if self.count == 0 { return 0; }
        self.history_draw[..self.count].iter().sum::<u64>() / self.count as u64
    }

    pub fn max_update_us(&self) -> u64 {
        if self.count == 0 { return 0; }
        *self.history_update[..self.count].iter().max().unwrap_or(&0)
    }

    pub fn max_draw_us(&self) -> u64 {
        if self.count == 0 { return 0; }
        *self.history_draw[..self.count].iter().max().unwrap_or(&0)
    }

    /// Returns the draw history as (index, value) pairs, oldest first
    pub fn draw_history(&self) -> impl Iterator<Item = (usize, u64)> + '_ {
        (0..self.count).map(move |i| {
            let idx = if self.count < PERF_HISTORY_LEN {
                i
            } else {
                (self.head + i) % PERF_HISTORY_LEN
            };
            (i, self.history_draw[idx])
        })
    }
}

pub struct MetropolisCity {
    pub vehicles: Vec<Vehicle>,
    pub raindrops: Vec<Raindrop>,
    pub splashes: Vec<Splash>,
    pub people: Vec<Person>,
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub cpu_smoothed: f32,
    pub ram_smoothed: f32,
    pub frame_count: u64,
    pub window_seed: u64,
    pub chase_cooldown: u32,
    pub distro: String,
    pub weather: Weather,
    pub debug_mode: bool,
    pub top_processes: Vec<String>,
    pub disk_usage: f32,
    pub solid_background_color: String,
    pub custom_monolith_text: String,
    pub custom_monolith_color: String,
    pub monolith_sign_text: String,
    pub theme: Theme,
    pub logo_asset: crate::logos::DistroLogo,
    pub simulation_config: crate::config::SimulationConfig,
    pub cpu_string: String,
    pub ram_string: String,
    pub perf: PerfStats,
    pub buildings_cache: Vec<BuildingInfo>,
    cached_width: u16,
    cached_height: u16,
}

impl MetropolisCity {
    pub fn new(
        distro: String, 
        weather: Weather, 
        theme: Theme,
        solid_background_color: String,
        custom_monolith_text: String,
        custom_monolith_color: String,
        simulation_config: crate::config::SimulationConfig,
    ) -> Self {
        let display_name = match distro.to_lowercase().as_str() {
            "popos" | "pop_os" => "POP! OS".to_string(),
            "endeavouros" => "ENDEAVOUR OS".to_string(),
            "artix" => "ARTIX LINUX".to_string(),
            "garuda" => "GARUDA LINUX".to_string(),
            "zorin" => "ZORIN OS".to_string(),
            "rhel" | "redhat" => "RED HAT".to_string(),
            _ => distro.to_uppercase(),
        };

        let monolith_sign_text = if !custom_monolith_text.is_empty() {
            custom_monolith_text.clone()
        } else {
            format!("{} CORP", display_name)
        };

        let logo_asset = crate::logos::get_logo(&distro);

        Self {
            vehicles: Vec::with_capacity(100),
            raindrops: Vec::with_capacity(250),
            splashes: Vec::with_capacity(50),
            people: Vec::with_capacity(30),
            cpu_usage: 0.0,
            ram_usage: 0.0,
            cpu_smoothed: 0.0,
            ram_smoothed: 0.0,
            frame_count: 0,
            window_seed: thread_rng().gen(),
            chase_cooldown: 0,
            distro,
            weather,
            debug_mode: false,
            top_processes: Vec::new(),
            disk_usage: 0.0,
            solid_background_color,
            custom_monolith_text,
            custom_monolith_color,
            monolith_sign_text,
            theme,
            logo_asset,
            simulation_config,
            cpu_string: String::from("0"),
            ram_string: String::from("0"),
            perf: PerfStats::new(),
            buildings_cache: Vec::new(),
            cached_width: 0,
            cached_height: 0,
        }
    }

    /// Rebuild the building geometry cache if terminal dimensions changed
    fn rebuild_buildings_if_needed(&mut self, area: Rect) {
        if area.width != self.cached_width || area.height != self.cached_height {
            self.buildings_cache = compute_buildings(area.width, area.height);
            self.cached_width = area.width;
            self.cached_height = area.height;
        }
    }

    pub fn update(&mut self, area: Rect, cpu: f32, ram: f32, disk: f32, processes: Vec<String>) {
        if area.width == 0 || area.height == 0 { return; }
        self.cpu_usage = cpu;
        self.ram_usage = ram;
        self.disk_usage = disk;
        self.top_processes = processes;
        self.cpu_smoothed = self.cpu_smoothed + (cpu - self.cpu_smoothed) * 0.05;
        self.ram_smoothed = self.ram_smoothed + (ram - self.ram_smoothed) * 0.05;
        self.frame_count = self.frame_count.wrapping_add(1);
        self.cpu_string = format!("{:>2.0}", self.cpu_smoothed);
        self.ram_string = format!("{:>2.0}", self.ram_smoothed);

        if area.width < 32 || area.height < 12 { return; }

        self.rebuild_buildings_if_needed(area);

        let mut rng = thread_rng();

        vehicles::update_vehicles(&mut self.vehicles, &mut self.chase_cooldown, self.frame_count, cpu, self.disk_usage, area, &self.theme, &self.simulation_config, &mut rng);
        let mut people = std::mem::take(&mut self.people);
        people::update_people(&mut people, self.frame_count, area, &self.theme, &self.simulation_config, &self.buildings_cache, &mut rng);
        self.people = people;
        weather::update_weather(&self.weather, &mut self.raindrops, &mut self.splashes, self.frame_count, area, &self.simulation_config, &mut rng);
    }

    fn render_background(&self, area: Rect, buf: &mut Buffer) {
        let bg_color = crate::theme::parse_color(&self.solid_background_color).unwrap_or(Color::Reset);
        if !self.solid_background_color.is_empty() {
            for y in area.top()..area.bottom() {
                for x in area.left()..area.right() {
                    buf.get_mut(x, y).set_symbol(" ").set_bg(bg_color);
                }
            }
        }
    }

    fn render_tiny_indicator(&self, area: Rect, buf: &mut Buffer) {
        let cx = area.x + area.width / 2;
        let cy = area.y + area.height / 2;
        let color = if self.frame_count % 20 < 10 { Color::Red } else { Color::Rgb(60, 0, 0) };
        safe_set_symbol(buf, cx, cy, "!", color);
    }

    fn render_mini_hud(&self, area: Rect, buf: &mut Buffer) {
        let cx = area.x.saturating_add(area.width.saturating_div(2));
        let cy = area.y.saturating_add(area.height.saturating_div(2));
        if area.width >= 24 && area.height >= 5 {
            let start_y = cy.saturating_sub(1);
            let start_x = cx.saturating_sub(10);
            
            let metrics = [
                ("CPU", self.cpu_smoothed, self.theme.neon_main),
                ("RAM", self.ram_smoothed, self.theme.neon_sub1),
                ("DSK", self.disk_usage, self.theme.neon_sub2)
            ];
            for (i, (label, pct, c)) in metrics.iter().enumerate() {
                let y = start_y + i as u16;
                safe_set_string(buf, start_x, y, label, Color::White);
                safe_set_string(buf, start_x + 4, y, "[", Color::DarkGray);
                safe_set_string(buf, start_x + 15, y, "]", Color::DarkGray);
                let fill = (pct / 10.0).max(0.0).min(10.0) as u16;
                for j in 0..10 {
                    let sym = if j < fill { "█" } else { "░" };
                    let fg = if j < fill { *c } else { Color::DarkGray };
                    safe_set_symbol(buf, start_x + 5 + j, y, sym, fg);
                }
                safe_set_string(buf, start_x + 17, y, &format!("{:>2.0}%", pct), *c);
            }
        } else if area.width >= 12 && area.height >= 2 {
            let label_c = Color::DarkGray;
            safe_set_string(buf, cx.saturating_sub(6), cy.saturating_sub(1), "CPU:", label_c);
            safe_set_string(buf, cx, cy.saturating_sub(1), &self.cpu_string, self.theme.neon_main);
            safe_set_string(buf, cx.saturating_sub(6), cy, "RAM:", label_c);
            safe_set_string(buf, cx, cy, &self.ram_string, self.theme.neon_sub1);
        }
    }

    fn render_stars(&self, area: Rect, buf: &mut Buffer) {
        let mut star_rng = StdRng::seed_from_u64(42); 
        for i in 0..25 {
            let x = star_rng.gen_range(0..area.width);
            let y = star_rng.gen_range(0..area.height / 2);
            let mut p_rng = StdRng::seed_from_u64(i as u64);
            let star_type = p_rng.gen_range(0..4);
            let (symbol, dim_color) = match star_type {
                0 => ('.', Color::Rgb(60, 60, 80)),
                1 => ('·', Color::Rgb(50, 50, 70)),
                2 => ('*', Color::Rgb(70, 70, 90)),
                _ => ('+', Color::Rgb(40, 40, 60)),
            };
            let pulse = ((self.frame_count as f32 * 0.1 + i as f32).sin() + 1.0) / 2.0;
            let color = if pulse > 0.85 {
                match star_type {
                    0 => Color::Rgb(200, 200, 255),
                    1 => Color::Cyan,
                    2 => Color::Rgb(255, 150, 255),
                    _ => Color::White,
                }
            } else if pulse > 0.5 { dim_color } else { Color::Rgb(30, 30, 45) };
            safe_set_char(buf, area.x + x, area.y + y, symbol, color);
        }
    }

    fn render_skyline(&self, area: Rect, buf: &mut Buffer) {
        let logo_asset = &self.logo_asset;
        let b_base_color = self.theme.building_base_colors[1];
        let ground_y = area.height.saturating_sub(3);

        let mut bg_rng = StdRng::seed_from_u64(12345);
        let bg_color = darken_color(self.theme.building_base_colors[0]);
        for x_bg in (0..area.width).step_by(15) {
            let bw = bg_rng.gen_range(6..15) as u16;
            let bh = bg_rng.gen_range(area.height / 5..area.height / 2) as u16;
            let start_x = area.x.saturating_add(x_bg);
            let start_y = ground_y.saturating_sub(bh);
            for y_rel in 0..bh {
                for x_rel in 0..bw {
                    let dx = start_x.saturating_add(x_rel);
                    let dy = start_y.saturating_add(y_rel);
                    if dx < area.x + area.width && dy < area.y + area.height {
                        safe_set_char_with_bg(buf, dx, dy, ' ', Color::Reset, bg_color);
                    }
                }
            }
        }

        for b in &self.buildings_cache {
            let i = b.index;
            let bw = b.width;
            let bh = b.height;
            let start_y = ground_y.saturating_sub(bh);
            let start_x = area.x.saturating_add(b.x_offset);

            for y_rel in 0..bh {
                for x_rel in 0..bw {
                    let dx = start_x.saturating_add(x_rel);
                    let dy = start_y.saturating_add(y_rel);
                    if dx < area.x + area.width && dy < area.y + area.height {
                        let mut symbol = " ";
                        let mut fg = b_base_color;
                        let mut bg = b_base_color;
                        let mut is_logo_pixel = false;
                        if i == 1 && y_rel < 20 && x_rel < 32 {
                            if let Some(pixel) = &logo_asset.grid[y_rel as usize][x_rel as usize] {
                                let logo_bg = if pixel.bg == Color::Reset { b_base_color } else { pixel.bg };
                                safe_set_char_with_bg(buf, dx, dy, pixel.ch, self.theme.logo_override.unwrap_or(pixel.color), logo_bg);
                                is_logo_pixel = true;
                            } else if self.distro.to_lowercase().contains("windows") && x_rel >= 6 && x_rel <= 26 && y_rel >= 3 && y_rel <= 14 {
                                bg = b_base_color;
                                is_logo_pixel = true;
                            } else if !logo_asset.is_compact && x_rel > 4 && x_rel < 28 {
                                bg = b_base_color;
                                is_logo_pixel = true;
                            }
                            if is_logo_pixel && logo_asset.grid[y_rel as usize][x_rel as usize].is_some() {
                                continue;
                            }
                        }
                        if !is_logo_pixel {
                            if x_rel == 0 || x_rel == bw.saturating_sub(1) { 
                                symbol = "┃"; 
                                fg = Color::Rgb(30, 30, 50); 
                            }
                            let has_sign = i % 2 == 1 && bh > 12;
                            let is_win_row = y_rel > 2 && y_rel < bh.saturating_sub(4) && y_rel % 3 == 0;
                            let x_clearance = if has_sign { bw.saturating_sub(2) } else { bw.saturating_sub(1) };
                            let mut near_logo = false;
                            if i == 1 {
                                for dy_off in -1..=1 {
                                    for dx_off in -1..=1 {
                                        let check_y = (y_rel as i32 + dy_off) as usize;
                                        let check_x = (x_rel as i32 + dx_off) as usize;
                                        if check_y < 20 && check_x < 32 {
                                            if logo_asset.grid[check_y][check_x].is_some() {
                                                near_logo = true; break;
                                            }
                                        }
                                    }
                                    if near_logo { break; }
                                }
                            }
                            if !near_logo && is_win_row && x_rel > 0 && x_rel < x_clearance && (dx.wrapping_add(dy as u16)) % 4 == 0 {
                                let door_x = bw / 2;
                                if !(y_rel >= bh.saturating_sub(3) && x_rel >= door_x.saturating_sub(1) && x_rel <= door_x + 1) {
                                    symbol = "▄";
                                    let seed = (dx as u64).wrapping_mul(100).wrapping_add(dy as u64).wrapping_add(self.window_seed);
                                    let mut wr = StdRng::seed_from_u64(seed);
                                    fg = if wr.gen_bool(0.25) { self.theme.window_lit } else { self.theme.window_unlit };
                                    bg = self.theme.window_dark;
                                }
                            }
                            if y_rel >= bh.saturating_sub(3) {
                                let door_x = bw / 2;
                                if x_rel >= door_x.saturating_sub(1) && x_rel <= door_x + 1 {
                                    if y_rel == bh.saturating_sub(3) { 
                                        symbol = "━"; 
                                        fg = if i % 2 == 0 { self.theme.neon_sub1 } else { self.theme.neon_main }; 
                                    } else { 
                                        symbol = "░"; 
                                        fg = self.theme.window_unlit; 
                                    }
                                }
                                if x_rel == door_x + 2 && y_rel == bh.saturating_sub(2) {
                                    symbol = "·"; 
                                    fg = if self.frame_count % 20 < 10 { Color::Red } else { Color::Green };
                                }
                            }
                        }
                        safe_set_symbol_with_bg(buf, dx, dy, symbol, fg, bg);
                    }
                }
            }

            if i % 2 == 1 && bh > 12 {
                let sign_text: &str;
                let sign_color;
                if i == 1 {
                    sign_text = &self.monolith_sign_text;
                    if !self.custom_monolith_color.is_empty() {
                        sign_color = crate::theme::parse_color(&self.custom_monolith_color).unwrap_or(self.theme.neon_main);
                    } else {
                        sign_color = self.theme.neon_main;
                    }
                } else {
                    let p_idx = (i / 2).saturating_sub(1) % self.top_processes.len().max(1);
                    sign_text = if self.top_processes.is_empty() {
                        "NULL"
                    } else {
                        &self.top_processes[p_idx % self.top_processes.len()]
                    };
                    sign_color = match (i / 2) % 3 {
                        0 => self.theme.neon_sub1,
                        1 => self.theme.neon_sub2,
                        _ => self.theme.neon_sub3,
                    };
                }
                let sign_y = start_y.saturating_add(5);
                draw_neon_sign(buf, start_x + bw.saturating_sub(1), sign_y, sign_text, sign_color, self.frame_count);
            }

            if i != 1 && i != 3 {
                let ant_x = start_x.saturating_add(2);
                if ant_x < buf.area.width {
                    let ant_y = area.y.saturating_add(start_y.saturating_sub(1));
                    if ant_y < buf.area.height {
                        match i % 3 {
                            0 => {
                                safe_set_symbol(buf, ant_x, ant_y, "┷", Color::Rgb(60, 60, 80));
                                if ant_y > area.y {
                                    safe_set_symbol(buf, ant_x, ant_y - 1, "┃", Color::Rgb(50, 50, 70));
                                    let beacon_color = if self.frame_count % 30 < 15 { Color::Red } else { Color::Rgb(60, 0, 0) };
                                    if ant_y > area.y + 1 { safe_set_symbol(buf, ant_x, ant_y - 2, "*", beacon_color); }
                                }
                            }
                            1 => { safe_set_symbol(buf, ant_x, ant_y, "📡", Color::Rgb(100, 100, 120)); }
                            _ => { safe_set_symbol(buf, ant_x, ant_y, "▝▘", Color::Rgb(40, 40, 50)); }
                        }
                    }
                }
            }
        }
    }

    fn render_street_lamps(&self, area: Rect, buf: &mut Buffer) {
        for x_lamp in (5..area.width).step_by(10) {
            let inside = self.buildings_cache.iter().any(|b| b.contains_x(x_lamp));
            if !inside {
                let lx = area.x + x_lamp; 
                let ground_y = area.y + area.height - 3;
                let bulb_c = if (self.frame_count + lx as u64) % 40 < 2 { self.theme.street_lamp_dim } else { self.theme.street_lamp_lit };
                safe_set_symbol(buf, lx, ground_y, "┃", self.theme.ground);
                safe_set_symbol(buf, lx, ground_y.saturating_sub(1), "┃", self.theme.ground);
                safe_set_symbol(buf, lx, ground_y.saturating_sub(2), "┃", self.theme.ground);
                safe_set_string(buf, lx.saturating_sub(1), ground_y.saturating_sub(3), "(O)", bulb_c);
            }
        }
    }

    fn render_weather_bg(&self, area: Rect, buf: &mut Buffer) {
        if self.weather == Weather::Rain {
            for r in &self.raindrops {
                let rx = area.x + r.x as u16; let ry = area.y + r.y as u16;
                let sym = if r.z_index == 1 { "|" } else { ":" };
                let color = if r.z_index == 1 { self.theme.rain } else { self.theme.rain_bg };
                safe_set_symbol(buf, rx, ry, sym, color);
            }
        } else if self.weather == Weather::Snow {
            for r in &self.raindrops {
                let rx = area.x + r.x as u16; let ry = area.y + r.y as u16;
                let sym = match (self.frame_count + (r.x as u64)) % 30 {
                    0..=10 => "*",
                    11..=20 => "·",
                    _ => "❄",
                };
                let color = if r.z_index == 1 { self.theme.snow } else { darken_color(self.theme.snow) };
                safe_set_symbol(buf, rx, ry, sym, color);
            }
            let ground_y = area.y + area.height - 3;
            for rx in 0..area.width {
                let dx = area.x + rx;
                let sym = if (dx as u64 + self.frame_count / 100) % 7 == 0 { "▆" } else { "█" };
                safe_set_symbol(buf, dx, ground_y + 1, sym, self.theme.snow);
                safe_set_symbol(buf, dx, ground_y + 2, "█", self.theme.snow);
            }
        }
    }

    fn render_megaboard(&self, area: Rect, buf: &mut Buffer) {
        let ground_y = area.height.saturating_sub(3);
        // Find building at index 3 from the cache
        if let Some(b3) = self.buildings_cache.iter().find(|b| b.index == 3) {
            let mb_x = area.x + 60;
            let mb_y = ground_y.saturating_sub(b3.height);
            draw_roof_megaboard(
                buf, 
                mb_x + 1, mb_y, 
                self.cpu_smoothed, self.ram_smoothed, 
                &self.cpu_string, &self.ram_string, 
                self.theme.neon_main, self.theme.neon_sub1,
                self.frame_count
            );
        }
    }

    fn render_people(&self, area: Rect, buf: &mut Buffer) {
        let ground_y = area.height.saturating_sub(3);
        let b_base_color = self.theme.building_base_colors[1];
        
        for p in &self.people {
            if p.x < 0.0 { continue; }
            let px = area.x + p.x as u16; 
            let py_l = area.y + ground_y; 
            let py_h = py_l.saturating_sub(1);
            if px < area.x + area.width && py_l < area.y + area.height {
                let building_bg = self.buildings_cache.iter().find(|b| {
                    let bx = area.x + b.x_offset;
                    let top_y = ground_y.saturating_sub(b.height);
                    px >= bx && px < bx + b.width && py_l >= area.y + top_y
                }).map(|_| b_base_color);

                let gait = if p.is_entering && p.entry_pause_timer > 0 { 1 } else { ((self.frame_count + p.id_offset) / 4) % 3 };
                let leg_char = match gait { 0 => 'Λ', 1 => '|', _ => 'λ' };
                safe_set_char_with_bg(buf, px, py_h, 'o', p.color, building_bg.unwrap_or(Color::Reset));
                safe_set_char_with_bg(buf, px, py_l, leg_char, p.color, building_bg.unwrap_or(Color::Reset));
            }
        }
    }

    fn render_vehicles(&self, area: Rect, buf: &mut Buffer) {
        for v in &self.vehicles {
            if v.x < -15.0 { continue; }
            let vx_f = area.x as f32 + v.x; let vy = area.y as u16 + v.y as u16;
            if vy >= area.y + area.height { continue; }
            let (body, tail_color) = match v.v_type { 
                VehicleType::Spinner => (vec!['◢', '■', '◣'], Some(self.theme.police_red)),
                VehicleType::Shuttle => {
                    let mut b = Vec::new(); b.push('▓');
                    for _ in 0..v.length.saturating_sub(2) { b.push('█'); }
                    b.push('▶'); (b, Some(self.theme.neon_main))
                },
                VehicleType::Police => (vec!['◤', '█', '◥'], None),
            };
            for (off, ch) in body.iter().enumerate() {
                let dx = (vx_f + off as f32) as u16;
                if dx >= area.x + area.width || vy >= area.y + area.height { continue; }
                let final_fg = if v.v_type == VehicleType::Police { match off { 0 => Color::White, 1 => Color::Rgb(60, 60, 70), _ => Color::White } } else { v.color };
                let sym = safe_get_symbol(buf, dx, vy);
                let fg_peek = safe_get_fg(buf, dx, vy);
                let bg_peek = safe_get_bg(buf, dx, vy);
                let effective_bg = if sym == "█" || sym == "▓" || sym == "▆" || sym == "▄" { fg_peek } else { bg_peek };
                safe_set_char_with_bg(buf, dx, vy, *ch, final_fg, effective_bg);
            }
            if v.v_type == VehicleType::Police && vy > area.y {
                let sy = vy.saturating_sub(1); let flash = (self.frame_count / 2) % 2 == 0;
                for (sx_f, base_color, is_on) in vec![(vx_f, self.theme.police_blue, flash), (vx_f + 2.0, self.theme.police_red, !flash)] {
                    let sx = sx_f as u16; if sx < area.x + area.width {
                        let sym = safe_get_symbol(buf, sx, sy);
                        let l_bg = if sym == "█" || sym == "▓" || sym == "▆" || sym == "▄" { safe_get_fg(buf, sx, sy) } else { safe_get_bg(buf, sx, sy) };
                        safe_set_char_with_bg(buf, sx, sy, '═', if is_on { base_color } else { Color::Rgb(40, 40, 60) }, l_bg);
                    }
                }
            }
            if let Some(t_color) = tail_color {
                let tx_f = v.x - 1.0;
                if tx_f >= 0.0 {
                    let tx = (area.x as f32 + tx_f) as u16;
                    if tx < area.x + area.width {
                        let sym = safe_get_symbol(buf, tx, vy);
                        let t_bg = if sym == "█" || sym == "▓" || sym == "▆" || sym == "▄" { safe_get_fg(buf, tx, vy) } else { safe_get_bg(buf, tx, vy) };
                        if v.v_type == VehicleType::Shuttle {
                            safe_set_char_with_bg(buf, tx, vy, ':', t_color, t_bg);
                            if tx >= area.x + 1 { 
                                let s2 = safe_get_symbol(buf, tx.saturating_sub(1), vy);
                                let t2_bg = if s2 == "█" || s2 == "▓" || s2 == "▆" || s2 == "▄" { safe_get_fg(buf, tx.saturating_sub(1), vy) } else { safe_get_bg(buf, tx.saturating_sub(1), vy) };
                                safe_set_char_with_bg(buf, tx.saturating_sub(1), vy, '·', t_color, t2_bg); 
                            }
                        } else {
                            safe_set_char_with_bg(buf, tx, vy, '·', t_color, t_bg);
                            if v.v_type == VehicleType::Spinner {
                                let t2x_f = v.x - 2.0;
                                if t2x_f >= 0.0 {
                                    let t2x = (area.x as f32 + t2x_f) as u16;
                                    if t2x < area.x + area.width { 
                                        let s2 = safe_get_symbol(buf, t2x, vy);
                                        let t2_bg = if s2 == "█" || s2 == "▓" || s2 == "▆" || s2 == "▄" { safe_get_fg(buf, t2x, vy) } else { safe_get_bg(buf, t2x, vy) };
                                        safe_set_char_with_bg(buf, t2x, vy, '·', Color::Rgb(85, 255, 255), t2_bg); 
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn render_weather_fg(&self, area: Rect, buf: &mut Buffer) {
        if self.weather == Weather::Rain {
            let ground_y = area.y + area.height.saturating_sub(3);
            for ry in (ground_y + 1)..(area.y + area.height) {
                let dist = ry.saturating_sub(ground_y); let sy = ground_y.saturating_sub(dist);
                let ripple = ((self.frame_count as f32 * 0.2 + ry as f32 * 0.5).sin() * 1.2) as i16;
                for rx in 0..area.width {
                    let target_x = area.x + rx;
                    let source_x = (area.x as i16 + rx as i16 + ripple).max(area.x as i16).min((area.x + area.width - 1) as i16) as u16;
                    let s_fg = safe_get_fg(buf, source_x, sy);
                    let s_bg = safe_get_bg(buf, source_x, sy);
                    let s_sym = safe_get_symbol(buf, source_x, sy);
                    if s_sym != " " || s_fg != Color::Reset {
                        let dim_fg = darken_color(s_fg); let dim_bg = darken_color(s_bg);
                        let sym = if dist == 1 { "█" } else if dist == 2 { "▓" } else { "▒" };
                        safe_set_symbol_with_bg(buf, target_x, ry, sym, dim_fg, dim_bg);
                    }
                }
            }
        }
    }

    fn render_diagnostics(&self, area: Rect, buf: &mut Buffer) {
        if self.debug_mode {
            let dx = area.x + 2; let dy = area.y + 2;
            let dg_color = Color::Rgb(85, 255, 85);
            let dim = Color::Rgb(60, 60, 60);
            let val_color = Color::White;

            // Header
            safe_set_string(buf, dx, dy, "╔══ DIAGNOSTICS ══╗", dg_color);

            // Original stats
            safe_set_string(buf, dx, dy + 1, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 1, "FRM:", dg_color);
            safe_set_string(buf, dx + 8, dy + 1, &format!("{:08}", self.frame_count), val_color);

            safe_set_string(buf, dx, dy + 2, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 2, "WTR:", dg_color);
            safe_set_string(buf, dx + 8, dy + 2, &format!("{:>8?}", self.weather), val_color);

            safe_set_string(buf, dx, dy + 3, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 3, "VHC:", dg_color);
            safe_set_string(buf, dx + 8, dy + 3, &format!("{:03}", self.vehicles.len()), val_color);

            safe_set_string(buf, dx, dy + 4, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 4, "PED:", dg_color);
            safe_set_string(buf, dx + 8, dy + 4, &format!("{:03}", self.people.len()), val_color);

            safe_set_string(buf, dx, dy + 5, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 5, "DRP:", dg_color);
            safe_set_string(buf, dx + 8, dy + 5, &format!("{:03}", self.raindrops.len()), val_color);

            // Separator
            safe_set_string(buf, dx, dy + 6, "╠══ PERF (µs) ════╣", dg_color);

            // Perf timings
            let warn = Color::Rgb(255, 200, 85);
            let hot = Color::Rgb(255, 85, 85);

            let upd = self.perf.update_us;
            let upd_avg = self.perf.avg_update_us();
            let upd_max = self.perf.max_update_us();
            let upd_color = if upd > 5000 { hot } else if upd > 2000 { warn } else { val_color };
            safe_set_string(buf, dx, dy + 7, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 7, "UPD:", dg_color);
            safe_set_string(buf, dx + 8, dy + 7, &format!("{:>6}", upd), upd_color);

            safe_set_string(buf, dx, dy + 8, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 8, " avg:", dim);
            safe_set_string(buf, dx + 8, dy + 8, &format!("{:>6}", upd_avg), Color::Rgb(170, 170, 170));
            safe_set_string(buf, dx + 15, dy + 8, &format!("p{:>3}", upd_max), dim);

            let drw = self.perf.draw_us;
            let drw_avg = self.perf.avg_draw_us();
            let drw_max = self.perf.max_draw_us();
            let drw_color = if drw > 10000 { hot } else if drw > 5000 { warn } else { val_color };
            safe_set_string(buf, dx, dy + 9, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 9, "DRW:", dg_color);
            safe_set_string(buf, dx + 8, dy + 9, &format!("{:>6}", drw), drw_color);

            safe_set_string(buf, dx, dy + 10, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 10, " avg:", dim);
            safe_set_string(buf, dx + 8, dy + 10, &format!("{:>6}", drw_avg), Color::Rgb(170, 170, 170));
            safe_set_string(buf, dx + 15, dy + 10, &format!("p{:>3}", drw_max), dim);

            let total = upd + drw;
            let total_avg = upd_avg + drw_avg;
            let total_color = if total > 16000 { hot } else if total > 8000 { warn } else { val_color };
            safe_set_string(buf, dx, dy + 11, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 11, "TOT:", dg_color);
            safe_set_string(buf, dx + 8, dy + 11, &format!("{:>6}", total), total_color);
            safe_set_string(buf, dx + 15, dy + 11, &format!("a{:>3}", total_avg / 1000), dim);

            // Sparkline - last 16 draw times as a mini bar chart
            safe_set_string(buf, dx, dy + 12, "╠══ DRAW HIST ════╣", dg_color);
            safe_set_string(buf, dx, dy + 13, "║                 ║", dim);
            safe_set_string(buf, dx, dy + 14, "║                 ║", dim);

            let bars: Vec<u64> = self.perf.draw_history()
                .map(|(_, v)| v)
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .take(16)
                .rev()
                .copied()
                .collect();

            if !bars.is_empty() {
                let max_val = *bars.iter().max().unwrap_or(&1).max(&1);
                let ticks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                for (i, &val) in bars.iter().enumerate() {
                    let norm = ((val as f64 / max_val as f64) * 7.0) as usize;
                    let tick = ticks[norm.min(7)];
                    let bar_color = if val > 10000 { hot } else if val > 5000 { warn } else { dg_color };
                    safe_set_char(buf, dx + 2 + i as u16, dy + 14, tick, bar_color);
                }
                // Scale label
                safe_set_string(buf, dx + 2, dy + 13, &format!("{:.0}ms", max_val as f64 / 1000.0), dim);
            }

            // Footer with budget indicator
            let budget_pct = (total_avg as f64 / 50000.0 * 100.0) as u64; // 50ms = 20fps budget
            let budget_color = if budget_pct > 80 { hot } else if budget_pct > 50 { warn } else { dg_color };
            safe_set_string(buf, dx, dy + 15, "║                 ║", dim);
            safe_set_string(buf, dx + 2, dy + 15, "BDG:", dg_color);
            safe_set_string(buf, dx + 8, dy + 15, &format!("{:>3}%/50ms", budget_pct), budget_color);

            safe_set_string(buf, dx, dy + 16, "╚═════════════════╝", dg_color);
        }
    }
}

impl Widget for &MetropolisCity {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 { return; }

        self.render_background(area, buf);

        if area.width < 12 || area.height < 3 {
            if area.width < 7 || area.height < 2 {
                self.render_tiny_indicator(area, buf);
            } else {
                self.render_mini_hud(area, buf);
            }
            return;
        }

        if area.width < 45 || area.height < 15 {
            self.render_mini_hud(area, buf);
            return;
        }

        self.render_stars(area, buf);
        self.render_skyline(area, buf);
        self.render_street_lamps(area, buf);
        self.render_weather_bg(area, buf);
        self.render_megaboard(area, buf);
        self.render_people(area, buf);
        self.render_vehicles(area, buf);
        self.render_weather_fg(area, buf);
        self.render_diagnostics(area, buf);
    }
}