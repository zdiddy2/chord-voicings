use std::rc::Rc;

// use std::{
//     fs::File,
//     io::{BufReader, Error},
// };
use druid::{im::Vector, Data, Env, EventCtx, Lens};

use self::chord_derived_lenses::scale_degree;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    bars: Vector<Bar>,
    modes: Vector<Rc<Vector<i8>>>,
    voicings: Vector<Rc<Voicing>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            bars: Vector::new(),
            modes: Vector::new(),
            voicings: default_voicings(),
        }
    }
    pub fn click_add_chord(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.bars
            .push_back(Bar::new(data.voicings.get(0).unwrap().clone(), 0, 1))
    }
}

fn default_voicings() -> Vector<Rc<Voicing>> {
    let voicings = vec![
        Rc::new(Voicing {
            name: "Root".to_string(),
            voicing: Vector::from(vec![1, 3, 5]),
        }),
        Rc::new(Voicing {
            name: "1st Inversion".to_string(),
            voicing: Vector::from(vec![3, 5, 8]),
        }),
        Rc::new(Voicing {
            name: "2nd Inversion".to_string(),
            voicing: Vector::from(vec![5, 8, 11]),
        }),
        Rc::new(Voicing {
            name: "3rd Inversion".to_string(),
            voicing: Vector::from(vec![8, 11, 13]),
        }),
    ];

    Vector::from(voicings)
}

#[derive(Clone, Data, Lens)]
pub struct Bar {
    chord: Chord,
}

impl Bar {
    pub fn default(voicing: Rc<Voicing>) -> Self {
        Self {
            chord: Chord::new(voicing, 0, 0),
        }
    }

    pub fn new(voicing: Rc<Voicing>, key: u8, degree: u8) -> Self {
        Self {
            chord: Chord::new(voicing, key, degree),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Chord {
    scale_degree: u8,
    voicing: Rc<Voicing>,
    key: Key,
}

impl Chord {
    pub fn new(voicing: Rc<Voicing>, key: u8, degree: u8) -> Self {
        Self {
            key: Key::new(key),
            scale_degree: degree,
            voicing: voicing,
        }
    }

    pub fn get_name(&self) -> String {
        let note = self.key.mode.scale.get(self.scale_degree.into()).unwrap() + self.key.key;

        match note {
            0 => "C",
            1 => "D♭",
            2 => "D",
            3 => "E♭",
            4 => "E",
            5 => "F",
            6 => "G♭",
            7 => "G",
            8 => "A♭",
            9 => "A",
            10 => "B♭",
            _ => "B",
        }
        .to_string()
    }
}

#[derive(Clone, Data, Lens)]
pub struct Key {
    pub key: u8, // 1-12
    pub mode: Rc<Mode>,
}

impl Key {
    pub fn new(key: u8) -> Self {
        Self {
            mode: Rc::new(Mode::ionian()),
            key,
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Mode {
    name: String,
    scale: Vector<u8>, //0-11 notes in scale
}

impl Mode {
    pub fn ionian() -> Self {
        Self {
            name: "Ionian".to_string(),
            scale: Vector::from(vec![0, 2, 4, 5, 7, 9, 11]),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Voicing {
    name: String,
    voicing: Vector<u64>,
}

impl Voicing {
    pub fn new() -> Self {
        Self {
            name: "Default".to_string(),
            voicing: Vector::new(),
        }
    }
}
