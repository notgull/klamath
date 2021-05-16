// Apache 2.0 License

#![allow(unused)]

use tinyvec::ArrayVec;

/// An octave
#[derive(Copy, Clone)]
pub struct Octave {
    inner: [u8; 17],
}

impl Octave {
    #[inline]
    pub const fn new(base: u8) -> Self {
        Self {
            inner: [
                base,
                base + 1,
                base + 1,
                base + 2,
                base + 3,
                base + 3,
                base + 4,
                base + 5,
                base + 6,
                base + 6,
                base + 7,
                base + 8,
                base + 8,
                base + 9,
                base + 10,
                base + 10,
                base + 11,
            ],
        }
    }

    #[inline]
    pub fn c(self) -> u8 {
        self.inner[0]
    }
    #[inline]
    pub fn cs(self) -> u8 {
        self.inner[1]
    }
    #[inline]
    pub fn db(self) -> u8 {
        self.inner[2]
    }
    #[inline]
    pub fn d(self) -> u8 {
        self.inner[3]
    }
    #[inline]
    pub fn ds(self) -> u8 {
        self.inner[4]
    }
    #[inline]
    pub fn eb(self) -> u8 {
        self.inner[5]
    }
    #[inline]
    pub fn e(self) -> u8 {
        self.inner[6]
    }
    #[inline]
    pub fn f(self) -> u8 {
        self.inner[7]
    }
    #[inline]
    pub fn fs(self) -> u8 {
        self.inner[8]
    }
    #[inline]
    pub fn gb(self) -> u8 {
        self.inner[9]
    }
    #[inline]
    pub fn g(self) -> u8 {
        self.inner[10]
    }
    #[inline]
    pub fn gs(self) -> u8 {
        self.inner[11]
    }
    #[inline]
    pub fn ab(self) -> u8 {
        self.inner[12]
    }
    #[inline]
    pub fn a(self) -> u8 {
        self.inner[13]
    }
    #[inline]
    pub fn r#as(self) -> u8 {
        self.inner[14]
    }
    #[inline]
    pub fn bd(self) -> u8 {
        self.inner[15]
    }
    #[inline]
    pub fn b(self) -> u8 {
        self.inner[16]
    }
}

pub static ON5: Octave = Octave::new(0);
pub static ON4: Octave = Octave::new(12);
pub static ON3: Octave = Octave::new(24);
pub static ON2: Octave = Octave::new(36);
pub static ON1: Octave = Octave::new(48);
pub static O0: Octave = Octave::new(60);
pub static O1: Octave = Octave::new(72);
pub static O2: Octave = Octave::new(84);
pub static O3: Octave = Octave::new(96);
pub static O4: Octave = Octave::new(108);
pub static O5: Octave = Octave::new(120);
