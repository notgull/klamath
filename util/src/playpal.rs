// Apache 2.0 License

use std::{
    array::IntoIter as ArrayIter,
    io::{self, prelude::*},
};

/// Generate the color pallete
#[inline]
pub fn generate_palette() -> crate::Result {
    let palette: Vec<[f32; 3]> = default_palette().collect();
    let expanded_palette: Vec<[u8; 3]> = palette
        .iter()
        .copied()
        .chain((0..8).flat_map(|i| {
            bias_palette(
                palette.iter().copied(),
                [1.0, 0.0, 0.0],
                (i as f32 + 1.0) * 0.9 / 8.0,
            )
        }))
        .chain((0..4).flat_map(|i| {
            bias_palette(
                palette.iter().copied(),
                [0.839, 0.729, 0.271],
                (i as f32 + 1.0) * 0.5 / 4.0,
            )
        }))
        .chain(bias_palette(
            palette.iter().copied(),
            [0.0, 1.0, 0.0],
            0.125,
        ))
        .map(|[r, g, b]| [saturate_byte(r), saturate_byte(g), saturate_byte(b)])
        .collect();

    let stdout = io::stdout();
    let mut cout = stdout.lock();

    write_palette(&mut cout, expanded_palette)
}

/// Take a palette and bias it in a certain direction.
#[inline]
fn bias_palette<I: IntoIterator<Item = [f32; 3]>>(
    palette: I,
    target: [f32; 3],
    bias: f32,
) -> impl Iterator<Item = [f32; 3]> {
    #[inline]
    fn bias_rgb(mut rgb: [f32; 3], target: &[f32; 3], bias: f32) -> [f32; 3] {
        for i in 0..3 {
            rgb[i] = (rgb[i] * (1.0 - bias)) + (target[i] * bias);
        }

        rgb
    }

    palette
        .into_iter()
        .map(move |rgb| bias_rgb(rgb, &target, bias))
}

#[inline]
fn write_palette<W: Write>(w: &mut W, palette: impl IntoIterator<Item = [u8; 3]>) -> crate::Result {
    palette
        .into_iter()
        //        .inspect(|rgb| eprintln!("Writing rgb: {:?}", rgb))
        .flat_map(|rgb| ArrayIter::new(rgb))
        .map(|b| w.write_all(&[b]))
        .collect::<io::Result<()>>()?;
    w.flush()?;
    Ok(())
}

#[inline]
fn default_palette() -> impl Iterator<Item = [f32; 3]> {
    // we add the important colors here:
    static IMPORTANT_COLORS: &[([f32; 3], usize)] = &[
        // just black
        ([0.0, 0.0, 0.0], 1),
        // 47 shades of white
        ([1.0, 1.0, 1.0], 47),
        // 16 shades of red
        ([1.0, 0.0, 0.0], 16),
        // 16 shades of orange
        ([1.0, 0.64, 0.0], 16),
        // 16 shades of yellow
        ([1.0, 1.0, 0.0], 16),
        // 16 shades of green
        ([0.0, 1.0, 0.0], 16),
        // 16 shades of blue
        ([0.0, 0.0, 1.0], 16),
        // 16 shades of purple
        ([0.75, 0.0, 0.75], 16),
        // 64 shades of brown
        ([0.96, 0.73, 0.28], 64),
        // 16 shades of pink
        ([1.0, 0.63, 0.97], 16),
        // 16 shades of rusty metal
        ([1.0, 0.74, 0.75], 16),
        // 16 shades of acquamrine
        ([0.0, 1.0, 1.0], 16),
    ];

    let collective_len: usize = IMPORTANT_COLORS
        .iter()
        .map(|(_, num_shades)| *num_shades)
        .sum();
    assert_eq!(collective_len, 256);

    IMPORTANT_COLORS
        .iter()
        .flat_map(|([r, g, b], num_shades)| make_palette_range(*r, *g, *b, *num_shades))
}

/// Build a mini-palette consisting of a color range from a very dark version of the color to that color.
#[inline]
fn make_palette_range(r: f32, g: f32, b: f32, n: usize) -> impl Iterator<Item = [f32; 3]> {
    (0..n).map(move |x| {
        let (n, x) = (n as f32, x as f32);
        let factor = (n - x) / n;
        [r * factor, g * factor, b * factor]
    })
}

#[inline]
fn saturate_byte(b: f32) -> u8 {
    if b >= 1.0 {
        255
    } else if b <= 0.0 {
        0
    } else {
        (b * 255.0) as u8
    }
}
