// Apache 2.0 License

use super::*;
use std::{array::IntoIter as ArrayIter, path::Path};

macro_rules! instrument {
    ($path: expr, off1=$off1: expr, $basedir: expr) => {{
        Instrument::load(&($basedir).join($path), None, $off1, 0, None)
    }};
    ($path: expr, note=$octave: expr, $basedir: expr) => {{
        Instrument::load(&($basedir).join($path), None, 0, 0, Some($octave))
    }};
    ($path: expr, $basedir: expr) => {{
        Instrument::load(&($basedir).join($path), None, 0, 0, None)
    }};
    ($path: expr, $path2: expr, $basedir: expr) => {{
        Instrument::load(
            &($basedir).join($path),
            Some(&($basedir).join($path2)),
            0,
            0,
            None,
        )
    }};
    ($basedir: expr) => {{
        instrument!("dummy.sbi", $basedir)
    }};
}

#[inline]
pub fn iter_instruments(basedir: &Path) -> impl Iterator<Item = crate::Result<Instrument>> {
    ArrayIter::new([
        instrument!("instr001.sbi", basedir), // //001 - Acoustic Grand Piano
        instrument!("instr002.sbi", basedir), // //002 - Bright Acoustic Piano
        instrument!("instr003.sbi", basedir), // //003 - Electric Grand Piano
        instrument!("instr004.sbi", "instr004-2.sbi", basedir), // //004 - Honky-tonk Piano
        instrument!("instr005.sbi", "instr005-2.sbi", basedir), // //005 - Electric Piano 1
        instrument!("instr006.sbi", off1 = -12, basedir), // //006 - Electric Piano 2
        instrument!("instr007.sbi", basedir), // //007 - Harpsichord
        instrument!("instr008.sbi", off1 = -12, basedir), // //008 - Clavi
        instrument!("instr009.sbi", off1 = -12, basedir), // //009 - Celesta
        instrument!("instr010.sbi", off1 = -12, basedir), // //010 - Glockenspiel
        instrument!("instr011.sbi", off1 = -12, basedir), // //011 - Music Box
        instrument!("instr012.sbi", off1 = -12, basedir), // //012 - Vibraphone
        instrument!("instr013.sbi", off1 = -12, basedir), // //013 - Marimba
        instrument!("instr014.sbi", off1 = -12, basedir), // //014 - Xylophone
        instrument!("instr015.sbi", off1 = -12, basedir), // //015 - Tubular Bells
        instrument!("instr016.sbi", off1 = -12, basedir), // //016 - Dulcimer
        instrument!("instr017.sbi", off1 = -12, basedir), // //017 - Drawbar Organ
        instrument!("instr018.sbi", off1 = -12, basedir), // //018 - Percussive Organ
        instrument!("instr019.sbi", off1 = -12, basedir), // //019 - Rock Organ
        instrument!("instr020.sbi", off1 = -12, basedir), // //020 - Church Organ
        instrument!("instr021.sbi", off1 = -12, basedir), // //021 - Reed Organ
        instrument!("instr022.sbi", off1 = -12, basedir), // //022 - Accordion
        instrument!("instr023.sbi", off1 = -12, basedir), // //023 - Harmonica
        instrument!("instr024.sbi", off1 = -12, basedir), // //024 - Tango Accordion
        instrument!("instr025.sbi", off1 = -12, basedir), // //025 - Acoustic Guitar (nylon)
        instrument!("instr026.sbi", off1 = -12, basedir), // //026 - Acoustic Guitar (steel)
        instrument!("instr027.sbi", off1 = -12, basedir), // //027 - Electric Guitar (jazz)
        instrument!("instr028.sbi", off1 = -12, basedir), // //028 - Electric Guitar (clean)
        instrument!("instr029.sbi", off1 = -12, basedir), // //029 - Electric Guitar (muted)
        instrument!("instr030.sbi", off1 = -12, basedir), // //030 - Overdriven Guitar
        instrument!("instr031.sbi", off1 = -12, basedir), // //031 - Distortion Guitar
        instrument!("instr032.sbi", off1 = -12, basedir), // //032 - Guitar harmonics
        instrument!("instr033.sbi", off1 = -12, basedir), // //033 - Acoustic Bass
        instrument!("instr034.sbi", off1 = -12, basedir), // //034 - Electric Bass (finger)
        instrument!("instr035.sbi", off1 = -12, basedir), // //035 - Electric Bass (pick)
        instrument!("instr036.sbi", off1 = -12, basedir), // //036 - Fretless Bass
        instrument!("instr037.sbi", off1 = -12, basedir), // //037 - Slap Bass 1
        instrument!("instr038.sbi", off1 = -12, basedir), // //038 - Slap Bass 2
        instrument!("instr039.sbi", off1 = -12, basedir), // //039 - Synth Bass 1
        instrument!("instr040.sbi", off1 = -12, basedir), // //040 - Synth Bass 2
        instrument!("instr041.sbi", off1 = -12, basedir), // //041 - Violin
        instrument!("instr042.sbi", off1 = -12, basedir), // //042 - Viola
        instrument!("instr043.sbi", off1 = -12, basedir), // //043 - Cello
        instrument!("instr044.sbi", off1 = -12, basedir), // //044 - Contrabass
        instrument!("instr045.sbi", off1 = -12, basedir), // //045 - Tremolo Strings
        instrument!("instr046.sbi", off1 = -12, basedir), // //046 - Pizzicato Strings
        instrument!("instr047.sbi", off1 = -12, basedir), // //047 - Orchestral Harp
        instrument!("instr048.sbi", off1 = -12, basedir), // //048 - Timpani
        instrument!("instr049.sbi", off1 = -12, basedir), // //049 - String Ensemble 1
        instrument!("instr050.sbi", off1 = -12, basedir), // //050 - String Ensemble 2
        instrument!("instr051.sbi", off1 = -12, basedir), // //051 - SynthStrings 1
        instrument!("instr052.sbi", off1 = -12, basedir), // //052 - SynthStrings 2
        instrument!("instr053.sbi", off1 = -12, basedir), // //053 - Choir Aahs
        instrument!("instr054.sbi", off1 = -12, basedir), // //054 - Voice Oohs
        instrument!("instr055.sbi", off1 = -12, basedir), // //055 - Synth Voice
        instrument!("instr056.sbi", off1 = -12, basedir), // //056 - Orchestra Hit
        instrument!("instr057.sbi", off1 = -12, basedir), // //057 - Trumpet
        instrument!("instr058.sbi", off1 = -12, basedir), // //058 - Trombone
        instrument!("instr059.sbi", off1 = -12, basedir), // //059 - Tuba
        instrument!("instr060.sbi", off1 = -12, basedir), // //060 - Muted Trumpet
        instrument!("instr061.sbi", off1 = -12, basedir), // //061 - French Horn
        instrument!("instr062.sbi", off1 = -12, basedir), // //062 - Brass Section
        instrument!("instr063.sbi", off1 = -12, basedir), // //063 - SynthBrass 1
        instrument!("instr064.sbi", off1 = -12, basedir), // //064 - SynthBrass 2
        instrument!("instr065.sbi", off1 = -12, basedir), // //065 - Soprano Sax
        instrument!("instr066.sbi", off1 = -12, basedir), // //066 - Alto Sax
        instrument!("instr067.sbi", off1 = -12, basedir), // //067 - Tenor Sax
        instrument!("instr068.sbi", off1 = -12, basedir), // //068 - Baritone Sax
        instrument!("instr069.sbi", off1 = -12, basedir), // //069 - Oboe
        instrument!("instr070.sbi", off1 = -12, basedir), // //070 - English Horn
        instrument!("instr071.sbi", off1 = -12, basedir), // //071 - Bassoon
        instrument!("instr072.sbi", off1 = -12, basedir), // //072 - Clarinet
        instrument!("instr073.sbi", off1 = -12, basedir), // //073 - Piccolo
        instrument!("instr074.sbi", off1 = -12, basedir), // //074 - Flute
        instrument!("instr075.sbi", off1 = -12, basedir), // //075 - Recorder
        instrument!("instr076.sbi", off1 = -12, basedir), // //076 - Pan Flute
        instrument!("instr077.sbi", off1 = -12, basedir), // //077 - Blown Bottle
        instrument!("instr078.sbi", off1 = -12, basedir), // //078 - Shakuhachi
        instrument!("instr079.sbi", off1 = -12, basedir), // //079 - Whistle
        instrument!("instr080.sbi", off1 = -12, basedir), // //080 - Ocarina
        instrument!("instr081.sbi", off1 = -12, basedir), // //081 - Lead 1 (square)
        instrument!("instr082.sbi", basedir), // //082 - Lead 2 (sawtooth)
        instrument!("instr083.sbi", off1 = -12, basedir), // //083 - Lead 3 (calliope)
        instrument!("instr084.sbi", off1 = -12, basedir), // //084 - Lead 4 (chiff)
        instrument!("instr085.sbi", off1 = -12, basedir), // //085 - Lead 5 (charang)
        instrument!("instr086.sbi", off1 = -12, basedir), // //086 - Lead 6 (voice)
        instrument!("instr087.sbi", off1 = -12, basedir), // //087 - Lead 7 (fifths)
        instrument!("instr088.sbi", off1 = -12, basedir), // //088 - Lead 8 (bass + lead)
        instrument!("instr089.sbi", off1 = -12, basedir), // //089 - Pad 1 (new age)
        instrument!("instr090.sbi", off1 = -12, basedir), // //090 - Pad 2 (warm)
        instrument!("instr091.sbi", off1 = -12, basedir), // //091 - Pad 3 (polysynth)
        instrument!("instr092.sbi", off1 = -12, basedir), // //092 - Pad 4 (choir)
        instrument!("instr093.sbi", off1 = -12, basedir), // //093 - Pad 5 (bowed)
        instrument!("instr094.sbi", off1 = -12, basedir), // //094 - Pad 6 (metallic)
        instrument!("instr095.sbi", off1 = -12, basedir), // //095 - Pad 7 (halo)
        instrument!("instr096.sbi", off1 = -12, basedir), // //096 - Pad 8 (sweep)
        instrument!("instr097.sbi", off1 = -12, basedir), // //097 - FX 1 (rain)
        instrument!("instr098.sbi", off1 = -12, basedir), // //098 - FX 2 (soundtrack)
        instrument!("instr099.sbi", off1 = -12, basedir), // //099 - FX 3 (crystal)
        instrument!("instr100.sbi", off1 = -12, basedir), // //100 - FX 4 (atmosphere)
        instrument!("instr101.sbi", off1 = -12, basedir), // //101 - FX 5 (brightness)
        instrument!("instr102.sbi", off1 = -12, basedir), // //102 - FX 6 (goblins)
        instrument!("instr103.sbi", off1 = -12, basedir), // //103 - FX 7 (echoes)
        instrument!("instr104.sbi", off1 = -12, basedir), // //104 - FX 8 (sci-fi)
        instrument!("instr105.sbi", off1 = -12, basedir), // //105 - Sitar
        instrument!("instr106.sbi", off1 = -12, basedir), // //106 - Banjo
        instrument!("instr107.sbi", off1 = -12, basedir), // //107 - Shamisen
        instrument!("instr108.sbi", off1 = -12, basedir), // //108 - Koto
        instrument!("instr109.sbi", off1 = -12, basedir), // //109 - Kalimba
        instrument!("instr110.sbi", off1 = -12, basedir), // //110 - Bag pipe
        instrument!("instr111.sbi", off1 = -12, basedir), // //111 - Fiddle
        instrument!("instr112.sbi", off1 = -12, basedir), // //112 - Shanai
        instrument!("instr113.sbi", off1 = -12, basedir), // //113 - Tinkle Bell
        instrument!("instr114.sbi", off1 = -12, basedir), // //114 - Agogo
        instrument!("instr115.sbi", off1 = -12, basedir), // //115 - Steel Drums
        instrument!("instr116.sbi", off1 = -12, basedir), // //116 - Woodblock
        instrument!("instr117.sbi", off1 = -12, basedir), // //117 - Taiko Drum
        instrument!("instr118.sbi", off1 = -12, basedir), // //118 - Melodic Tom
        instrument!("instr119.sbi", off1 = -12, basedir), // //119 - Synth Drum
        instrument!("instr120.sbi", off1 = -12, basedir), // //120 - Reverse Cymbal
        instrument!("instr121.sbi", off1 = -12, basedir), // //121 - Guitar Fret Noise
        instrument!("instr122.sbi", off1 = -12, basedir), // //122 - Breath Noise
        instrument!("instr123.sbi", off1 = -12, basedir), // //123 - Seashore
        instrument!("instr124.sbi", off1 = -12, basedir), // //124 - Bird Tweet
        instrument!("instr125.sbi", off1 = -12, basedir), // //125 - Telephone Ring
        instrument!("instr126.sbi", off1 = -12, basedir), // //126 - Helicopter
        instrument!("instr127.sbi", off1 = -12, basedir), // //127 - Applause
        instrument!("instr128.sbi", off1 = -12, basedir), // //128 - Gunshot
    ])
}

#[inline]
pub fn iter_percussion(basedir: &Path) -> impl Iterator<Item = crate::Result<Instrument>> {
    ArrayIter::new([
        instrument!("perc35.sbi", note = ON4.a(), basedir), // //35 Acoustic Bass Drum
        instrument!("perc36.sbi", note = ON4.a(), basedir), // //36 Bass Drum 1
        instrument!("perc37.sbi", note = ON1.c(), basedir), // //37 Side Stick
        instrument!("perc38.sbi", note = ON3.gs(), basedir), // //38 Acoustic Snare
        instrument!("perc39.sbi", note = O3.c(), basedir),  // //39 Hand Clap
        instrument!("perc40.sbi", note = ON1.cs(), basedir), // //40 Electric Snare
        instrument!("perc41.sbi", note = ON3.d(), basedir), // //41 Low Floor Tom
        instrument!("perc42.sbi", note = O1.gs(), basedir), // //42 Closed Hi Hat
        instrument!("perc43.sbi", note = ON3.gs(), basedir), // //43 High Floor Tom
        instrument!("perc44.sbi", note = O1.gs(), basedir), // //44 Pedal Hi-Hat
        instrument!("perc45.sbi", note = ON2.c(), basedir), // //45 Low Tom
        instrument!("perc46.sbi", note = O1.gs(), basedir), // //46 Open Hi-Hat
        instrument!("perc47.sbi", note = ON2.fs(), basedir), // //47 Low-Mid Tom
        instrument!("perc48.sbi", note = ON2.a(), basedir), // //48 Hi-Mid Tom
        instrument!("perc49.sbi", note = ON1.c(), basedir), // //49 Crash Cymbal 1
        instrument!("perc50.sbi", note = ON1.cs(), basedir), // //50 High Tom
        instrument!("perc51.sbi", note = ON1.b(), basedir), // //51 Ride Cymbal 1
        instrument!("perc52.sbi", note = ON1.c(), basedir), // //52 Chinese Cymbal
        instrument!("perc53.sbi", note = O1.e(), basedir),  // //53 Ride Bell
        instrument!("perc54.sbi", note = O0.e(), basedir),  // //54 Tambourine
        instrument!(basedir),                               // TODO - //55 Splash Cymbal
        instrument!(basedir),                               // TODO - //56 Cowbell
        instrument!("perc57.sbi", note = ON1.r#as(), basedir), // //57 Crash Cymbal 2
        instrument!(basedir),                               // TODO - //58 Vibraslap
        instrument!("perc59.sbi", note = O0.e(), basedir),  // //59 Ride Cymbal 2
        instrument!(basedir),                               // TODO - //60 Hi Bongo
        instrument!(basedir),                               // TODO - //61 Low Bongo
        instrument!(basedir),                               // TODO - //62 Mute Hi Conga
        instrument!(basedir),                               // TODO - //63 Open Hi Conga
        instrument!(basedir),                               // TODO - //64 Low Conga
        instrument!(basedir),                               // TODO - //65 High Timbale
        instrument!(basedir),                               // TODO - //66 Low Timbale
        instrument!(basedir),                               // TODO - //67 High Agogo
        instrument!(basedir),                               // TODO - //68 Low Agogo
        instrument!(basedir),                               // TODO - //69 Cabasa
        instrument!("perc70.sbi", note = ON5.e(), basedir), // //70 Maracas
        instrument!("perc71.sbi", note = ON5.e(), basedir), // //71 Short Whistle
        instrument!("perc72.sbi", note = ON5.e(), basedir), // //72 Long Whistle
        instrument!("perc73.sbi", note = ON5.e(), basedir), // //73 Short Guiro
        instrument!("perc74.sbi", note = ON5.e(), basedir), // //74 Long Guiro
        instrument!("perc75.sbi", note = ON5.e(), basedir), // //75 Claves
        instrument!("perc76.sbi", note = ON5.e(), basedir), // //76 Hi Wood Block
        instrument!("perc77.sbi", note = ON5.e(), basedir), // //77 Low Wood Block
        instrument!("perc78.sbi", note = ON5.e(), basedir), // //78 Mute Cuica
        instrument!("perc79.sbi", note = ON5.e(), basedir), // //79 Open Cuica
        instrument!("perc80.sbi", note = ON5.e(), basedir), // //80 Mute Triangle
        instrument!("perc81.sbi", note = ON5.e(), basedir), // //81 Open Triangle
    ])
}
