// Apache 2.0 License

use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path,
    rc::Rc,
};
use tinyvec::TinyVec;

/// Generate the DMXGUS lump for the WAD.
#[inline]
pub fn generate_dmxgus(config: &Path) -> crate::Result {
    // load the config file
    let mut config = BufReader::new(File::open(config)?);
    let config: Dmxgus = serde_yaml::from_reader(&mut config)?;

    // convert to instrument stats
    let stats: InstrumentStats = config.into();

    // get patchsets
    let patchsets = vec![
        patchset(256 * 1024, &stats),
        patchset(512 * 1024, &stats),
        patchset(768 * 1024, &stats),
        patchset(1024 * 1024, &stats),
    ];

    // write out patchsets
    let stdout = io::stdout();
    let mut cout = stdout.lock();
    write_patchsets(&mut cout, patchsets, stats)
}

/// Given a patch set, convert it to writeable form and write it to the output.
#[inline]
fn write_patchsets<W: Write>(
    w: &mut W,
    patchsets: Vec<HashMap<u16, u16>>,
    stats: InstrumentStats,
) -> crate::Result {
    // make sure the leader always comes first
    stats
        .instrument_groups
        .into_iter()
        .flat_map(|g| g.members)
        .filter_map(std::convert::identity)
        .map(|instrument| (instrument.midi_id, instrument.patch_name.clone()))
        .try_for_each::<_, crate::Result>(|(midi_id, patch_name)| {
            writeln!(
                w,
                "{}, {}, {}, {}, {}, {}",
                midi_id,
                patchsets[0].get(&midi_id).unwrap(),
                patchsets[1].get(&midi_id).unwrap(),
                patchsets[2].get(&midi_id).unwrap(),
                patchsets[3].get(&midi_id).unwrap(),
                patch_name
            )?;
            Ok(())
        })?;

    Ok(())
}

/// Calculate a patch set for the specified size
#[inline]
fn patchset(mut size: usize, stats: &InstrumentStats) -> HashMap<u16, u16> {
    // gus wants us to reserve 8K + 32 bytes for other stuff
    size = size.checked_sub((32 * 1024) + 8).expect("Can't fit it!");

    // create a patchset that patches every possible sound with a potential replacement
    let mut patchset: HashMap<u16, u16> = stats
        .instrument_groups
        .iter()
        .flat_map(|group| {
            let leader = group.leader().midi_id;
            group
                .members
                .iter()
                .filter_map(Option::as_ref)
                .map(move |ins| (ins.midi_id, leader))
        })
        .collect();

    // go through the patchset and replace midi instruments with their equivalents as long as we can keep it under
    // the desired size
    let mut current_size = patch_size(
        (&patchset)
            .into_iter()
            .map(|(i1, i2)| (stats.lookup(*i1).clone(), stats.lookup(*i2).clone())),
    );
    assert!(
        current_size < size,
        "Minimal patch set won't fit in {} bytes!",
        size
    );

    // instruments are already sorted by decreasing priority
    stats.instruments.iter().for_each(|instrument| {
        if patchset.get(&instrument.midi_id).copied().unwrap() != instrument.midi_id
            && instrument.patch_file_size + current_size < size
        {
            patchset.insert(instrument.midi_id, instrument.midi_id);
            current_size += instrument.patch_file_size;
        }
    });

    patchset
}

// Calculate the size of an image-to-image mapping
#[inline]
fn patch_size<I: IntoIterator<Item = (Rc<Instrument>, Rc<Instrument>)>>(i: I) -> usize {
    i.into_iter()
        .filter(|(i1, i2)| Rc::ptr_eq(i1, i2))
        .map(|(i, _)| i.patch_file_size)
        .sum()
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Dmxgus {
    gus_instr_patches: HashMap<u16, String>,
    patch_file_sizes: HashMap<String, usize>,
    similar_groups: Vec<Vec<String>>,
    instrument_stats: Vec<u16>,
}

impl From<Dmxgus> for InstrumentStats {
    #[inline]
    fn from(d: Dmxgus) -> InstrumentStats {
        let Dmxgus {
            gus_instr_patches,
            patch_file_sizes,
            similar_groups,
            mut instrument_stats,
        } = d;

        normalize_stats(&mut instrument_stats);

        let instruments = gus_instr_patches
            .into_iter()
            .map(|(midi_id, patch_name)| {
                let patch_file_size =
                    patch_file_sizes
                        .get(&patch_name)
                        .cloned()
                        .unwrap_or_else(|| {
                            panic!("Cannot find patch file size for patch {}", &patch_name)
                        });
                let usage_score = instrument_stats[midi_id as usize];
                (
                    patch_name.clone(),
                    Rc::new(Instrument {
                        midi_id,
                        patch_name: patch_name.into_boxed_str().into(),
                        patch_file_size,
                        usage_score,
                    }),
                )
            })
            .collect::<HashMap<String, Rc<Instrument>>>();

        let instrument_groups = similar_groups
            .into_iter()
            .map(|group| {
                let members = group
                    .into_iter()
                    .map(|iname| {
                        Some(instruments.get(&iname).cloned().unwrap_or_else(|| {
                            panic!("Cannot find member {} of instrument group", &iname)
                        }))
                    })
                    .collect();
                InstrumentGroup { members }
            })
            .collect::<Vec<_>>();

        let mut instruments: Box<[_]> = instruments.into_iter().map(|(_, i)| i).collect();

        // sort the instruments by their priority
        instruments.sort_by_key(|instrument| {
            // dont feel like using ordered_float, so just convert it to a fixed-point number
            let res = (instrument.usage_score as f32 / instrument.patch_file_size as f32) * 1e7;
            res as usize
        });
        instruments.reverse();

        Self {
            instruments,
            instrument_groups,
        }
    }
}

struct Instrument {
    midi_id: u16,
    patch_name: Rc<str>,
    patch_file_size: usize,
    usage_score: u16,
}

struct InstrumentGroup {
    members: TinyVec<[Option<Rc<Instrument>>; 8]>,
}

impl InstrumentGroup {
    #[inline]
    fn leader(&self) -> &Instrument {
        self.members[0].as_ref().unwrap()
    }
}

struct InstrumentStats {
    instruments: Box<[Rc<Instrument>]>,
    instrument_groups: Vec<InstrumentGroup>,
}

impl InstrumentStats {
    #[inline]
    fn lookup(&self, id: u16) -> &Rc<Instrument> {
        self.instruments
            .iter()
            .find(|i| i.midi_id == id)
            .expect("No matching id?")
    }
}

#[inline]
fn normalize_stats(stats: &mut [u16]) {
    const DIVIDE: usize = 128;
    let main_av = stats.iter().take(DIVIDE).map(|s| *s as f32).sum::<f32>() / DIVIDE as f32;
    let perc_av = stats.iter().skip(DIVIDE).map(|s| *s as f32).sum::<f32>() / DIVIDE as f32;
    stats.iter_mut().skip(DIVIDE).for_each(move |stat| {
        let s = *stat as f32;
        let r = (s * main_av) / perc_av;
        *stat = r as u16;
    });
}
