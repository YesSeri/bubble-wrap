// bars: 24
// total subbeats: 384

use crate::wasm4::*;
const VOICE_0_VOL: u32 = 60;
const VOICE_1_VOL: u32 = 60;
const VOICE_2_VOL: u32 = 100;
const VOICE_3_VOL: u32 = 100;
const VOICE_4_VOL: u32 = 100;

const VOICE_0_DUR: u32 = 13;
const VOICE_1_DUR: u32 = 13;
const VOICE_2_DUR: u32 = 7;
const VOICE_3_DUR: u32 = 30;
const VOICE_4_DUR: u32 = 30;

use crate::wasm4::*;
static mut LIMITER: usize = 0;
static mut COUNTER: usize = 100;

pub fn update_music() {
    unsafe {
        LIMITER += 1;
        if LIMITER < 4 {
            return;
        }
        LIMITER = 0;
    }
    let c = unsafe {
        if COUNTER == 384 {
            COUNTER = 0;
        }
        COUNTER
    };

    let (bar, subbeat) = unsafe {
        let bar: u16 = (c / 16).try_into().unwrap_unchecked();
        let subbeat: u16 = (c % 16).try_into().unwrap_unchecked();
        (bar, subbeat)
    };
    match bar {
        0 => {
            play_bar_0_voice_0(subbeat);
            play_bar_0_voice_1(subbeat);
            play_bar_0_voice_2(subbeat);
        }
        1 => {
            play_bar_1_voice_0(subbeat);
            play_bar_1_voice_1(subbeat);
            play_bar_1_voice_2(subbeat);
        }
        2 => {
            play_bar_2_voice_0(subbeat);
            play_bar_2_voice_1(subbeat);
            play_bar_2_voice_2(subbeat);
        }
        3 => {
            play_bar_3_voice_0(subbeat);
            play_bar_3_voice_1(subbeat);
            play_bar_3_voice_2(subbeat);
        }
        4 => {
            play_bar_4_voice_0(subbeat);
            play_bar_4_voice_1(subbeat);
            play_bar_4_voice_2(subbeat);
        }
        5 => {
            play_bar_5_voice_0(subbeat);
            play_bar_5_voice_1(subbeat);
            play_bar_5_voice_2(subbeat);
        }
        6 => {
            play_bar_6_voice_0(subbeat);
            play_bar_6_voice_1(subbeat);
            play_bar_6_voice_2(subbeat);
        }
        7 => {
            play_bar_7_voice_0(subbeat);
            play_bar_7_voice_1(subbeat);
            play_bar_7_voice_2(subbeat);
        }
        8 => {
            play_bar_8_voice_0(subbeat);
            play_bar_8_voice_1(subbeat);
            play_bar_8_voice_2(subbeat);
        }
        9 => {
            play_bar_9_voice_0(subbeat);
            play_bar_9_voice_1(subbeat);
            play_bar_9_voice_2(subbeat);
        }
        10 => {
            play_bar_10_voice_0(subbeat);
            play_bar_10_voice_1(subbeat);
            play_bar_10_voice_2(subbeat);
        }
        11 => {
            play_bar_11_voice_0(subbeat);
            play_bar_11_voice_1(subbeat);
            play_bar_11_voice_2(subbeat);
        }
        12 => {
            play_bar_12_voice_0(subbeat);
            play_bar_12_voice_1(subbeat);
            play_bar_12_voice_2(subbeat);
        }
        13 => {
            play_bar_13_voice_0(subbeat);
            play_bar_13_voice_1(subbeat);
            play_bar_13_voice_2(subbeat);
        }
        14 => {
            play_bar_14_voice_0(subbeat);
            play_bar_14_voice_1(subbeat);
            play_bar_14_voice_2(subbeat);
        }
        15 => {
            play_bar_15_voice_0(subbeat);
            play_bar_15_voice_1(subbeat);
            play_bar_15_voice_2(subbeat);
        }
        16 => {
            play_bar_16_voice_0(subbeat);
            play_bar_16_voice_1(subbeat);
            play_bar_16_voice_2(subbeat);
        }
        17 => {
            play_bar_17_voice_0(subbeat);
            play_bar_17_voice_1(subbeat);
            play_bar_17_voice_2(subbeat);
        }
        18 => {
            play_bar_18_voice_0(subbeat);
            play_bar_18_voice_1(subbeat);
            play_bar_18_voice_2(subbeat);
        }
        19 => {
            play_bar_19_voice_0(subbeat);
            play_bar_19_voice_1(subbeat);
            play_bar_19_voice_2(subbeat);
        }
        20 => {
            play_bar_20_voice_0(subbeat);
            play_bar_20_voice_1(subbeat);
            play_bar_20_voice_2(subbeat);
        }
        21 => {
            play_bar_21_voice_0(subbeat);
            play_bar_21_voice_1(subbeat);
            play_bar_21_voice_2(subbeat);
        }
        22 => {
            play_bar_22_voice_0(subbeat);
            play_bar_22_voice_1(subbeat);
            play_bar_22_voice_2(subbeat);
        }
        23 => {
            play_bar_23_voice_0(subbeat);
            play_bar_23_voice_1(subbeat);
            play_bar_23_voice_2(subbeat);
        }

        _ => (),
    }
    unsafe {
        COUNTER += 1;
    }
}

fn play_bar_0_voice_0(subbeat: u16) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_1_voice_0(subbeat: u16) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_2_voice_0(subbeat: u16) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_3_voice_0(subbeat: u16) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_4_voice_0(subbeat: u16) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_5_voice_0(subbeat: u16) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_6_voice_0(subbeat: u16) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_7_voice_0(subbeat: u16) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_8_voice_0(subbeat: u16) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_9_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(262, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        1 => {
            tone(262, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        2 => {
            tone(262, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        3 => {
            tone(262, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        4 => {
            tone(277, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        5 => {
            tone(277, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        6 => {
            tone(277, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        7 => {
            tone(277, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        8 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_10_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        2 => {
            tone(988, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // B
        }
        6 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        8 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_11_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        2 => {
            tone(988, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // B
        }
        6 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        8 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_12_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        4 => {
            tone(988, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // B
        }
        6 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        8 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        12 => {
            tone(415, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Ab
        }
        14 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        _ => {}
    }
}

fn play_bar_13_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(554, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        2 => {
            tone(988, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // B
        }
        6 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        8 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_14_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(262, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        1 => {
            tone(262, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        2 => {
            tone(262, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        3 => {
            tone(262, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        4 => {
            tone(277, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        5 => {
            tone(277, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        6 => {
            tone(277, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        7 => {
            tone(277, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        8 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_15_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        2 => {
            tone(554, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        6 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        8 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_16_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        2 => {
            tone(554, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        6 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        8 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_17_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        4 => {
            tone(554, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        6 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        8 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        12 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        14 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        _ => {}
    }
}

fn play_bar_18_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(622, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Eb
        }
        2 => {
            tone(554, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        6 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        8 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_19_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        4 => {}
        8 => {}
        _ => {}
    }
}

fn play_bar_20_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(466, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Bb
        }
        4 => {}
        8 => {}
        _ => {}
    }
}

fn play_bar_21_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        4 => {}
        8 => {}
        _ => {}
    }
}

fn play_bar_22_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        4 => {
            tone(524, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        8 => {
            tone(262, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // C
        }
        10 => {
            tone(277, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Db
        }
        14 => {
            tone(311, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // Eb
        }
        _ => {}
    }
}

fn play_bar_23_voice_0(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // F
        }
        4 => {}
        8 => {
            tone(349, VOICE_0_DUR, VOICE_0_VOL, TONE_PULSE1) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_0_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_1_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_2_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        4 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {
            tone(311, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Eb
        }
        14 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        _ => {}
    }
}

fn play_bar_3_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(415, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Ab
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_4_voice_1(subbeat: u16) {
    match subbeat {
        0 => {}
        4 => {}
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_5_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_6_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_7_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        4 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {
            tone(311, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Eb
        }
        14 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        _ => {}
    }
}

fn play_bar_8_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(415, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Ab
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_9_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        1 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        2 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        3 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        4 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        5 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        7 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_10_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_11_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_12_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        4 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {
            tone(311, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Eb
        }
        14 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        _ => {}
    }
}

fn play_bar_13_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(415, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Ab
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_14_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        1 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        2 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        3 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        4 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        5 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        7 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_15_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_16_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_17_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        4 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {
            tone(311, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Eb
        }
        14 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        _ => {}
    }
}

fn play_bar_18_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(415, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Ab
        }
        2 => {
            tone(370, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Gb
        }
        6 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        8 => {
            tone(349, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // F
        }
        12 => {}
        _ => {}
    }
}

fn play_bar_19_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(440, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // A
        }
        4 => {}
        8 => {}
        _ => {}
    }
}

fn play_bar_20_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(494, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // B
        }
        4 => {}
        8 => {}
        _ => {}
    }
}

fn play_bar_21_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(494, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // B
        }
        4 => {}
        8 => {}
        _ => {}
    }
}

fn play_bar_22_voice_1(subbeat: u16) {
    match subbeat {
        0 => {
            tone(494, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // B
        }
        4 => {
            tone(415, VOICE_1_DUR, VOICE_1_VOL, TONE_PULSE2) // Ab
        }
        8 => {}
        _ => {}
    }
}

fn play_bar_23_voice_1(subbeat: u16) {
    match subbeat {
        0 => {}
        _ => {}
    }
}

fn play_bar_0_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {}
        _ => {}
    }
}

fn play_bar_1_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {}
        _ => {}
    }
}

fn play_bar_2_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {}
        _ => {}
    }
}

fn play_bar_3_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(330, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        8 => {
            tone(349, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        _ => {}
    }
}

fn play_bar_4_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(207, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Ab
        }
        11 => {
            tone(220, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // A
        }
        12 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(207, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Ab
        }
        15 => {
            tone(220, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // A
        }
        _ => {}
    }
}

fn play_bar_5_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(330, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        11 => {
            tone(349, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        12 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        15 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        _ => {}
    }
}

fn play_bar_6_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(330, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        11 => {
            tone(349, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        12 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        15 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        _ => {}
    }
}

fn play_bar_7_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(207, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Ab
        }
        11 => {
            tone(220, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // A
        }
        12 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        15 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        _ => {}
    }
}

fn play_bar_8_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(233, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        9 => {
            tone(220, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // A
        }
        10 => {
            tone(207, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Ab
        }
        11 => {
            tone(196, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // G
        }
        12 => {
            tone(185, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Gb
        }
        13 => {
            tone(174, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        14 => {
            tone(165, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        15 => {
            tone(155, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Eb
        }
        _ => {}
    }
}

fn play_bar_9_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(165, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        11 => {
            tone(174, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        12 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        15 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        _ => {}
    }
}

fn play_bar_10_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(165, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        11 => {
            tone(174, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        12 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        15 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        _ => {}
    }
}

fn play_bar_11_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(165, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        11 => {
            tone(174, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        12 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        15 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        _ => {}
    }
}

fn play_bar_12_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(103, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Ab
        }
        11 => {
            tone(110, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // A
        }
        12 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        15 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        _ => {}
    }
}

fn play_bar_13_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(165, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        11 => {
            tone(174, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        12 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(165, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        15 => {
            tone(174, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        _ => {}
    }
}

fn play_bar_14_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(165, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        11 => {
            tone(174, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        12 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        15 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        _ => {}
    }
}

fn play_bar_15_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(165, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        11 => {
            tone(174, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        12 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        15 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        _ => {}
    }
}

fn play_bar_16_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(165, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        11 => {
            tone(174, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        12 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(165, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        15 => {
            tone(174, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        _ => {}
    }
}

fn play_bar_17_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        10 => {
            tone(103, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Ab
        }
        11 => {
            tone(110, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // A
        }
        12 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        14 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        15 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        _ => {}
    }
}

fn play_bar_18_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        2 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        3 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        4 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        6 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        7 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        8 => {
            tone(116, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Bb
        }
        9 => {
            tone(110, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // A
        }
        10 => {
            tone(103, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Ab
        }
        11 => {
            tone(98, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // G
        }
        12 => {
            tone(92, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Gb
        }
        13 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        14 => {
            tone(82, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // E
        }
        15 => {
            tone(77, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Eb
        }
        _ => {}
    }
}

fn play_bar_19_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        4 => {}
        8 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        10 => {
            tone(92, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Gb
        }
        14 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        _ => {}
    }
}

fn play_bar_20_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        4 => {}
        8 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        10 => {
            tone(92, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Gb
        }
        14 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        _ => {}
    }
}

fn play_bar_21_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        4 => {}
        8 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        12 => {
            tone(92, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Gb
        }
        14 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        _ => {}
    }
}

fn play_bar_22_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        4 => {
            tone(77, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Eb
        }
        6 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        8 => {
            tone(103, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Ab
        }
        10 => {
            tone(92, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // Gb
        }
        14 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        _ => {}
    }
}

fn play_bar_23_voice_2(subbeat: u16) {
    match subbeat {
        0 => {
            tone(87, VOICE_2_DUR, VOICE_2_VOL, TONE_TRIANGLE) // F
        }
        4 => {}
        8 => {}
        _ => {}
    }
}
