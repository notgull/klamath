// Apache 2.0 License

use super::Octave;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

const HEADER: &[u8; 4] = b"SBI\x1A";

#[derive(Debug, Clone)]
pub struct Instrument {
    pub voice1: Voice,
    voice2: Option<Voice>,
    off1: isize,
    off2: isize,
    octave: Option<u8>,
}

impl Instrument {
    #[inline]
    pub fn load(
        p1: &Path,
        p2: Option<&Path>,
        off1: isize,
        off2: isize,
        octave: Option<u8>,
    ) -> crate::Result<Self> {
        Ok(Self {
            voice1: Voice::load(p1)?,
            voice2: match p2 {
                Some(p2) => Some(Voice::load(p2)?),
                None => None,
            },
            off1,
            off2,
            octave,
        })
    }

    #[inline]
    pub fn voice1(&self) -> &Voice {
        &self.voice1
    }

    #[inline]
    pub fn voice2(&self) -> Option<&Voice> {
        self.voice2.as_ref()
    }

    #[inline]
    pub fn offset1(&self) -> isize {
        self.off1
    }

    #[inline]
    pub fn offset2(&self) -> isize {
        self.off2
    }

    #[inline]
    pub fn octave(&self) -> Option<u8> {
        self.octave
    }
}

#[derive(Debug, Default, Clone)]
pub struct Voice {
    pub m_am_vibrato_eg: u8,
    pub c_am_vibrato_eg: u8,
    pub m_ksl_volume: u8,
    pub c_ksl_volume: u8,
    pub m_attack_decay: u8,
    pub c_attack_decay: u8,
    pub m_sustain_release: u8,
    pub c_sustain_release: u8,
    pub m_waveform: u8,
    pub c_waveform: u8,
    pub feedback_fm: u8,
    pub name: Vec<u8>,
}

impl Voice {
    #[inline]
    fn load<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let mut v = Self::default();
        let name = load_instrument(path, |name, value| {
            let field: &mut u8 = match name {
                "m_am_vibrato_eg" => &mut v.m_am_vibrato_eg,
                "c_am_vibrato_eg" => &mut v.c_am_vibrato_eg,
                "m_ksl_volume" => &mut v.m_ksl_volume,
                "c_ksl_volume" => &mut v.c_ksl_volume,
                "m_attack_decay" => &mut v.m_attack_decay,
                "c_attack_decay" => &mut v.c_attack_decay,
                "m_sustain_release" => &mut v.m_sustain_release,
                "c_sustain_release" => &mut v.c_sustain_release,
                "m_waveform" => &mut v.m_waveform,
                "c_waveform" => &mut v.c_waveform,
                "feedback_fm" => &mut v.feedback_fm,
                name => {
                    eprintln!("Unintelligible name: {}", name);
                    return;
                }
            };
        })?;
        v.name = name;
        Ok(v)
    }
}

#[inline]
fn load_instrument<P: AsRef<Path>, F: FnMut(&str, u8)>(
    path: P,
    mut operator: F,
) -> crate::Result<Vec<u8>> {
    const FIELDS: &[&str] = &[
        "m_am_vibrato_eg",
        "c_am_vibrato_eg",
        "m_ksl_volume",
        "c_ksl_volume",
        "m_attack_decay",
        "c_attack_decay",
        "m_sustain_release",
        "c_sustain_release",
        "m_waveform",
        "c_waveform",
        "feedback_fm",
    ];

    let mut data = vec![];
    let p: &Path = path.as_ref();
    let mut file =
        BufReader::new(File::open(p).unwrap_or_else(|_| panic!("Could not find file {:?}", p)));
    file.read_to_end(&mut data)?;

    let instrument_data = data.split_off(36);
    let name_data = data.split_off(4);

    if data.as_slice() != HEADER {
        return Err(crate::Error::StaticMsg("SBI file doesn't have SBI header"));
    }

    FIELDS.iter().enumerate().for_each(|(i, field)| {
        operator(*field, instrument_data[i].clone());
    });

    Ok(name_data)
}
