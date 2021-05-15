// Apache 2.0 License

use std::{
    array::IntoIter as ArrayIter,
    borrow::Cow,
    io::{self, prelude::*},
};
use tinyvec::ArrayVec;

const NUM_LUMPS: usize = 3;

// DeuTeX requires another IWAD to be present in order to build IWADs, for some reason. In order to circumvent
// this limitation, this script manually generates the bare-minimum IWAD to be fed into DeuTeX.
pub fn write_bootstrap() -> crate::Result {
    // stdin needs to not be a tty
    if atty::is(atty::Stream::Stdin) {
        return Err(crate::Error::StaticMsg("Stdin needs to be a file"));
    }

    // playpal is put in
    let mut playpal_bytes = vec![];
    let stdin = io::stdin();
    let mut cin = stdin.lock();

    cin.read_to_end(&mut playpal_bytes)?;

    // we need to write out a minimum of the three lumps DeuTeX needs:
    //  * a valid pallette
    //  * an empty TEXTURE1 lump
    //  * a PNAMES with one lump in it (a null lump)
    // note that lump names need to be exactly 8 bytes in length, hence why the names are
    // padded
    let lumps: [Lump; NUM_LUMPS] = [
        Lump {
            name: b"PLAYPAL\0",
            data: playpal_bytes.into(),
        },
        Lump {
            name: b"TEXTURE1",
            data: (&[0u8, 0, 0, 0]).as_ref().into(),
        },
        Lump {
            name: b"PNAMES\0\0",
            data: (&[1u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).as_ref().into(),
        },
    ];

    let mut pos = 12;
    let waddir: ArrayVec<[Waddir; NUM_LUMPS]> = lumps
        .iter()
        .map(|lump| {
            let len = lump.data.len();
            let w = Waddir {
                pos,
                len,
                name: lump.name,
            };
            pos += len;
            w
        })
        .collect();

    // write the header for the wad
    let stdout = io::stdout();
    let mut cout = stdout.lock();

    cout.write_all(b"IWAD")?; // four-byte IWAD header
    cout.write_all(&[lumps.len() as u8, 0, 0, 0])?; // number of lumps
    cout.write_all(&[pos as u8, 0, 0, 0])?; // final position

    // write the lumps
    ArrayIter::new(lumps)
        .map(|lump| cout.write_all(&lump.data))
        .collect::<io::Result<()>>()?;

    // write the waddir
    waddir
        .into_iter()
        .map(|waddir| {
            cout.write_all(&[waddir.pos as u8, 0, 0, 0])?;
            cout.write_all(&[waddir.len as u8, 0, 0, 0])?;
            cout.write_all(waddir.name)
        })
        .collect::<io::Result<()>>()?;

    Ok(())
}

struct Lump {
    name: &'static [u8],
    data: Cow<'static, [u8]>,
}

#[derive(Default)]
struct Waddir {
    pos: usize,
    len: usize,
    name: &'static [u8],
}
