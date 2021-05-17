// Apache 2.0 License

mod types;

//use rayon::prelude::*;
use std::{
    array::IntoIter as ArrayIter,
    collections::HashMap,
    fs::{self, File},
    io::{self, prelude::*, BufReader, BufWriter, ErrorKind},
    iter, mem,
    path::{Path, PathBuf},
};
use tinyvec::ArrayVec;
use types::{Flat, Material, Matinfo, Patch, Texture};

#[inline]
pub fn process_textures(
    pnames: &Path,
    texture1: &Path,
    patchfolder: PathBuf,
    flatfolder: PathBuf,
    wadinfo_in: &Path,
    wadinfo_out: &Path,
    matdir: &Path,
) -> crate::Result {
    // read in the matinfo from stdin
    let stdin = io::stdin();
    let mut cin = stdin.lock();
    let Matinfo {
        flats,
        patches,
        textures,
        ..
    } = Matinfo::load(&mut cin)?;
    mem::drop(cin);

    eprintln!("Read matinfo");

    let mut pname_file = BufWriter::new(File::create(pnames)?);
    let mut wadinfo_in = BufReader::new(File::open(wadinfo_in)?);
    let mut wadinfo_out = BufWriter::new(File::create(wadinfo_out)?);
    io::copy(&mut wadinfo_in, &mut wadinfo_out)?;

    eprintln!("copied wadinfo_in to wadinfo_out");

    // these operations involve a high degree of file I/O, so we use rayon to parallelize parts of it
    // first, write all the files we can to the flat directory
    fs::create_dir_all(&flatfolder)?;
    writeln!(wadinfo_out, "[flats]")?;
    flats
        .into_iter()
        .map(|Flat { name, mat }| {
            let fname = flatfolder.join(format!("{}.png", &name));
            // use a hard link to avoid copying the file
            match fs::hard_link(&matdir.join(&mat.file), &fname) {
                Ok(_) => (),
                Err(e) if e.kind() == ErrorKind::AlreadyExists => (),
                Err(e) => return Err(e.into()),
            }

            wadinfo_out.write_all(format!("{}\n", name).as_bytes())?;
            Ok(())
        })
        .collect::<crate::Result>()?;

    eprintln!("Wrote flats");

    // then, write all the patch names to a file. the patch lump consists of the number of patches (4 bytes)
    // followed by the eight-byte name for each patch name
    fs::create_dir_all(&patchfolder)?;
    writeln!(wadinfo_out, "[patches]")?;
    patches
        .into_iter()
        .enumerate()
        .map(move |(i, patch)| {
            let fname = patchfolder.join(format!("{}.png", &patch.name));
            let namebytes = patch
                .name
                .to_uppercase()
                .into_bytes()
                .into_iter()
                .chain(iter::repeat(0))
                .take(8)
                .collect::<ArrayVec<[u8; 8]>>()
                .into_inner();
            // TODO: transforms
            match fs::hard_link(&matdir.join(&patch.mat.file), &fname) {
                Ok(_) => (),
                Err(e) if e.kind() == ErrorKind::AlreadyExists => (),
                Err(e) => return Err(e.into()),
            }
            pname_file.write_all(&namebytes)?;

            writeln!(wadinfo_out, "{}", &patch.name)?;

            patch.assign_index(i);
            Ok(())
        })
        .collect::<crate::Result>()?;

    // finally, write the textures lump
    let mut textures_sum = vec![];
    let mut texture1_file = BufWriter::new(File::create(texture1)?);
    let num_textures = (textures.len() as u32).to_le_bytes();
    let mut offsets: Vec<u32> = vec![];
    let mut cur_rel_pos = 0;

    texture1_file.write_all(&num_textures)?;
    textures.into_iter().for_each(
        |Texture {
             name,
             width,
             height,
             patches,
         }| {
            offsets.push(cur_rel_pos);
            // write 8 ascii chars for the name
            textures_sum.extend(
                name.to_uppercase()
                    .into_bytes()
                    .into_iter()
                    .chain(iter::repeat(0))
                    .take(8),
            );
            // flag is unused
            textures_sum.extend(ArrayIter::new([0, 0]));
            // horizontal and vertical scaling
            textures_sum.extend(ArrayIter::new([8, 8]));
            // width and height
            let width = width.to_le_bytes();
            let height = height.to_le_bytes();
            textures_sum.extend(ArrayIter::new(width));
            textures_sum.extend(ArrayIter::new(height));
            // obsolete
            textures_sum.extend(ArrayIter::new([0, 0, 0, 0]));
            // number of patches we have
            let num_patches = patches.len() as u16;
            textures_sum.extend(ArrayIter::new(num_patches.to_le_bytes()));

            // at this point we should have extended textures_sum by 0x16 bytes
            cur_rel_pos += 0x16;

            // add the patches
            patches.into_iter().for_each(|(patch, width, height)| {
                // width and height
                textures_sum.extend(ArrayIter::new(width.to_le_bytes()));
                textures_sum.extend(ArrayIter::new(height.to_le_bytes()));
                // texture index
                textures_sum.extend(ArrayIter::new(patch.index().to_le_bytes()));
                // four unused bytes
                textures_sum.extend(iter::repeat(0).take(4));
                // we've advanced by  10 bytes
                cur_rel_pos += 0x10;
            });
        },
    );

    // make the positions absolute
    let offset = (offsets.len() as u32 * 4) + 4;
    let offsetbytes: Vec<u8> = offsets
        .into_iter()
        .flat_map(move |i| ArrayIter::new((i + offset).to_le_bytes()))
        .collect();
    texture1_file.write_all(&offsetbytes)?;
    texture1_file.write_all(&textures_sum)?;

    Ok(())
}
