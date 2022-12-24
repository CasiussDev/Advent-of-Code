const INIT_X: i16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    NoOp,
    AddX(i16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Device {
    x: i16,
    prev_x: i16,
    cycle_count: u32,
}

pub struct CrtBeam {
    pixel_id: u16,
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
        }
    }

    pub fn execute(&mut self, instr: Instr) {
        if let Instr::AddX(d) = instr {
            self.prev_x = self.x;
            self.x += d;
        }

        self.cycle_count += instr.cycle_duration();
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
}
