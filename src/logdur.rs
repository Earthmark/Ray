use std::fmt::{self, Debug};
use std::time::Instant;

pub struct ChroMark {
    inst: Instant,
}

impl ChroMark {
    pub fn new() -> ChroMark {
        ChroMark {
            inst: Instant::now(),
        }
    }
}

impl fmt::Debug for ChroMark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dur = Instant::now().duration_since(self.inst);
        dur.fmt(f)
    }
}

impl fmt::Display for ChroMark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dur = Instant::now().duration_since(self.inst);
        dur.fmt(f)
    }
}
