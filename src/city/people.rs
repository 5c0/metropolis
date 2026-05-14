use ratatui::style::Color;
use super::buildings::BuildingInfo;

#[derive(Debug, Clone)]
pub struct Person {
    pub x: f32,
    pub speed: f32,
    pub color: Color,
    pub id_offset: u64,
    pub is_entering: bool,
    pub entry_pause_timer: u8,
}

pub fn update_people(
    people: &mut Vec<Person>,
    frame_count: u64,
    area: ratatui::layout::Rect,
    theme: &crate::theme::Theme,
    config: &crate::config::SimulationConfig,
    buildings: &[BuildingInfo],
    rng: &mut impl rand::Rng,
) {
    if people.len() < config.max_pedestrians && frame_count % 15 == 0 {
        let dir = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let start_x = if dir > 0.0 { -2.0 } else { area.width as f32 + 2.0 };
        people.push(Person { x: start_x, speed: rng.gen_range(0.1..0.3) * dir, color: theme.pedestrian, id_offset: rng.gen_range(0..100), is_entering: false, entry_pause_timer: 0 });
    }
    people.retain_mut(|p| {
        if !p.is_entering {
            p.x += p.speed;
            let mut is_main = false; let mut near_door = false;
            for b in buildings {
                if (p.x as u16).abs_diff(b.door_x) < 1 {
                    near_door = true;
                    if b.index == 1 { is_main = true; }
                    break;
                }
            }
            if near_door && rng.gen_bool(0.02) {
                let chance = if is_main { 0.4 } else { 0.1 };
                if rng.gen_bool(chance) { p.is_entering = true; p.entry_pause_timer = 60; }
            }
        } else {
            if p.entry_pause_timer > 0 { p.entry_pause_timer -= 1; }
            else { return false; }
        }
        p.x > -5.0 && p.x < (area.width as f32 + 5.0)
    });
}