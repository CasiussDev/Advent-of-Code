const INIT_X: i16 = 1;
const PIXELS_PER_LINE: u16 = 40;
const LINE_COUNT: u16 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    NoOp,
    AddX(i16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    x: i16,
    prev_x: i16,
    cycle_count: u32,
    crt: CrtBeam,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrtBeam {
    curr_pixel_id: u16,
    picture: String,
}

impl Default for CrtBeam {
    fn default() -> Self {
        Self {
            curr_pixel_id: 0,
            picture: String::with_capacity((PIXELS_PER_LINE * LINE_COUNT + LINE_COUNT).into()),
        }
    }
}

impl CrtBeam {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run_cycle(&mut self, sprite_x: i16) {
        let curr_x_pos = (self.curr_pixel_id % PIXELS_PER_LINE) as i16;
        if (curr_x_pos - sprite_x).abs() <= 1 {
            self.picture.push('#');
        } else {
            self.picture.push('·');
        }

        if curr_x_pos == (PIXELS_PER_LINE - 1) as i16 {
            self.picture.push('\n');
        }

        self.curr_pixel_id += 1;
    }

    pub fn picture(&self) -> &str {
        self.picture.as_str()
    }
}

impl Instr {
    pub fn cycle_duration(self) -> u32 {
        match self {
            Instr::NoOp => 1,
            Instr::AddX(_) => 2,
        }
    }
}

impl Device {
    pub fn new() -> Self {
        Self {
            x: INIT_X,
            prev_x: INIT_X,
            cycle_count: 1,
            crt: Default::default()
        }
    }

    pub fn execute(&mut self, instr: Instr) {
        let instr_duration = instr.cycle_duration();

        for _ in 0..instr_duration {
            self.crt.run_cycle(self.x());
        }

        if let Instr::AddX(d) = instr {
            self.prev_x = self.x;
            self.x += d;
        }

        self.cycle_count += instr_duration;
    }

    pub fn x(&self) -> i16 {
        self.x
    }

    pub fn prev_x(&self) -> i16 {
        self.prev_x
    }

    pub fn cycle_count(&self) -> u32 {
        self.cycle_count
    }

    pub fn picture(&self) -> &str {
        self.crt.picture()
    }
}
