use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::f64::consts::PI;

// interpolation step size
const MM_PER_SEGMENT: f64 = 1.0;

#[derive(Clone, Debug)]
pub enum Command {
    G20,
    G21,
    N(isize),
    M(isize),
    J(isize),
    K(isize),
    X(f64),
    Y(f64),
}

#[derive(PartialEq)]
enum Direction {
    CW,
    CCW,
}

pub struct FileLoader {
    commands: Vec<Command>,
    iterator: usize,
}

impl FileLoader {
    pub fn new() -> Self {
        FileLoader {
            commands: Vec::new(),
            iterator: 0,
        }
    }

    pub fn parse(&mut self, path: String) {
        let mut file = File::open(path).unwrap();
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        let lines: Vec<&str> = text.lines().filter(|x| x.len() > 0).collect();

        let mut x_pos = 0.0;
        let mut y_pos = 0.0;

        for line in lines {
            let mut words: Vec<&str> = line.split(' ').collect();
            let mut raw_gcode = words.remove(0).to_string();
            let mut gcode = -1;
            if raw_gcode.remove(0) == 'G' {
                gcode = isize::from_str(&raw_gcode).unwrap();
            }

            match gcode {
                0 | 1 => {
                    for word in words {
                        let mut word = word.to_string();
                        match word.remove(0) {
                            'X' => {
                                let pos = f64::from_str(&word).unwrap();
                                x_pos = pos;
                                self.commands.push(Command::X(pos));
                            }
                            'Y' => {
                                let pos = f64::from_str(&word).unwrap();
                                y_pos = pos;
                                self.commands.push(Command::Y(pos));
                            }
                            'N' => self.commands.push(Command::N(isize::from_str(&word).unwrap())),
                            'M' => self.commands.push(Command::M(isize::from_str(&word).unwrap())),
                            'J' => self.commands.push(Command::J(isize::from_str(&word).unwrap())),
                            'K' => self.commands.push(Command::K(isize::from_str(&word).unwrap())),
                            _ => {}
                        }
                    }
                }
                2 | 3 => {
                    let mut x = 0.0;
                    let mut y = 0.0;
                    let mut i = 0.0;
                    let mut j = 0.0;
                    let dir = match gcode {
                        2 => Direction::CW,
                        3 => Direction::CCW,
                        _ => unreachable!(),
                    };
                    for word in words {
                        let mut word = word.to_string();
                        match word.remove(0) {
                            'X' => x = f64::from_str(&word).unwrap(),
                            'Y' => y = f64::from_str(&word).unwrap(),
                            'I' => i = f64::from_str(&word).unwrap(),
                            'J' => j = f64::from_str(&word).unwrap(),
                            _ => {}
                        }
                    }
                    let mut inter = arc_inter(x_pos, y_pos, x, y, i, j, dir);
                    self.commands.append(&mut inter);
                    x_pos = x;
                    y_pos = y;
                }
                20 => self.commands.push(Command::G20),
                21 => self.commands.push(Command::G21),
                _ => {}
            }

        }
    }
}

impl Iterator for FileLoader {
    type Item = Command;
    fn next(&mut self) -> Option<Command> {
        let iter = self.iterator;
        if iter < self.commands.len() {
            self.iterator += 1;
            Some(self.commands[iter].clone())
        } else {
            None
        }
    }
}

fn arc_inter(sx: f64, sy: f64, x: f64, y: f64, cx: f64, cy: f64, dir: Direction) -> Vec<Command> {
    let mut result = Vec::new();

    // damn perverts with relative coordinate system...
    let cx = cx + sx;
    let cy = cy + sy;

    // get radius of arc
    let dx = sx - cx;
    let dy = sy - cy;
    let radius = (dx.powf(2.0) + dy.powf(2.0)).sqrt();

    // get sweep of arc
    let mut angle1 = atan3(dy, dx);
    let mut angle2 = atan3(y - cy, x - cx);
    let mut sweep = angle2 - angle1;

    // some strange magic of trigonometry
    if dir == Direction::CCW && sweep < 0.0 {
        angle2 += PI * 2.0;
    } else if dir == Direction::CW && sweep > 0.0 {
        angle1 += PI * 2.0;
    }
    sweep = angle2 - angle1;

    let len = sweep.abs() * radius;

    let num_segments = (len / MM_PER_SEGMENT) as usize;

    // strange magic of interpolation
    for i in 0..num_segments {
        let fraction = i as f64 / num_segments as f64;
        let angle3 = (sweep * fraction) + angle1;

        let next_x = cx + angle3.cos() * radius;
        let next_y = cy + angle3.sin() * radius;
        result.push(Command::X(next_x));
        result.push(Command::Y(next_y));
    }

    // go to end point of arc
    result.push(Command::X(x));
    result.push(Command::Y(y));

    result
}

fn atan3(dy: f64, dx: f64) -> f64 {
    let mut a = dy.atan2(dx);
    if a < 0.0 {
        a = (PI * 2.0) + a;
    }
    a
}

impl Command {
    pub fn to_string(&self) -> String {
        match *self {
            Command::G20 => "G20".into(),
            Command::G21 => "G21".into(),
            Command::X(x) => format!("X{}", x).into(),
            Command::Y(y) => format!("Y{}", y).into(),
            Command::N(v) => format!("N{}", v).into(),
            Command::M(v) => format!("M{}", v).into(),
            Command::J(v) => format!("J{}", v).into(),
            Command::K(v) => format!("K{}", v).into(),
        }
    }
}