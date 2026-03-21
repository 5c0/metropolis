use ratatui::style::Color;

#[derive(Clone)]
pub struct LogoPixel {
    pub ch: char,
    pub color: Color,
    pub bg: Color,
}

pub struct DistroLogo {
    pub grid: Vec<Vec<Option<LogoPixel>>>,
    pub is_compact: bool,
}

pub fn get_logo(distro: &str) -> DistroLogo {
    let d = distro.to_lowercase();
    let mut grid = vec![vec![None; 32]; 20];
    let b_bg = Color::Rgb(20, 20, 35);
    let mut is_compact = false;

    if d.contains("ubuntu") {
        is_compact = true;
        let white = Color::White;
        let orange = Color::Rgb(226, 88, 34);
        let lines = vec![
            "         _    ",
            "     ---(_)   ",
            " _/  ---  \\   ",
            "(_) |   |     ",
            "  \\  --- _/   ",
            "     ---(_)   ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = if ch == '_' || ch == '-' || ch == '/' || ch == '\\' || ch == '|' {
                        white
                    } else {
                        orange
                    };
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }

    } else if d.contains("debian") {
        is_compact = true;
        let red = Color::Rgb(215, 10, 83);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "  _____     ",
            " /  __ \\    ",
            "|  /    |   ",
            "|  \\___-    ",
            " -_          ",
            "   --_       ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = red;
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("freebsd") {
        is_compact = true;
        let red = Color::Rgb(204, 0, 0);
        let white = Color::White;
        let b_bg = Color::Rgb(20, 20, 35);

        let lines = vec![
            "/\\,-'''''-,/\\   ",
            "\\_)       (_/   ",
            "|           |   ",
            "|           |   ",
            " ;         ;    ",
            "  '-_____-'     ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let mut color = red;
                    if (y == 0 && (x <= 1 || x >= 11)) || 
                       (y == 1 && (x <= 2 || x >= 10)) {
                           color = white;
                    }

                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }

    } else if d.contains("openbsd") {
        is_compact = true;
        let yellow = Color::Yellow;
        let white = Color::White;
        let b_bg = Color::Rgb(20, 20, 35);

        let lines = vec![
            "      _____       ",
            "    \\-     -/     ",
            " \\_/         \\    ",
            " |        O O |   ",
            " |_  <   )  3 )   ",
            " / \\         /    ",
            "    /-_____-\\     ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let mut color = yellow;
                    if (y == 3 && (x == 10 || x == 12)) ||
                       (y == 4 && (x == 17 || x == 18 || x == 12 || x == 13 || x == 14)) ||
                       (y == 4 && ch == '<') ||
                       (y == 4 && ch == ')') ||
                       (y == 4 && ch == '3') ||
                       ch == 'O' {
                            color = white;
                    }

                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }

    } else if d.contains("arch") {  // I use Arch btw
        is_compact = true;
        let cyan = Color::Cyan;
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "       /\\       ",
            "      /  \\      ",
            "     /\\   \\     ",
            "    /      \\    ",
            "   /   ,,   \\   ",
            "  /   |  |   \\  ",
            " /_,,_    _,,_\\ ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = cyan;
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("windows") {
        is_compact = true;
        let blue = Color::Rgb(0, 180, 255);
        for y in 3..14 { 
            for x in 6..26 { 
                if x == 15 || y == 8 { continue; } 
                grid[y][x] = Some(LogoPixel { ch: '█', color: blue, bg: b_bg }); 
            } 
        }
    } else if d.contains("macos") || d.contains("apple") || d.contains("darwin") {
        is_compact = true;
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "       .:'     ",
            "    _ :'_      ",
            " .'`_`-'_``.   ",
            ":________.-'   ",
            ":_______:      ",
            ":_______:      ",
            " :_______`-;   ",
            "  `._.-._.'    ",
        ];

        let colors = [
            Color::Green,
            Color::Green,
            Color::Yellow,
            Color::Rgb(255, 165, 0),
            Color::Red,
            Color::Magenta,
            Color::Magenta,
            Color::Blue,
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2; 

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = colors[y % colors.len()];
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("fedora") {
        is_compact = true;
        let blue = Color::Rgb(60, 110, 255);
        let white = Color::White;
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "      _____     ",
            "     /   __)\\    ",
            "     |  /  \\ \\   ",
            "  ___|  |__/ /   ",
            " / (_    _)_/    ",
            "/ /  |  |        ",
            "\\ \\__/  |        ",
            " \\(_____/        ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = match y {
                        0 => white,
                        1 => if x <= 11 { white } else { blue },
                        2 => if x <= 10 { white } else { blue },
                        3 => if (x >= 4 && x <= 5) || (x >= 8 && x <= 9) { white } else { blue },
                        4 => if x >= 3 && x <= 10 { white } else { blue },
                        5 => if x >= 5 { white } else { blue },
                        6 => if (x >= 3 && x <= 5) || x >= 8 { white } else { blue },
                        7 => if x >= 2 { white } else { blue },
                        _ => white,
                    };
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("nixos") {
        let light_cyan = Color::Rgb(126, 186, 223);
        let dark_cyan = Color::Rgb(82, 119, 186);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "          ___   __              ",
            "   /#\\     \\QQ\\ /fy;            ",
            "   \\#+\\     \\lQvfy/             ",
            ",=#####=##+\\ \\QOy/   /,         ",
            "/+=#######=++\\ \\Qq\\  /+#;        ",
            "     ,——,       \\O/ /+#/_        ",
            "_____/fy/         ‘ /+###+\\       ",
            "\\QOOQfy/           /##/¯¯¯¯       ",
            " ¯¯/fy/ ,         /y#/            ",
            "  ,fy/ /+\\  _____________        ",
            "   \\Y  \\##\\ \\QQqQeeOoQQQy\\       ",
            "       /#|#\\ ‾‾‾‾‾\\EQ\\‾‾‾‾       ",
            "      ,+#,\\#\\      \\QQ\\          ",
            "      \\#/ \\##\\      \\Q/          ",
            "       ‾   ‾‾‾                  ",
        ];
        for (y, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            for (x, &ch) in chars.iter().enumerate() {
                if ch != ' ' {
                    let color = if "#+=/|Y_——".contains(ch) { dark_cyan } else { light_cyan };
                    let gx = x + 4;
                    if gx < 32 && y < 20 {
                        grid[y+2][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                    }
                }
            }
        }
    } else if d.contains("pop") {
        is_compact = true;
        let cyan = Color::Rgb(82, 187, 205);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "______              ",
            "\\   _ \\        __   ",
            " \\ \\ \\ \\      / /   ",
            "  \\ \\_\\ \\    / /    ",
            "   \\  ___\\  /_/     ",
            "    \\ \\    _        ",
            "   __\\_\\__(_)_      ",
            "  (___________)`    ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = cyan;
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("kali") {
        let kali_blue = Color::Rgb(85, 155, 180);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "  ÆÆÆ                        ",
            "          ÆÆÆÆÆÆ             ",
            "               Æ             ",
            "        ÆÆÆÆ    ÆÆ           ",
            "    ÆÆÆ        ÆÆÆÆÆÆÆÆÆÆ    ",
            "              ÆÆ       ÆÆÆ   ",
            "              ÆÆ          ÆÆ ",
            "              ÆÆÆÆ           ",
            "                 ÆÆÆÆÆÆÆ     ",
            "                       ÆÆÆ   ",
            "                          Æ  ",
            "                           Æ ",
            "                           ÆÆ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' && ch != '⠀' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = (20_usize.saturating_sub(art_height)) / 2;

            for (x, y, ch) in points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel { ch, color: kali_blue, bg: b_bg });
                }
            }
        }
    } else if d.contains("gentoo") {
        is_compact = true;
        let purple = Color::Rgb(125, 115, 180);
        let white = Color::White;
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            " _-----_ ",
            "(       \\",
            "\\    0   \\",
            " \\        )",
            " /      _/",
            "(     _- ",
            "\\____-   ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2; 

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = if ch == '0' { white } else { purple };
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("suse") {
        is_compact = true;
        let green = Color::Rgb(115, 186, 37);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "  _______     ",
            "__|   __ \\    ",
            "     / .\\ \\   ",
            "     \\__/ |   ",
            "   _______|   ",
            "   \\_______   ",
            "__________/   ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = green;
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("manjaro") {
        is_compact = true;
        let green = Color::Rgb(53, 191, 92);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "||||||||| ||||   ",
            "||||||||| ||||   ",
            "||||      ||||   ",
            "|||| |||| ||||   ",
            "|||| |||| ||||   ",
            "|||| |||| ||||   ",
            "|||| |||| ||||   ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = green;
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("void") {
        is_compact = true;
        let green = Color::Rgb(71, 128, 97);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "    _______     ",
            " _ \\______ -    ",
            "| \\  ___  \\ |   ",
            "| | /   \\ | |   ",
            "| | \\___/ | |   ",
            "| \\______ \\_|   ",
            " -_______\\      ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = green;
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("windows") {
        is_compact = true;
        let blue = Color::Rgb(13, 89, 127);
        let white = Color::White;
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "   /\\ /\\       ",
            "  // \\  \\      ",
            " //   \\  \\     ",
            "///    \\  \\    ",
            "//      \\  \\   ",
            "         \\     ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let mut color = blue;
                    if ch == '/' {
                        if (y == 1 && x == 3) ||
                           (y == 2 && x == 2) ||
                           (y == 3 && (x == 1 || x == 2)) ||
                           (y == 4 && (x == 0 || x == 1)) {
                            color = white;
                        }
                    }
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("centos") {
        is_compact = true;
        let yellow = Color::Yellow;
        let green = Color::Green;
        let cyan = Color::Cyan;
        let b_bg = Color::Rgb(20, 20, 35);
        
        let lines = vec![
            " ____^____    ",
            " |\\  |  /|    ",
            " | \\ | / |    ",
            "<---- ---->   ",
            " | / | \\ |    ",
            " |/__|__\\|    ",
            "     v        ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let mut color = Color::White;
                    
                    if y <= 2 && x <= 4 { color = green; }
                    else if y == 0 && ch == '^' { color = yellow; }
                    else if y <= 2 && x == 5 { color = yellow; }
                    else if y <= 2 && x >= 6 { color = cyan; }
                    else if y == 3 && x <= 4 { color = cyan; }
                    else if y == 3 && x >= 6 { color = yellow; }
                    else if y >= 4 && y <= 5 && x <= 4 { color = yellow; }
                    else if y >= 4 && y <= 5 && x == 5 { color = green; }
                    else if y >= 4 && y <= 5 && x >= 6 { color = yellow; }
                    else if y == 6 && ch == 'v' { color = green; }
                    

                    if y <= 2 && x <= 4 { color = Color::Green; }
                    else if y <= 2 && x == 5 { color = Color::Yellow; }
                    else if y <= 2 && x >= 6 { color = Color::Cyan; }
                    else if y == 3 && x <= 4 { color = Color::Cyan; }
                    else if y == 3 && x >= 6 { color = Color::Rgb(255, 165, 0); } // Orange (c3 equivalent)
                    else if y >= 4 && y <= 5 && x <= 4 { color = Color::Rgb(255, 165, 0); }
                    else if y >= 4 && y <= 5 && x == 5 { color = Color::Green; }
                    else if y >= 4 && y <= 5 && x >= 6 { color = Color::Yellow; }
                    else if y == 6 { color = Color::Green; }

                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("mint") {
        is_compact = true;
        let green = Color::Rgb(141, 198, 63);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            " ___________ ",
            "|_          \\",
            "  | | _____ |",
            "  | | | | | |",
            "  | | | | | |",
            "  | \\_____/ |",
            "  \\_________/",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel { ch, color: green, bg: b_bg });
                }
            }
        }
    } else if d.contains("android") {
        is_compact = true;
        let green = Color::Rgb(164, 198, 57);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "  ;,           ,; ",
            "   ';,.-----.,;'  ",
            "  ,'           ', ",
            " /    O     O    \\",
            "|                 |",
            "'-----------------'",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel { ch, color: green, bg: b_bg });
                }
            }
        }
    } else if d.contains("elementary") {
        is_compact = true;
        let blue = Color::Rgb(64, 150, 238);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "  _______  ",
            " / ____  \\ ",
            "/  |  /  /\\",
            "|__\\ /  / |",
            "\\   /__/  /",
            " \\_______/ ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel { ch, color: blue, bg: b_bg });
                }
            }
        }
    } else if d.contains("slackware") {
        is_compact = true;
        let blue = Color::Rgb(50, 100, 200);
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "   ________   ",
            "  /  ______|  ",
            "  | |______   ",
            "  \\______  \\  ",
            "   ______| |  ",
            "| |________/  ",
            "|____________ ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel { ch, color: blue, bg: b_bg });
                }
            }
        }
    } else if d.contains("parrot") {
        is_compact = true;
        let cyan = Color::Rgb(0, 255, 255);
        let white = Color::White;
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "       _------.      ",
            "      /  ,     \\_    ",
            "    /   /  /{}\\ |o\\_ ",
            "   /    \\  `--' /-' \\",
            "  |      \\      \\    |",
            " |              |`-, |",
            " /              /__/)/",
            "|              |      ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = if ch == '{' || ch == '}' || ch == 'o' || ch == ',' || ch == '`' || ch == '\'' {
                        white
                    } else {
                        cyan
                    };
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("endeavour") {
        is_compact = true;
        let c1 = Color::Rgb(255, 74, 90);   // Red
        let c2 = Color::Rgb(125, 124, 196); // Purple
        let c3 = Color::Rgb(0, 180, 255);   // Light Blue/Cyan
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "           /*",
            "         #***** ",
            "       #********#",
            "     ##**********##",
            "   ##*************##",
            " ###**************##",
            "   ###############",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let mut color = c1;
                    match y {
                        0 => color = c1,
                        1 => {
                            if x >= 10 && x <= 13 { color = c2; }
                            else if x == 14 { color = c3; }
                        }
                        2 => {
                            if x >= 8 && x <= 15 { color = c2; }
                            else if x == 16 { color = c3; }
                        }
                        3 => {
                            if x >= 7 && x <= 16 { color = c2; }
                            else if x >= 17 { color = c3; }
                        }
                        4 => {
                            if x >= 6 && x <= 17 { color = c2; }
                            else if x >= 18 { color = c3; }
                        }
                        5 => {
                            if x >= 5 && x <= 17 { color = c2; }
                            else if x >= 18 { color = c3; }
                        }
                        6 => {
                            color = c3;
                        }
                        _ => {}
                    }
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    } else if d.contains("rocky") || d.contains("linux") {
        is_compact = true;
        let green = Color::Rgb(16, 172, 80);
        let white = Color::White;
        let b_bg = Color::Rgb(20, 20, 35);
        let lines = vec![
            "        #####       ",
            "       #######      ",
            "       ##O#O##      ",
            "       #######      ",
            "     ###########    ",
            "    #############   ",
            "   ###############  ",
            "   ################ ",
            "  ################# ",
            "#####################",
            "#####################",
            "  ################# ",
        ];

        let mut min_x = 999; let mut max_x = 0;
        let mut min_y = 999; let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x); max_x = max_x.max(x);
                    min_y = min_y.min(y); max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 1;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel { ch: ' ', color: b_bg, bg: b_bg });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = if ch == 'O' { white } else { green };
                    grid[gy][gx] = Some(LogoPixel { ch, color, bg: b_bg });
                }
            }
        }
    }

    DistroLogo { grid, is_compact }
}
