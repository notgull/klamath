// Apache 2.0 License

use std::{
    array::IntoIter as ArrayIter,
    cmp,
    fs::File,
    io::{self, prelude::*},
    iter,
    path::Path,
    str::FromStr,
};

/// Generate the COLORMAP lump.
#[inline]
pub fn generate_colormap(dark_color: [u8; 3]) -> crate::Result {
    if atty::is(atty::Stream::Stdin) {
        return Err(crate::Error::StaticMsg("Stdin needs to be a file"));
    }

    // the input should be the palette
    let stdin = io::stdin();
    let mut cin = stdin.lock();
    let palette = read_palette(cin)?.collect::<crate::Result<Vec<_>>>()?;

    // create a list that's nothing but the dark color
    let dark_color = iter::repeat(dark_color).take(256);

    // create the color palette and write to the stdout
    let stdout = io::stdout();
    let mut cout = stdout.lock();
    //  * the first 32 are a variety of darkened palettes
    write_output(
        &mut cout,
        (0..32)
            .flat_map(|i| {
                let factor = (32.0 - i as f32) / (32.0);
                generate_from_palette(
                    blend_colors(dark_color.clone(), palette.iter().copied(), factor),
                    palette.iter().copied(),
                )
            })
            .chain(
                // the 33rd is the inverted color scheme
                generate_from_palette(
                    invert_colors(palette.iter().copied()),
                    palette.iter().copied(),
                ),
            )
            .chain(
                // the 34th is just the color black
                generate_from_palette(dark_color.clone(), palette.iter().copied()),
            ),
    )
}

// write a color map to an output
#[inline]
fn write_output<W: Write, I: IntoIterator<Item = u8>>(w: &mut W, i: I) -> crate::Result {
    let end: Vec<u8> = i.into_iter().collect();
    w.write_all(&end)?;
    Ok(())
}

// generate a colormap from a list of colors and a palette
#[inline]
fn generate_from_palette<
    CI: IntoIterator<Item = [u8; 3]>,
    PI: IntoIterator<Item = [u8; 3]> + Clone,
>(
    ci: CI,
    pi: PI,
) -> Vec<u8> {
    ci.into_iter()
        .map(move |color| search_for_closest(pi.clone(), color))
        .collect()
}

// tint a list of colors according to another color and a brightness factor
#[inline]
fn tint_colors<I: IntoIterator<Item = [u8; 3]>>(
    i: I,
    clr: &[u8; 3],
    factor: f32,
) -> impl Iterator<Item = [u8; 3]> {
    bytify(floatify(i).map(move |[r, g, b]| {
        let a = (r + g + b) * factor;
        let b = 255.0;
        let intensity = if a < b { a } else { b } / 255.0;
        [r * intensity, g * intensity, b * intensity]
    }))
}

// given two lists of colors, blend them together
#[inline]
fn blend_colors<I1: IntoIterator<Item = [u8; 3]>, I2: IntoIterator<Item = [u8; 3]>>(
    i1: I1,
    i2: I2,
    factor: f32,
) -> impl Iterator<Item = [u8; 3]> {
    bytify(
        floatify(i1)
            .zip(floatify(i2))
            .map(move |([r1, g1, b1], [r2, g2, b2])| {
                macro_rules! cvt {
                    ($e1: expr, $e2: expr, $f: expr) => {{
                        (($e1) * $f) + (($e2) * (1.0 - $f))
                    }};
                }

                [
                    cvt!(r1, r2, factor),
                    cvt!(g1, g2, factor),
                    cvt!(b1, b2, factor),
                ]
            }),
    )
}

// given a list of colors, invert them
#[inline]
fn invert_colors<I: IntoIterator<Item = [u8; 3]>>(i: I) -> impl Iterator<Item = [u8; 3]> {
    bytify(floatify(i).map(|[r, g, b]| {
        // blatantly stoled from Freedoom's code
        let greyscaled = (r * 0.2126) + (g * 0.7152) + (b * 0.0722);
        let inverse = 255.0 - greyscaled;
        [inverse, inverse, inverse]
    }))
}

// given a palette and a color, look for the color with the least amount of difference
#[inline]
fn search_for_closest<I: IntoIterator<Item = [u8; 3]>>(i: I, search: [u8; 3]) -> u8 {
    let [r2, g2, b2] = search;
    i.into_iter()
        .enumerate()
        .min_by_key(|(_, [r1, g1, b1])| {
            let rd = abs_diff(*r1, r2) as u32;
            let gd = abs_diff(*g1, g2) as u32;
            let bd = abs_diff(*b1, b2) as u32;
            (rd * rd) + (gd * gd) + (bd * bd)
        })
        .expect("Palette is empty?")
        .0 as u8
}

#[inline]
fn read_palette<R: Read>(r: R) -> crate::Result<impl Iterator<Item = crate::Result<[u8; 3]>>> {
    // custom iterator that groups elements into groups of 3 and is also result-aware
    struct GroupByThrees<I> {
        inner: I,
    }

    impl<T, E, I: Iterator<Item = std::result::Result<T, E>>> Iterator for GroupByThrees<I> {
        type Item = std::result::Result<[T; 3], E>;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            match self.inner.next() {
                None => None,
                Some(Err(e)) => Some(Err(e)),
                Some(Ok(b1)) => match self.inner.next() {
                    None => None,
                    Some(Err(e)) => Some(Err(e)),
                    Some(Ok(b2)) => match self.inner.next() {
                        None => None,
                        Some(Err(e)) => Some(Err(e)),
                        Some(Ok(b3)) => Some(Ok([b1, b2, b3])),
                    },
                },
            }
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            let (l, h) = self.inner.size_hint();
            (l / 3, h.map(|h| h / 3))
        }
    }

    Ok(GroupByThrees {
        inner: r.bytes().map(|b| {
            let b = b?;
            crate::Result::Ok(b)
        }),
    }
    .take(256))
}

#[inline]
fn abs_diff(a: u8, b: u8) -> u8 {
    let (l, h) = (cmp::min(a, b), cmp::max(a, b));
    h - l
}

#[inline]
fn floatify<I: IntoIterator<Item = [u8; 3]>>(i: I) -> impl Iterator<Item = [f32; 3]> {
    i.into_iter()
        .map(|[r, g, b]| [r as f32, g as f32, b as f32])
}

#[inline]
fn bytify<I: IntoIterator<Item = [f32; 3]>>(i: I) -> impl Iterator<Item = [u8; 3]> {
    #[inline]
    fn cvt(f: f32) -> u8 {
        if f < 0.0 {
            0
        } else if f > 255.0 {
            255
        } else {
            f as u8
        }
    }

    i.into_iter().map(|[r, g, b]| [cvt(r), cvt(g), cvt(b)])
}
