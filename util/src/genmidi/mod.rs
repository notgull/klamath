// Apache 2.0 License

mod instrument;
mod list;
mod midi;

pub use instrument::{Instrument, Voice};
pub use midi::*;

use std::{
    convert::TryInto,
    io::{self, prelude::*},
    iter,
    path::Path,
};

const HEADER: &[u8; 8] = b"#OPL_II#";

/// Create the Genmidi Lump
#[inline]
pub fn generate_genmidi<P: AsRef<Path>>(basedir: P) -> crate::Result {
    let stdout = io::stdout();
    let mut cout = stdout.lock();
    cout.write_all(HEADER)?;

    let instruments = list::iter_instruments(basedir.as_ref())
        .chain(list::iter_percussion(basedir.as_ref()))
        .collect::<crate::Result<Vec<_>>>()?;
    let null_instrument = Instrument::load(&basedir.as_ref().join("dummy.sbi"), None, 0, 0, None)?;

    //    eprintln!("Instruments: {:#?}", &instruments);

    instruments
        .iter()
        .cloned()
        .try_for_each::<_, crate::Result>(|i| {
            encode_instrument(&mut cout, i, null_instrument.voice1())
        })?;

    instruments
        .into_iter()
        .try_for_each::<_, crate::Result>(|i| {
            let name = i.voice1.name;
            let namedata = name
                .into_iter()
                .chain(iter::repeat(0))
                .take(32)
                .collect::<Vec<u8>>();
            cout.write_all(&namedata)?;
            Ok(())
        })?;

    cout.flush()?;
    Ok(())
}

#[inline]
fn encode_instrument<W: Write>(
    w: &mut W,
    instrument: Instrument,
    null_voice: &Voice,
) -> crate::Result {
    const FLAG_TWO_VOICE: i16 = 0x0004;
    const FLAG_FIXED_PITCH: i16 = 0x0001;

    let mut flags = 0;
    let voice2 = instrument.voice2();
    let octave = instrument.octave();

    if voice2.is_some() {
        flags |= FLAG_TWO_VOICE;
    }
    if octave.is_some() {
        flags |= FLAG_FIXED_PITCH;
    }

    let flags = flags.to_le_bytes();
    w.write_all(&flags)?;
    w.write_all(&[128])?;
    w.write_all(&[octave.unwrap_or(0)])?;

    encode_voice(
        w,
        instrument.voice1(),
        instrument.offset1().try_into().unwrap(),
    )?;

    if let Some(voice2) = voice2 {
        encode_voice(w, voice2, instrument.offset2().try_into().unwrap())?;
    } else {
        encode_voice(w, null_voice, 0)?;
    }

    Ok(())
}

#[inline]
fn encode_voice<W: Write>(w: &mut W, v: &Voice, offset: i16) -> crate::Result {
    const KSL_MASK: u8 = 0xC0;
    const VOLUME_MASK: u8 = 0x3F;

    let bytes = [
        v.m_am_vibrato_eg,
        v.m_attack_decay,
        v.m_sustain_release,
        v.m_waveform,
        v.m_ksl_volume & KSL_MASK,
        v.m_ksl_volume & VOLUME_MASK,
        v.feedback_fm,
        v.c_am_vibrato_eg,
        v.c_attack_decay,
        v.c_sustain_release,
        v.c_waveform,
        v.c_ksl_volume & KSL_MASK,
        v.c_ksl_volume & VOLUME_MASK,
        0,
    ];
    let bytes2 = offset.to_le_bytes();

    w.write_all(&bytes)?;
    w.write_all(&bytes2)?;
    Ok(())
}
