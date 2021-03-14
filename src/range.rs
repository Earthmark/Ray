use glam::UVec2;

pub struct RangeIterator {
    min: UVec2,
    max: UVec2,
    cur: UVec2,
}

pub fn range(max: UVec2) -> RangeIterator {
    RangeIterator {
        min: UVec2::ZERO,
        max,
        cur: UVec2::ZERO,
    }
}

pub fn sub_range(min: UVec2, max: UVec2) -> RangeIterator {
    RangeIterator { min, max, cur: min }
}

impl Iterator for RangeIterator {
    type Item = UVec2;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.x >= self.max.x {
            self.cur.x = self.min.x;
            self.cur.y += 1;
        }
        if self.cur.y >= self.max.y {
            return None;
        }
        let result = self.cur;
        self.cur.x += 1;
        return Some(result);
    }
}

impl ExactSizeIterator for RangeIterator {
    fn len(&self) -> usize {
        let size = self.max - self.min;
        (size.x & size.y) as usize
    }
}
