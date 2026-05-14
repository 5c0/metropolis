use ratatui::buffer::Buffer;
use ratatui::style::Color;
use super::utils::*;

#[derive(Debug, Clone, Copy)]
pub struct BuildingInfo {
    /// Left edge relative to area (the x_base value from step_by(20))
    pub x_offset: u16,
    pub width: u16,
    pub height: u16,
    /// Original index in the step_by(20) enumeration
    pub index: usize,
    /// Door x position relative to area (x_offset + width/2)
    pub door_x: u16,
    /// Whether the next slot is consumed (i==1 and i==3 span two slots)
    pub spans_two: bool,
}

impl BuildingInfo {
    /// Check if a given x position (relative to area) falls within this building
    pub fn contains_x(&self, x: u16) -> bool {
        x >= self.x_offset && x < self.x_offset + self.width
    }
}

pub fn compute_buildings(area_width: u16, area_height: u16) -> Vec<BuildingInfo> {
    let mut buildings = Vec::with_capacity(16);
    let mut skip_next = false;
    let ground_y = area_height.saturating_sub(3);

    for (i, x_base) in (0..area_width).step_by(20).enumerate() {
        if skip_next { skip_next = false; continue; }

        let mut bw = 8 + (x_base % 7) as u16;
        let mut bh = (area_height / 3) + (x_base % 11) as u16;
        let mut spans_two = false;

        if i == 1 {
            bw = 32;
            bh = area_height.saturating_sub(8);
            spans_two = true;
            skip_next = true;
        }
        if i == 3 {
            bw = 28;
            spans_two = true;
            skip_next = true;
            // bh stays as the default formula
        }

        // Clamp height so building doesn't exceed ground
        if bh > ground_y {
            bh = ground_y;
        }

        buildings.push(BuildingInfo {
            x_offset: x_base,
            width: bw,
            height: bh,
            index: i,
            door_x: x_base + bw / 2,
            spans_two,
        });
    }

    buildings
}

pub fn draw_roof_megaboard(
    buf: &mut Buffer, 
    x: u16, y: u16, 
    cpu: f32, ram: f32, 
    cpu_str: &str, ram_str: &str, 
    cpu_color: Color, ram_color: Color,
    frame: u64
) {
    let dim_cpu = darken_color(cpu_color);
    let dim_ram = darken_color(ram_color);
    
    let strut_pulse = ((frame as f32 * 0.05).sin() + 1.0) * 0.5;
    let strut_color = if strut_pulse > 0.8 { Color::Rgb(170, 170, 170) } else { Color::Rgb(85, 85, 85) };
    safe_set_symbol(buf, x + 4, y.saturating_sub(1), "╨", strut_color);
    safe_set_symbol(buf, x + 20, y.saturating_sub(1), "╨", strut_color);
    
    let board_y = y.saturating_sub(6);
    let width = 26;
    let height = 5;

    for dx in x..x+width {
        safe_set_symbol(buf, dx, board_y, "┄", Color::Rgb(170, 170, 170));
        safe_set_symbol(buf, dx, board_y + height - 1, "┄", Color::Rgb(170, 170, 170));
    }
    safe_set_symbol(buf, x, board_y, "⌜", Color::Rgb(170, 170, 170));
    safe_set_symbol(buf, x + width - 1, board_y, "⌝", Color::Rgb(170, 170, 170));
    safe_set_symbol(buf, x, board_y + height - 1, "⌞", Color::Rgb(170, 170, 170));
    safe_set_symbol(buf, x + width - 1, board_y + height - 1, "⌟", Color::Rgb(170, 170, 170));

    safe_set_string(buf, x + 2, board_y + 1, "CPU", Color::Rgb(170, 170, 170));
    let cpu_bars = (cpu / 6.6).min(15.0) as usize;
    for i in 0..15 {
        let bx = x + 7 + i as u16;
        let char = if i < cpu_bars { "❘" } else { " " };
        let color = if i < cpu_bars { cpu_color } else { dim_cpu };
        safe_set_symbol(buf, bx, board_y + 1, char, color);
    }
    safe_set_string(buf, x + 23, board_y + 1, cpu_str, Color::White);

    safe_set_string(buf, x + 2, board_y + 3, "MEM", Color::Rgb(170, 170, 170));
    let ram_bars = (ram / 6.6).min(15.0) as usize;
    for i in 0..15 {
        let bx = x + 7 + i as u16;
        let char = if i < ram_bars { "❘" } else { " " };
        let color = if i < ram_bars { ram_color } else { dim_ram };
        safe_set_symbol(buf, bx, board_y + 3, char, color);
    }
    safe_set_string(buf, x + 23, board_y + 3, ram_str, Color::White);
}

pub fn draw_neon_sign(buf: &mut Buffer, x: u16, y: u16, text: &str, color: Color, frame: u64) {
    if x >= buf.area.width { return; }
    for (i, ch) in text.chars().enumerate() {
        let dy = y.saturating_add(i as u16);
        if dy < buf.area.height {
            let seed = (frame / 2).wrapping_add(i as u64).wrapping_add(x as u64);
            let noise = seed.wrapping_mul(11400714819323198485);
            let final_color = if (noise % 100) < 95 { 
                color 
            } else { 
                Color::Rgb(30, 30, 45) 
            };
            
            safe_set_char(buf, x, dy, ch, final_color);
        }
    }
}