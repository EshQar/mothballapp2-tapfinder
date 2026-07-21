use crate::args::handle_keyword_arg;
use crate::math;
use crate::expr_eval;
use crate::parser;
use crate::errors;
use crate::parser::{Tokenized, DataType};
use crate::functions::{Argument, ArgumentValue, FunctionData, Function, FullArgumentValue};

use indexmap::IndexMap;
use std::collections::{HashMap, VecDeque};

use std::f32;
use std::f64;

use std::ops::Deref;

// Equivalent to Python subclass of str.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MothballSequence(pub String);

impl Deref for MothballSequence {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressionType {
    ZLabel = 0,
    ZLabelWithExpression = 1,
    XLabel = 2,
    XLabelWithExpression = 3,
    GeneralLabel = 4,
    GeneralLabelWithNumber = 5,
    GeneralLabelWithExpression = 6,
    Warning = 7,
    Text = 8,
    ZInertiaHit = 9,
    ZInertiaMiss = 10,
    XInertiaHit = 11,
    XInertiaMiss = 12,
}

enum OutputExpression {
    ZLabel(String, &'static str, String),
    ZLabelWithExpression(String, &'static str, String),
    XLabel(String, &'static str, String),
    XLabelWithExpression(String, &'static str, String),
    GeneralLabel(String),
    GeneralLabelWithNumber(String, &'static str, String),
    GeneralLabelWithExpression(String, &'static str, String, &'static str, String),
    Warning(&'static str, &'static str, String),
    Text(String),
    ZInertiaMiss(String, String, &'static str, String, String),
    ZInertiaHit(String, String, &'static str, String, String),
    XInertiaHit(String, String, &'static str, String, String),
    XInertiaMiss(String, String, &'static str, String, String),
    GeneralInertiaLabel(String, &'static str, String, &'static str, String, &'static str)
}

enum InertiaRecordParam {
    Axis(&'static str),
    Tick(u32),
    Tolerance(f64),
}

impl InertiaRecordParam {
    fn unwrap_f64(&self) -> f64 {
        match self {
            InertiaRecordParam::Axis(_) => {
                panic!();
            }

            InertiaRecordParam::Tick(_) => {
                panic!();
            }

            InertiaRecordParam::Tolerance(float) => {
                return *float
            }
        }
    }

    fn unwrap_tick_as_mut(&mut self) -> &mut u32 {
        match self {
            InertiaRecordParam::Axis(_) => {
                panic!();
            }

            InertiaRecordParam::Tick(tick) => {
                return tick
            }

            InertiaRecordParam::Tolerance(_) => {
                panic!();
            }
        }
    }

    fn unwrap_tick(&self) -> u32 {
        match self {
            InertiaRecordParam::Axis(_) => {
                panic!();
            }

            InertiaRecordParam::Tick(tick) => {
                return *tick
            }

            InertiaRecordParam::Tolerance(_) => {
                panic!();
            }
        }
    }

    fn unwrap_axis(&self) -> &'static str {
        match self {
            InertiaRecordParam::Axis(string) => {
                return string
            }

            InertiaRecordParam::Tick(_) => {
                panic!();
            }

            InertiaRecordParam::Tolerance(_) => {
                panic!();
            }
        }
    }
}

enum PossibilitiesRecordParam {
    Axis(&'static str),
    Tick(u32),
    MinDist(f64),
    Xoffset(f64),
    Xincrement(f64),
    Zoffset(f64),
    Zincrement(f64),
    Miss(Option<f64>),
}

impl PossibilitiesRecordParam {
    fn unwrap_f64(&self) -> f64 {
        match self {
            PossibilitiesRecordParam::Axis(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Tick(_) => {
                panic!();
            }

            PossibilitiesRecordParam::MinDist(float) => {
                return *float
            }

            PossibilitiesRecordParam::Xoffset(float) => {
                return *float
            }

            PossibilitiesRecordParam::Xincrement(float) => {
                return *float
            }

            PossibilitiesRecordParam::Zoffset(float) => {
                return *float
            }

            PossibilitiesRecordParam::Zincrement(float) => {
                return *float
            }

            PossibilitiesRecordParam::Miss(maybe_float) => {
                match maybe_float {
                    Some(float) => { return *float }
                    None => panic!()
                }
            }
        }
    }

    fn unwrap_tick_as_mut(&mut self) -> &mut u32 {
        match self {
            PossibilitiesRecordParam::Axis(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Tick(tick) => {
                return tick
            }

            PossibilitiesRecordParam::MinDist(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Xoffset(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Xincrement(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Zoffset(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Zincrement(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Miss(_) => {
                panic!();
            }
        }
    }

    fn unwrap_tick(&self) -> u32 {
        match self {
            PossibilitiesRecordParam::Axis(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Tick(tick) => {
                return *tick
            }

            PossibilitiesRecordParam::MinDist(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Xoffset(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Xincrement(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Zoffset(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Zincrement(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Miss(_) => {
                panic!();
            }
        }
    }

    fn unwrap_axis(&self) -> &'static str {
        match self {
            PossibilitiesRecordParam::Axis(string) => {
                return string
            }

            PossibilitiesRecordParam::Tick(_) => {
                panic!();
            }

            PossibilitiesRecordParam::MinDist(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Xoffset(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Xincrement(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Zoffset(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Zincrement(_) => {
                panic!();
            }

            PossibilitiesRecordParam::Miss(_) => {
                panic!();
            }
        }
    }
}

#[derive(Clone)]
enum State {
    Ground,
    Air,
    Jump,
}


pub struct Tick {
//    pub w: bool,
//    pub a: bool,
//    pub s: bool,
//    pub d: bool,
//    pub sneak: bool,
//    pub sprint: bool,
//    pub space: bool,
//    pub right_click: bool,
//    pub last_turn: f32,
//
//    pub x: Option<f32>,
//    pub z: Option<f32>,
//    pub vx: Option<f32>,
//    pub vz: Option<f32>,
}

impl Tick {
    pub fn new(
        w: bool,
        a: bool,
        s: bool,
        d: bool,
        sneak: bool,
        sprint: bool,
        space: bool,
        right_click: bool,
        last_turn: f32,
        x: Option<f32>,
        z: Option<f32>,
        vx: Option<f32>,
        vz: Option<f32>,
    ) -> Self {
        Self {
//            w,
//            a,
//            s,
//            d,
//            sneak,
//            sprint,
//            space,
//            right_click,
//            last_turn,
//            x,
//            z,
//            vx,
//            vz,
        }
    }
}

pub fn copy_player(player: &PlayerSimulationXZ) -> PlayerSimulationXZ {
    let dummy_player = PlayerSimulationXZ::new();

    PlayerSimulationXZ {
        angle_queue: player.angle_queue.clone(),
        turn_queue: player.turn_queue.clone(),
        rotation: player.rotation,
        state: player.state.clone(),
        default_ground_slip: player.default_ground_slip,
        previous_slip: player.previous_slip,
        air_sprint_delay: player.air_sprint_delay,
        sneak_delay: player.sneak_delay,
        previously_sneaking: player.previously_sneaking,
        speed_effect: player.speed_effect,
        slow_effect: player.slow_effect,
        local_vars: player.local_vars.clone(),
        ..dummy_player
    }
}

pub struct Simulation;

impl Simulation {
    pub const PI: f64 = 3.14159265358979323846;

    pub const FORTYFIVE_METHODS: &'static [&'static str] = &[
        "walk45",
        "walkair45",
        "walkjump45",
        "sprint45",
        "sprintair45",
        "sprintjump45",
        "sneak45",
        "sneakair45",
        "sneakjump45",
        "sneaksprint45",
        "sneaksprintair45",
        "sneaksprintjump45",
        "walkpessi45",
        "sprintpessi45",
        "forcemomentum45",
    ];

    pub const CAN_HAVE_INPUT: &'static [&'static str] = &[
        "walk",
        "walkair",
        "walkjump",
        "sprint",
        "sprintair",
        "sprintjump",
        "sneak",
        "sneakair",
        "sneakjump",
        "sneaksprint",
        "sneaksprintair",
        "sneaksprintjump",
        "walkpessi",
        "sprintpessi",
        "forcemomentum",
    ];

    pub const CAN_HAVE_MODIFIERS: &'static [&'static str] = &[
        "walk45",
        "walkair45",
        "walkjump45",
        "sprint45",
        "sprintair45",
        "sprintjump45",
        "sneak45",
        "sneakair45",
        "sneakjump45",
        "sneaksprint45",
        "sneaksprintair45",
        "sneaksprintjump45",
        "walkpessi45",
        "sprintpessi45",
        "forcemomentum45",
        "walk",
        "walkair",
        "walkjump",
        "sprint",
        "sprintair",
        "sprintjump",
        "sneak",
        "sneakair",
        "sneakjump",
        "sneaksprint",
        "sneaksprintair",
        "sneaksprintjump",
        "walkpessi",
        "sprintpessi",
        "forcemomentum",
        "stop",
        "stopjump",
        "stopair",
        "sneakstop",
        "sneakstopair",
        "sneakstopjump",
    ];

    pub fn mm_to_dist(mm: f32) -> f32 {
        mm + mm.signum() * 0.6
    }

    pub fn dist_to_mm(dist: f32) -> f32 {
        dist - dist.signum() * 0.6
    }

    pub fn dist_to_block(mm: f32) -> f32 {
        Self::mm_to_dist(mm)
    }

    pub fn block_to_dist(dist: f32) -> f32 {
        Self::dist_to_mm(dist)
    }

    pub const OLD_SPRINTJUMP_BOOST: f32 = 0.2;
    pub const NEW_SPRINTJUMP_BOOST: f64 = 0.2;

    pub const JUMP: i32 = 0;
    pub const GROUND: i32 = 1;
    pub const AIR: i32 = 2;

    pub const OLD_COMPUTATION: i32 = 0;
    pub const NEW_COMPUTATION: i32 = 1;

    pub const WATER: u32 = 0b1;
    pub const LAVA: u32 = 0b10;
    pub const WEB: u32 = 0b100;
    pub const BLOCK: u32 = 0b1000;
    pub const LADDER: u32 = 0b10000;
    pub const SOULSAND: u32 = 0b100000;

    pub const MODIFIERS: &'static [u32] = &[
        Self::WATER,
        Self::WEB,
        Self::LAVA,
        Self::BLOCK,
        Self::LADDER,
        Self::SOULSAND,
    ];

    pub fn alias_to_modifier() -> HashMap<&'static str, u32> {
        HashMap::from([
            ("water", Self::WATER),
            ("wt", Self::WATER),
            ("lv", Self::LAVA),
            ("lava", Self::LAVA),
            ("web", Self::WEB),
            ("block", Self::BLOCK),
            ("bl", Self::BLOCK),
            ("ladder", Self::LADDER),
            ("ld", Self::LADDER),
            ("vine", Self::LADDER),
            ("soulsand", Self::SOULSAND),
            ("ss", Self::SOULSAND),
        ])
    }

    pub fn functions_by_type() -> HashMap<&'static str, Vec<&'static str>> {
        HashMap::from([
            (
                "fast-movers",
                vec![
                    "sprint","s","sprint45","s45","sprintjump","sprintjump45",
                    "sj","sj45","sprintair","sa","sprintair45","sa45",
                    "sprintstrafejump","sprintstrafejump45","strafejump",
                    "strafejump45","stfj","stfj45","sneaksprint",
                    "sneaksprintair","sneaksprintjump","sns","snsa","snsj",
                    "sneaksprint45","sneaksprintair45","sneaksprintjump45",
                    "sns45","snsa45","snsj45","sprintpessi","sp",
                    "sprintpessi45","sp45","forcemomentum","fmm",
                    "forcemomentum45","fmm45",
                ],
            ),
            (
                "slow-movers",
                vec![
                    "walk","w","walkair","wa","walkjump","wj","walk45",
                    "w45","walkair45","wa45","walkjump45","wj45",
                    "sneak","sneak45","sn","sn45","sneakair",
                    "sneakair45","sna","sna45","sneakjump","snj",
                    "sneakjump45","snj45","walkpessi","walkpessi45",
                    "wp","wp45",
                ],
            ),
            (
                "stoppers",
                vec![
                    "stop","stopground","st","stopair","sta","stopjump",
                    "stj","sneakstop","sneakstopair","sneakstopjump",
                    "snst","snsta","snstj",
                ],
            ),
            (
                "returners",
                vec![
                    "outz","zmm","zb","outvz","outx","xmm","xb",
                    "outvx","vec","help","print","effectsmultiplier",
                    "effects","dimensions","dim","outangle","outa",
                    "outfacing","outf","outturn","outt","macro",
                    "angleinfo","ai",
                ],
            ),
            (
                "calculators",
                vec![
                    "bwmm","xbwmm","wall","xwall","inv","xinv",
                    "blocks","xblocks","repeat","r","possibilities",
                    "poss","xpossibilities","xposs","xzpossibilities",
                    "xzposs","taps",
                ],
            ),
            (
                "setters",
                vec![
                    "face","facing","f","turn","setposz","z","setvz","vz",
                    "setposx","x","setvx","vx","setslip","slip",
                    "setprecision","precision","pre","inertia",
                    "sprintairdelay","sdel","version","v","anglequeue",
                    "aq","tq","turnqueue","speed","slow","slowness",
                    "sndel","sneakdelay","var","function","func",
                    "alias","toggle","singleaxisinertia",
                    "inertialistener","il","xinertialistener","xil",
                    "zinertialistener","zil","xzinertialistener",
                    "xzil","addposx","addposz","addz","addx",
                    "addvx","addvz",
                ],
            ),
        ])
    }
}

pub struct PlayerSimulationXZ {
    pub precision: i32,
    pub inertia_threshold: f32,
    pub modifiers: i32,
    pub reverse: bool,

    pub previously_sprinting: bool,
    pub previously_sneaking: bool,
    pub previously_in_web: bool,

    pub local_vars: IndexMap<String, parser::Data>,

    output: Vec<(ExpressionType, OutputExpression)>,

    pub call_stack: Vec<String>,

    pub stop_flag: bool,

    pub last_returned_value: f64,

    pub x: f64,
    pub z: f64,
    pub vx: f64,
    pub vz: f64,

    pub default_ground_slip: f32,
    pub current_slip: f32,
    pub previous_slip: Option<f32>,

    pub total_angles: i32,

    pub rotation: f32,
    pub last_rotation: f32,
    pub last_turn: f32,

    pub angle_queue: VecDeque<f32>,
    pub turn_queue: VecDeque<f32>,

    pub air_sprint_delay: bool,
    pub sneak_delay: bool,
    pub inertia_axis: i32,

    pub inputs: String,

    pub version_computation: i32,
    state: State,

    record: HashMap<String, PossibilitiesRecordParam>,
    record_inertia: HashMap<String, InertiaRecordParam>,

    pub speed_effect: i32,
    pub slow_effect: i32,

    pub history: Vec<Tick>,

    macros: HashMap<String, String>,
    pub functions: HashMap<i32, Function>,
    pub alias_to_id_map: HashMap<String, i32>,
}

impl PlayerSimulationXZ {
    pub fn maybe_get_function(&self, alias: &str) -> Option<&Function> {
        let id = self.alias_to_id_map.get(alias);
        match id {
            Some(id) => self.functions.get(id),
            None => None
        }
    }

    pub fn get_function(&self, alias: &str) -> &Function {
        &self.functions[&self.alias_to_id_map[alias]]
    }

    pub fn has_function(&self, alias: &str) -> bool {
        self.functions.contains_key(&self.alias_to_id_map[alias])
    }
}

impl PlayerSimulationXZ {
    fn mccos(&self, facing: f32) -> f32 {
        math::mccos(self.total_angles, Simulation::PI as f32, facing)
    }

    fn mcsin(&self, facing: f32) -> f32 {
        math::mcsin(self.total_angles, Simulation::PI as f32, facing)
    }

    fn truncate_number(&self, value:f64) -> String {
        math::truncate_number(self.precision, value)
    }
}

impl PlayerSimulationXZ {
    pub fn optimize_xz<F>(
        &self,
        x: f64,
        z: f64,
        sequence: &str,
        conversion: F,
    ) -> (f64, f64)
    where
        F: Fn(f64) -> f64,
    {
        let mut p1 = copy_player(self);
        p1.inertia_threshold = 0.0;
        p1.simulate(sequence.to_string(), true, None, true);

        let mut p2 = copy_player(self);
        p2.inertia_threshold = 0.0;
        p2.vz = 1.0;
        p2.vx = 1.0;
        p2.simulate(sequence.to_string(), true, None, true);

        if true {
            if p1.x == p2.x {
                panic!(
                    "Float division by 0, perhaps you reset your position at the end of a sequence or nested same axis optimize functions?"
                );
            }

            let vx = (p1.x - conversion(x)) / (p1.x - p2.x);

            if true {
                if p1.z == p2.z {
                    panic!(
                        "Float division by 0, perhaps you reset your position at the end of a sequence or nested same axis optimize functions?"
                    );
                }

                let vz = (p1.z - conversion(z)) / (p1.z - p2.z);

                (vx, vz) // return (vx, vz)
            } else {
                panic!() // return vx
            }
        } else if z != 0.0 {
            if p1.z == p2.z {
                panic!(
                    "Float division by 0, perhaps you reset your position at the end of a sequence or nested same axis optimize functions?"
                );
            }

            let vz = (p1.z - conversion(z)) / (p1.z - p2.z);

            panic!() // return vz
        } else {
            panic!()
        }
    }

    pub fn optimize_x<F>(
        &self,
        x: f64,
        z: f64,
        sequence: &str,
        conversion: F,
    ) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let mut p1 = copy_player(self);
        p1.inertia_threshold = 0.0;
        p1.simulate(sequence.to_string(), true, None, true);

        let mut p2 = copy_player(self);
        p2.inertia_threshold = 0.0;
        p2.vz = 1.0;
        p2.vx = 1.0;
        p2.simulate(sequence.to_string(), true, None, true);

        if true {
            if p1.x == p2.x {
                panic!(
                    "Float division by 0, perhaps you reset your position at the end of a sequence or nested same axis optimize functions?"
                );
            }

            let vx = (p1.x - conversion(x) as f64) / (p1.x - p2.x);

            if false {
                if p1.z == p2.z {
                    panic!(
                        "Float division by 0, perhaps you reset your position at the end of a sequence or nested same axis optimize functions?"
                    );
                }

                let vz = (p1.z - conversion(z)) / (p1.z - p2.z);

                panic!() // return (vx, vz)
            } else {
                vx // return vx
            }
        } else if false {
            if p1.z == p2.z {
                panic!(
                    "Float division by 0, perhaps you reset your position at the end of a sequence or nested same axis optimize functions?"
                );
            }

            let vz = (p1.z - conversion(z)) / (p1.z - p2.z);

            panic!() // return vz
        } else {
            panic!()
        }
    }

    pub fn optimize_z<F>(
        &self,
        x: f64,
        z: f64,
        sequence: &str,
        conversion: F,
    ) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let mut p1 = copy_player(self);
        p1.inertia_threshold = 0.0;
        p1.simulate(sequence.to_string(), true, None, true);

        let mut p2 = copy_player(self);
        p2.inertia_threshold = 0.0;
        p2.vz = 1.0;
        p2.vx = 1.0;
        p2.simulate(sequence.to_string(), true, None, true);

        if false {
            if p1.x == p2.x {
                panic!(
                    "Float division by 0, perhaps you reset your position at the end of a sequence or nested same axis optimize functions?"
                );
            }

            let vx = (p1.x - conversion(x)) / (p1.x - p2.x);

            if z != 0.0 {
                if p1.z == p2.z {
                    panic!(
                        "Float division by 0, perhaps you reset your position at the end of a sequence or nested same axis optimize functions?"
                    );
                }

                let vz = (p1.z - conversion(z)) / (p1.z - p2.z);

                panic!() // return (vx, vz)
            } else {
                panic!() // return vx
            }
        } else if true {
            if p1.z == p2.z {
                panic!(
                    "Float division by 0, perhaps you reset your position at the end of a sequence or nested same axis optimize functions?"
                );
            }

            let vz = (p1.z - conversion(z)) / (p1.z - p2.z);

            vz // return vz
        } else {
            panic!()
        }
    }
}

impl PlayerSimulationXZ {
    pub fn new() -> Self {
        let mut local_vars = IndexMap::new();
        local_vars.insert("px".to_string(), parser::Data::Float(0.0625));
        let mut funcs: Vec<(i32, Function)> = Vec::new();
        // Do not ask 😭😭😭
		funcs.push((1, Function::OutZ(FunctionData::new(1, "outz".to_string(), vec!["outz".to_string()], vec![Argument::PositionalOnly("centered_about".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0f64)), false), Argument::PositionalOrKeyword("label".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("outz".to_string())), false)]))));
		funcs.push((2, Function::Zmm(FunctionData::new(2, "zmm".to_string(), vec!["zmm".to_string()], vec![Argument::PositionalOnly("centered_about".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0f64)), false), Argument::PositionalOrKeyword("label".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("zmm".to_string())), false)]))));
		funcs.push((3, Function::Bwmm(FunctionData::new(3, "bwmm".to_string(), vec!["bwmm".to_string()], vec![Argument::PositionalOnly("zmm".to_string(), ArgumentValue::Empty(DataType::Float), true), Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true)]))));
		funcs.push((4, Function::Zb(FunctionData::new(4, "zb".to_string(), vec!["zb".to_string()], vec![Argument::PositionalOnly("centered_about".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0f64)), false), Argument::PositionalOrKeyword("label".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("zb".to_string())), false)]))));
		funcs.push((5, Function::OutVz(FunctionData::new(5, "outvz".to_string(), vec!["outvz".to_string()], vec![Argument::PositionalOnly("centered_about".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0f64)), false), Argument::PositionalOrKeyword("label".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("vz".to_string())), false)]))));
		funcs.push((6, Function::Wall(FunctionData::new(6, "wall".to_string(), vec!["wall".to_string(), "inv".to_string()], vec![Argument::PositionalOnly("z".to_string(), ArgumentValue::Empty(DataType::Float), true), Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true)]))));
		funcs.push((7, Function::OutX(FunctionData::new(7, "outx".to_string(), vec!["outx".to_string()], vec![Argument::PositionalOnly("centered_about".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0f64)), false), Argument::PositionalOrKeyword("label".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("outx".to_string())), false)]))));
		funcs.push((8, Function::Xmm(FunctionData::new(8, "xmm".to_string(), vec!["xmm".to_string()], vec![Argument::PositionalOnly("centered_about".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0f64)), false), Argument::PositionalOrKeyword("label".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("xmm".to_string())), false)]))));
		funcs.push((9, Function::Blocks(FunctionData::new(9, "blocks".to_string(), vec!["blocks".to_string()], vec![Argument::PositionalOnly("zb".to_string(), ArgumentValue::Empty(DataType::Float), true), Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true)]))));
		funcs.push((10, Function::Xb(FunctionData::new(10, "xb".to_string(), vec!["xb".to_string()], vec![Argument::PositionalOnly("centered_about".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0f64)), false), Argument::PositionalOrKeyword("label".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("xb".to_string())), false)]))));
		funcs.push((11, Function::OutVx(FunctionData::new(11, "outvx".to_string(), vec!["outvx".to_string()], vec![Argument::PositionalOnly("centered_about".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0f64)), false), Argument::PositionalOrKeyword("label".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("vx".to_string())), false)]))));
		funcs.push((12, Function::XBwmm(FunctionData::new(12, "xbwmm".to_string(), vec!["xbwmm".to_string()], vec![Argument::PositionalOnly("xmm".to_string(), ArgumentValue::Empty(DataType::Float), true), Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true)]))));
		funcs.push((13, Function::Vec(FunctionData::new(13, "vec".to_string(), vec!["vec".to_string()], vec![]))));
		funcs.push((14, Function::OutAngle(FunctionData::new(14, "outfacing".to_string(), vec!["outfacing".to_string(), "outangle".to_string(), "outa".to_string(), "outf".to_string()], vec![Argument::PositionalOnly("centered_about".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0f64)), false), Argument::PositionalOrKeyword("label".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("facing".to_string())), false)]))));
		funcs.push((15, Function::XWall(FunctionData::new(15, "xwall".to_string(), vec!["xwall".to_string(), "xinv".to_string()], vec![Argument::PositionalOnly("x".to_string(), ArgumentValue::Empty(DataType::Float), true), Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true)]))));
		funcs.push((16, Function::OutTurn(FunctionData::new(16, "outturn".to_string(), vec!["outturn".to_string(), "outt".to_string()], vec![Argument::PositionalOnly("centered_about".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0f64)), false), Argument::PositionalOrKeyword("label".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("turn".to_string())), false)]))));
		funcs.push((17, Function::EffectsMultiplier(FunctionData::new(17, "effectsmultiplier".to_string(), vec!["effectsmultiplier".to_string(), "effects".to_string()], vec![Argument::PositionalOrKeyword("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::PositionalOrKeyword("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((18, Function::Walk(FunctionData::new(18, "walk".to_string(), vec!["walk".to_string(), "w".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((19, Function::XBlocks(FunctionData::new(19, "xblocks".to_string(), vec!["xblocks".to_string()], vec![Argument::PositionalOnly("xb".to_string(), ArgumentValue::Empty(DataType::Float), true), Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true)]))));
		funcs.push((20, Function::AngleInfo(FunctionData::new(20, "angleinfo".to_string(), vec!["angleinfo".to_string(), "ai".to_string()], vec![Argument::PositionalOnly("angle".to_string(), ArgumentValue::HasValue(DataType::F32, FullArgumentValue::F32(0.0f32)), false)]))));
		funcs.push((21, Function::Walk45(FunctionData::new(21, "walk45".to_string(), vec!["walk45".to_string(), "w45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((22, Function::Face(FunctionData::new(22, "facing".to_string(), vec!["facing".to_string(), "face".to_string(), "f".to_string()], vec![Argument::PositionalOnly("angle_in_degrees".to_string(), ArgumentValue::Empty(DataType::F32), true)]))));
		funcs.push((23, Function::Sprint(FunctionData::new(23, "sprint".to_string(), vec!["sprint".to_string(), "s".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((24, Function::Turn(FunctionData::new(24, "turn".to_string(), vec!["turn".to_string()], vec![Argument::PositionalOnly("angle_in_degrees".to_string(), ArgumentValue::Empty(DataType::F32), true)]))));
		funcs.push((25, Function::Sprint45(FunctionData::new(25, "sprint45".to_string(), vec!["sprint45".to_string(), "s45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((26, Function::Macro(FunctionData::new(26, "macro".to_string(), vec!["macro".to_string()], vec![Argument::PositionalOnly("name".to_string(), ArgumentValue::Empty(DataType::Str), true), Argument::PositionalOnly("formatting".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("mpk".to_string())), false)]))));
		funcs.push((27, Function::SetPosZ(FunctionData::new(27, "setposz".to_string(), vec!["setposz".to_string(), "z".to_string()], vec![Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::Float), true)]))));
		funcs.push((28, Function::WalkAir(FunctionData::new(28, "walkair".to_string(), vec!["walkair".to_string(), "wa".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((29, Function::SetVz(FunctionData::new(29, "setvz".to_string(), vec!["setvz".to_string(), "vz".to_string()], vec![Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::Float), true)]))));
		funcs.push((30, Function::WalkAir45(FunctionData::new(30, "walkair45".to_string(), vec!["walkair45".to_string(), "wa45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((31, Function::SetPosX(FunctionData::new(31, "setposx".to_string(), vec!["setposx".to_string(), "x".to_string()], vec![Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::Float), true)]))));
		funcs.push((32, Function::SprintAir(FunctionData::new(32, "sprintair".to_string(), vec!["sprintair".to_string(), "sa".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((33, Function::SetVx(FunctionData::new(33, "setvx".to_string(), vec!["setvx".to_string(), "vx".to_string()], vec![Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::Float), true)]))));
		funcs.push((34, Function::SprintAir45(FunctionData::new(34, "sprintair45".to_string(), vec!["sprintair45".to_string(), "sa45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((35, Function::AddPosZ(FunctionData::new(35, "addposz".to_string(), vec!["addposz".to_string(), "addz".to_string()], vec![Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::Float), true)]))));
		funcs.push((36, Function::WalkJump(FunctionData::new(36, "walkjump".to_string(), vec!["walkjump".to_string(), "wj".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((37, Function::AddPosX(FunctionData::new(37, "addposx".to_string(), vec!["addposx".to_string(), "addx".to_string()], vec![Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::Float), true)]))));
		funcs.push((38, Function::WalkJump45(FunctionData::new(38, "walkjump45".to_string(), vec!["walkjump45".to_string(), "wj45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((39, Function::AddVx(FunctionData::new(39, "addvx".to_string(), vec!["addvx".to_string()], vec![Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::Float), true)]))));
		funcs.push((40, Function::WalkPessi(FunctionData::new(40, "walkpessi".to_string(), vec!["walkpessi".to_string(), "wp".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("delay".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((41, Function::AddVz(FunctionData::new(41, "addvz".to_string(), vec!["addvz".to_string()], vec![Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::Float), true)]))));
		funcs.push((42, Function::WalkPessi45(FunctionData::new(42, "walkpessi45".to_string(), vec!["walkpessi45".to_string(), "wp45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("delay".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((43, Function::SetSlip(FunctionData::new(43, "setslip".to_string(), vec!["setslip".to_string(), "slip".to_string()], vec![Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::F32), true)]))));
		funcs.push((44, Function::SprintJump(FunctionData::new(44, "sprintjump".to_string(), vec!["sprintjump".to_string(), "sj".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((45, Function::Inertia(FunctionData::new(45, "inertia".to_string(), vec!["inertia".to_string()], vec![Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::F32), true), Argument::PositionalOrKeyword("single_axis".to_string(), ArgumentValue::Empty(DataType::Bool), false)]))));
		funcs.push((46, Function::SprintJump45(FunctionData::new(46, "sprintjump45".to_string(), vec!["sprintjump45".to_string(), "sj45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((47, Function::SprintAirDelay(FunctionData::new(47, "sprintairdelay".to_string(), vec!["sprintairdelay".to_string(), "sdel".to_string()], vec![Argument::PositionalOnly("toggle".to_string(), ArgumentValue::Empty(DataType::Bool), true)]))));
		funcs.push((48, Function::SprintStrafeJump(FunctionData::new(48, "sprintstrafejump".to_string(), vec!["sprintstrafejump".to_string(), "strafejump".to_string(), "stfj".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((49, Function::SneakDelay(FunctionData::new(49, "sneakdelay".to_string(), vec!["sneakdelay".to_string(), "sndel".to_string()], vec![Argument::PositionalOnly("toggle".to_string(), ArgumentValue::Empty(DataType::Bool), true)]))));
		funcs.push((50, Function::SprintStrafeJump45(FunctionData::new(50, "sprintstrafejump45".to_string(), vec!["sprintstrafejump45".to_string(), "strafejump45".to_string(), "stfj45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((51, Function::SingleAxisInertia(FunctionData::new(51, "singleaxisinertia".to_string(), vec!["singleaxisinertia".to_string(), "sai".to_string()], vec![Argument::PositionalOnly("toggle".to_string(), ArgumentValue::Empty(DataType::Bool), true)]))));
		funcs.push((52, Function::SprintPessi(FunctionData::new(52, "sprintpessi".to_string(), vec!["sprintpessi".to_string(), "sp".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("delay".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((53, Function::Version(FunctionData::new(53, "version".to_string(), vec!["version".to_string(), "v".to_string()], vec![Argument::PositionalOnly("string".to_string(), ArgumentValue::Empty(DataType::Str), true)]))));
		funcs.push((54, Function::SprintPessi45(FunctionData::new(54, "sprintpessi45".to_string(), vec!["sprintpessi45".to_string(), "sp45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("delay".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((55, Function::Repeat(FunctionData::new(55, "repeat".to_string(), vec!["repeat".to_string(), "r".to_string()], vec![Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true), Argument::PositionalOnly("count".to_string(), ArgumentValue::Empty(DataType::Int), true)]))));
		funcs.push((56, Function::Speed(FunctionData::new(56, "speed".to_string(), vec!["speed".to_string()], vec![Argument::PositionalOnly("multiplier".to_string(), ArgumentValue::Empty(DataType::Int), true)]))));
		funcs.push((57, Function::ForceMomentum(FunctionData::new(57, "forcemomentum".to_string(), vec!["forcemomentum".to_string(), "fmm".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("delay".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((58, Function::Slowness(FunctionData::new(58, "slowness".to_string(), vec!["slowness".to_string(), "slow".to_string()], vec![Argument::PositionalOnly("multiplier".to_string(), ArgumentValue::Empty(DataType::Int), true)]))));
		funcs.push((59, Function::ForceMomentum45(FunctionData::new(59, "forcemomentum45".to_string(), vec!["forcemomentum45".to_string(), "fmm45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("delay".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((60, Function::Print(FunctionData::new(60, "print".to_string(), vec!["print".to_string()], vec![Argument::PositionalOnly("string".to_string(), ArgumentValue::HasValue(DataType::Str, FullArgumentValue::Str("".to_string())), false)]))));
		funcs.push((61, Function::AngleQueue(FunctionData::new(61, "anglequeue".to_string(), vec!["anglequeue".to_string(), "aq".to_string()], vec![Argument::VarPositional("angles".to_string(), ArgumentValue::Empty(DataType::F32), true)]))));
		funcs.push((62, Function::Sneak(FunctionData::new(62, "sneak".to_string(), vec!["sneak".to_string(), "sn".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((63, Function::TurnQueue(FunctionData::new(63, "turnqueue".to_string(), vec!["turnqueue".to_string(), "tq".to_string()], vec![Argument::VarPositional("angles".to_string(), ArgumentValue::Empty(DataType::Float), true)]))));
		funcs.push((64, Function::Sneak45(FunctionData::new(64, "sneak45".to_string(), vec!["sneak45".to_string(), "sn45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((65, Function::Var(FunctionData::new(65, "var".to_string(), vec!["var".to_string()], vec![Argument::PositionalOnly("variable_name".to_string(), ArgumentValue::Empty(DataType::Str), true), Argument::PositionalOnly("value".to_string(), ArgumentValue::Empty(DataType::Str), false)]))));
		funcs.push((66, Function::SneakAir(FunctionData::new(66, "sneakair".to_string(), vec!["sneakair".to_string(), "sna".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((67, Function::SetPrecision(FunctionData::new(67, "setprecision".to_string(), vec!["setprecision".to_string(), "precision".to_string(), "pre".to_string()], vec![Argument::PositionalOnly("decimal_places".to_string(), ArgumentValue::Empty(DataType::Int), true)]))));
		funcs.push((68, Function::Possibilities(FunctionData::new(68, "possibilities".to_string(), vec!["possibilities".to_string(), "poss".to_string()], vec![Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true), Argument::PositionalOnly("min_distance".to_string(), ArgumentValue::Empty(DataType::Float), true), Argument::PositionalOnly("offset".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.6f64)), false), Argument::KeywordOnly("increment".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0625f64)), false), Argument::KeywordOnly("miss".to_string(), ArgumentValue::Empty(DataType::Float), false)]))));
		funcs.push((69, Function::SneakAir45(FunctionData::new(69, "sneakair45".to_string(), vec!["sneakair45".to_string(), "sna45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((70, Function::SneakJump(FunctionData::new(70, "sneakjump".to_string(), vec!["sneakjump".to_string(), "snj".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((71, Function::BallHelp(FunctionData::new(71, "ballhelp".to_string(), vec!["ballhelp".to_string(), "help".to_string()], vec![Argument::PositionalOrKeyword("func".to_string(), ArgumentValue::Empty(DataType::Str), true)]))));
		funcs.push((72, Function::XPossibilities(FunctionData::new(72, "xpossibilities".to_string(), vec!["xpossibilities".to_string(), "xposs".to_string()], vec![Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true), Argument::PositionalOnly("min_distance".to_string(), ArgumentValue::Empty(DataType::Float), true), Argument::PositionalOnly("offset".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.6f64)), false), Argument::KeywordOnly("increment".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0625f64)), false), Argument::KeywordOnly("miss".to_string(), ArgumentValue::Empty(DataType::Float), false)]))));
		funcs.push((73, Function::SneakJump45(FunctionData::new(73, "sneakjump45".to_string(), vec!["sneakjump45".to_string(), "snj45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((74, Function::Stop(FunctionData::new(74, "stopground".to_string(), vec!["stopground".to_string(), "stop".to_string(), "st".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((75, Function::StopAir(FunctionData::new(75, "stopair".to_string(), vec!["stopair".to_string(), "sta".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false)]))));
		funcs.push((76, Function::XZPossibilities(FunctionData::new(76, "xzpossibilities".to_string(), vec!["xzpossibilities".to_string(), "xzposs".to_string()], vec![Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true), Argument::PositionalOnly("min_distance".to_string(), ArgumentValue::Empty(DataType::Float), true), Argument::PositionalOnly("x_offset".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.6f64)), false), Argument::PositionalOnly("z_offset".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.6f64)), false), Argument::KeywordOnly("x_increment".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0625f64)), false), Argument::KeywordOnly("z_increment".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.0625f64)), false), Argument::KeywordOnly("miss".to_string(), ArgumentValue::Empty(DataType::Float), false)]))));
		funcs.push((77, Function::StopJump(FunctionData::new(77, "stopjump".to_string(), vec!["stopjump".to_string(), "stj".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((78, Function::SneakStop(FunctionData::new(78, "sneakstop".to_string(), vec!["sneakstop".to_string(), "snst".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((79, Function::InertiaListener(FunctionData::new(79, "xzinertialistener".to_string(), vec!["xzinertialistener".to_string(), "inertialistener".to_string(), "xzil".to_string(), "il".to_string()], vec![Argument::PositionalOnly("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true), Argument::PositionalOrKeyword("tolerance".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.002f64)), false)]))));
		funcs.push((80, Function::SneakStopAir(FunctionData::new(80, "sneakstopair".to_string(), vec!["sneakstopair".to_string(), "snsta".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false)]))));
		funcs.push((81, Function::SneakStopJump(FunctionData::new(81, "sneakstopjump".to_string(), vec!["sneakstopjump".to_string(), "snstj".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((82, Function::XInertiaListener(FunctionData::new(82, "xinertialistener".to_string(), vec!["xinertialistener".to_string(), "xil".to_string()], vec![Argument::PositionalOrKeyword("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true), Argument::PositionalOrKeyword("tolerance".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.002f64)), false)]))));
		funcs.push((83, Function::SneakSprint(FunctionData::new(83, "sneaksprint".to_string(), vec!["sneaksprint".to_string(), "sns".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((84, Function::SneakSprint45(FunctionData::new(84, "sneaksprint45".to_string(), vec!["sneaksprint45".to_string(), "sns45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((85, Function::ZInertiaListener(FunctionData::new(85, "zinertialistener".to_string(), vec!["zinertialistener".to_string(), "zil".to_string()], vec![Argument::PositionalOrKeyword("sequence".to_string(), ArgumentValue::Empty(DataType::Str), true), Argument::PositionalOrKeyword("tolerance".to_string(), ArgumentValue::HasValue(DataType::Float, FullArgumentValue::Float(0.002f64)), false)]))));
		funcs.push((86, Function::SneakSprintAir(FunctionData::new(86, "sneaksprintair".to_string(), vec!["sneaksprintair".to_string(), "snsa".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((87, Function::Dimensions(FunctionData::new(87, "dimensions".to_string(), vec!["dimensions".to_string(), "dim".to_string()], vec![Argument::PositionalOnly("x".to_string(), ArgumentValue::Empty(DataType::Float), true), Argument::PositionalOnly("z".to_string(), ArgumentValue::Empty(DataType::Float), true)]))));
		funcs.push((88, Function::SneakSprintAir45(FunctionData::new(88, "sneaksprintair45".to_string(), vec!["sneaksprintair45".to_string(), "snsa45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false)]))));
		funcs.push((89, Function::SneakSprintJump(FunctionData::new(89, "sneaksprintjump".to_string(), vec!["sneaksprintjump".to_string(), "snsj".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((90, Function::SneakSprintJump45(FunctionData::new(90, "sneaksprintjump45".to_string(), vec!["sneaksprintjump45".to_string(), "snsj45".to_string()], vec![Argument::PositionalOnly("duration".to_string(), ArgumentValue::HasValue(DataType::Int, FullArgumentValue::Int(1)), false), Argument::PositionalOnly("rotation".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("slip".to_string(), ArgumentValue::Empty(DataType::F32), false), Argument::KeywordOnly("speed".to_string(), ArgumentValue::Empty(DataType::Int), false), Argument::KeywordOnly("slow".to_string(), ArgumentValue::Empty(DataType::Int), false)]))));
		funcs.push((91, Function::Taps(FunctionData::new(91, "taps".to_string(), vec!["taps".to_string()], vec![Argument::VarPositional("seq_or_num".to_string(), ArgumentValue::Empty(DataType::Str), true)]))));

        let mut alias_to_id_map: HashMap<String, i32> = HashMap::new();
        for (id, func) in funcs.iter() {
            for alias in func.aliases() {
                alias_to_id_map.insert(alias.clone(),*id);
            }
        }

        Self {
            precision: 7,
            inertia_threshold: 0.005,
            modifiers: 0,
            reverse: false,

            previously_sprinting: false,
            previously_sneaking: false,
            previously_in_web: false,

            local_vars,

            output: Vec::new(),
            call_stack: Vec::new(),

            stop_flag: false,

            last_returned_value: 0.0,

            x: 0.0,
            z: 0.0,
            vx: 0.0,
            vz: 0.0,

            default_ground_slip: 0.6,
            current_slip: 0.6,
            previous_slip: None,

            total_angles: 65536,

            rotation: 0.0,
            last_rotation: 0.0,
            last_turn: 0.0,

            angle_queue: VecDeque::new(),
            turn_queue: VecDeque::new(),

            air_sprint_delay: true,
            sneak_delay: false,
            inertia_axis: 1,

            inputs: String::new(),

            version_computation: Simulation::OLD_COMPUTATION,
            state: State::Ground,

            record: HashMap::new(),
            record_inertia: HashMap::new(),

            speed_effect: 0,
            slow_effect: 0,

            history: Vec::new(),

            macros: HashMap::new(),

            alias_to_id_map,
            functions: funcs.into_iter().collect()
        }
    }
}

impl PlayerSimulationXZ {
    pub fn get_angle(&mut self) -> f32 {
        // Returns the next angle from the rotation queue or, if empty,
        // returns the current/default rotation.

        if let Some(angle) = self.angle_queue.pop_front() {
            self.rotation = angle;
        }

        if let Some(turn) = self.turn_queue.pop_front() {
            self.rotation += turn;
        }

        self.rotation
    }

    #[allow(clippy::too_many_arguments)]
    /// state = State::Ground, is_sprinting = false, is_sneaking = false, rotation_offset = 0.0f32
    fn move_player(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        rotation_offset: f64,
        slip: Option<f32>,
        is_sprinting: bool,
        is_sneaking: bool,
        speed: Option<i32>,
        slow: Option<i32>,
        state: State,
    ) {
        if self.version_computation == Simulation::OLD_COMPUTATION {
            self.move_old(
                duration,
                rotation,
                rotation_offset,
                slip,
                is_sprinting,
                is_sneaking,
                speed,
                slow,
                state,
            );
        } else if self.version_computation == Simulation::NEW_COMPUTATION {
            self.move_new(
                duration,
                rotation,
                rotation_offset,
                slip,
                is_sprinting,
                is_sneaking,
                speed,
                slow,
                state,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn move_old(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        rotation_offset: f64,
        slip: Option<f32>,
        is_sprinting: bool,
        is_sneaking: bool,
        speed: Option<i32>,
        slow: Option<i32>,
        state: State,
    ) {
        let mut slip = slip;
        let mut rotation = rotation;

        // Setting slipperiness here and treating it like air is analytically and numerically equivalent
        if self.modifiers & Simulation::WATER as i32 != 0 {
            slip = Some((0.8f64 / 0.91f64) as f32);
        } else if self.modifiers & Simulation::LAVA as i32 != 0 {
            slip = Some((0.5f64 / 0.91f64) as f32);
        }

        let mut sj_boost = Simulation::OLD_SPRINTJUMP_BOOST;

        if self.previous_slip.is_none() {
            self.previous_slip = Some(self.default_ground_slip);
        }

        if rotation_offset == 45.0 {
            self.inputs = "wa".to_string();
        }

        let speed = speed.unwrap_or(self.speed_effect);
        let slow = slow.unwrap_or(self.slow_effect);

        self.state = state;

        // If sneaking is modified by ladders, always set the state to AIR
        if ((self.sneak_delay && self.previously_sneaking)
            || (!self.sneak_delay && is_sneaking))
            && (self.modifiers & Simulation::LAVA as i32 != 0)
        {
            self.state = State::Air;
        }

        let mut override_rotation = false;

        if let Some(rot) = rotation {
            override_rotation = true;
            rotation = Some(((rot as f64) + rotation_offset) as f32);
        }

        // If slip is not given, assume ground slip
        if slip.is_none() {
            slip = Some(self.default_ground_slip);
        }

        let slip = slip.unwrap();

        for _ in 0..duration {
            if !override_rotation {
                rotation = Some((self.get_angle() as f64 + rotation_offset) as f32);
            }

            let rotation = rotation.unwrap();

            // MOVING THE PLAYER
            self.x += self.vx;
            self.z += self.vz;

            if self.modifiers & Simulation::SOULSAND as i32 != 0 {
                self.vx *= 0.4;
                self.vz *= 0.4;
            }

            let (mut forward, mut strafe) = self.movement_values();

            if self.reverse {
                forward *= -1.0;
                strafe *= -1.0;
                sj_boost *= -1.0;
            }

            // Finalize Momentum
            self.vx *= (0.91f32 * self.previous_slip.unwrap()) as f64;
            self.vz *= (0.91f32 * self.previous_slip.unwrap()) as f64;

            // Apply inertia or web
            if self.inertia_axis == 1 {
                if self.vx.abs() < self.inertia_threshold as f64 || self.previously_in_web {
                    self.vx = 0.0;
                }

                if self.vz.abs() < self.inertia_threshold as f64 || self.previously_in_web {
                    self.vz = 0.0;
                }
            } else if self.inertia_axis == 2 {
                if (self.vz * self.vz + self.vx * self.vx).sqrt()
                    < self.inertia_threshold as f64
                    || self.previously_in_web
                {
                    self.vx = 0.0;
                    self.vz = 0.0;
                }
            }

            // Get Movement Multiplier M
            let m = self.movement_multiplier_old(
                slip,
                is_sprinting,
                speed,
                slow,
                self.state.clone(),
            );

            // Sprint jump boost
            if matches!(self.state, State::Jump) && is_sprinting {
                let facing = rotation * 0.017453292f32;

                self.vx -= (self.mcsin(facing) * sj_boost) as f64;
                self.vz += (self.mccos(facing) * sj_boost) as f64;
            }

            // BLOCKING
            if self.modifiers & Simulation::BLOCK as i32 != 0 {
                forward = (forward as f64 * 0.2f64) as f32;
                strafe = (strafe as f64 * 0.2f64) as f32;
            }

            // SNEAKING
            if (self.sneak_delay && self.previously_sneaking)
                || (!self.sneak_delay && is_sneaking)
            {
                forward = (forward as f64 * 0.3f64) as f32;
                strafe = (strafe as f64 * 0.3f64) as f32;
            }

            forward *= 0.98;
            strafe *= 0.98;

            let mut distance = strafe * strafe + forward * forward;

            if distance >= 0.0001 {
                distance = (distance as f64).sqrt() as f32;

                if distance < 1.0 {
                    distance = 1.0;
                }

                distance = m / distance;

                forward *= distance;
                strafe *= distance;

                let sin_yaw = self.mcsin(
                    rotation * (Simulation::PI as f32) / 180.0
                );

                let cos_yaw = self.mccos(
                    rotation * (Simulation::PI as f32) / 180.0
                );

                self.vx += (strafe * cos_yaw - forward * sin_yaw) as f64;
                self.vz += (forward * cos_yaw + strafe * sin_yaw) as f64;
            }

            if self.modifiers & Simulation::WEB as i32 != 0 {
                self.vx /= 4.0;
                self.vz /= 4.0;
            }

            if self.modifiers & Simulation::LADDER as i32 != 0 {
                self.vx = self.vx.clamp(-0.15, 0.15);
                self.vz = self.vz.clamp(-0.15, 0.15);
            }

            // Prep for next tick
            self.previous_slip = Some(slip);
            self.previously_sprinting = is_sprinting;
            self.previously_sneaking = is_sneaking;
            self.previously_in_web = (self.modifiers & Simulation::WEB as i32) != 0;

            self.last_turn = rotation - self.last_rotation;
            self.last_rotation = rotation;

            // Record possibilities and history
            self.possibilities_helper();

            self.inertialistener_helper();

            self.history.push(Tick::new(
                self.inputs.contains('w'),
                self.inputs.contains('a'),
                self.inputs.contains('s'),
                self.inputs.contains('d'),
                is_sneaking,
                is_sprinting,
                matches!(self.state, State::Jump),
                (self.modifiers & Simulation::BLOCK as i32) != 0,
                self.last_turn,
                Some(self.x as f32),
                Some(self.z as f32),
                Some(self.vx as f32),
                Some(self.vz as f32),
            ));
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn move_new(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        rotation_offset: f64,
        slip: Option<f32>,
        is_sprinting: bool,
        is_sneaking: bool,
        speed: Option<i32>,
        slow: Option<i32>,
        state: State,
    ) {
        let mut rotation = rotation;
        let mut slip = slip;

        // Setting slipperiness here and treating it like air is analytically and numerically equivalent
        if self.modifiers & Simulation::WATER as i32 != 0 {
            slip = Some((0.8f64 / 0.91f64) as f32);
        } else if self.modifiers & Simulation::LAVA as i32 != 0 {
            slip = Some((0.5f64 / 0.91f64) as f32);
        }

        let mut sj_boost = Simulation::NEW_SPRINTJUMP_BOOST;

        if self.previous_slip.is_none() {
            self.previous_slip = Some(self.default_ground_slip);
        }

        if rotation_offset == 45.0 {
            self.inputs = "wa".to_string();
        }

        let speed = speed.unwrap_or(self.speed_effect);
        let slow = slow.unwrap_or(self.slow_effect);

        self.state = state;

        // If sneaking is modified by ladders, always set the state to AIR
        if ((self.sneak_delay && self.previously_sneaking)
            || (!self.sneak_delay && is_sneaking))
            && (self.modifiers & Simulation::LAVA as i32 != 0)
        {
            self.state = State::Air;
        }

        let mut override_rotation = false;

        if let Some(rot) = rotation {
            override_rotation = true;
            rotation = Some((rot as f64 + rotation_offset) as f32);
        }

        // If slip is not given, assume its ground slip since air slip (0.1) is always passed into the argument
        if slip.is_none() {
            slip = Some(self.default_ground_slip);
        }

        let slip = slip.unwrap();

        for _ in 0..duration {
            if !override_rotation {
                rotation = Some((self.get_angle() as f64 + rotation_offset) as f32);
            }

            let rotation = rotation.unwrap();

            // MOVING THE PLAYER
            self.x += self.vx;
            self.z += self.vz;

            if self.modifiers & Simulation::SOULSAND as i32 != 0 {
                // Like 13 df accurate minimum (old computation)
                self.vx *= 0.4;
                self.vz *= 0.4;
            }

            let (mut forward, mut strafe) = self.movement_values();

            if self.reverse {
                forward *= -1.0;
                strafe *= -1.0;
                sj_boost *= -1.0;
            }

            // Finalize Momentum
            self.vx *= (0.91f32 * self.previous_slip.unwrap()) as f64;
            self.vz *= (0.91f32 * self.previous_slip.unwrap()) as f64;

            // Apply inertia or web
            if self.inertia_axis == 1 {
                if self.vx.abs() < self.inertia_threshold as f64 || self.previously_in_web {
                    self.vx = 0.0;
                }

                if self.vz.abs() < self.inertia_threshold as f64 || self.previously_in_web {
                    self.vz = 0.0;
                }
            } else if self.inertia_axis == 2 {
                if (self.vz * self.vz + self.vx * self.vx).sqrt()
                    < self.inertia_threshold as f64
                    || self.previously_in_web
                {
                    self.vx = 0.0;
                    self.vz = 0.0;
                }
            }

            // Get Movement Multiplier M
            let m = self.movement_multiplier_new(
                slip,
                is_sprinting,
                speed,
                slow,
                &self.state,
            );

            // Sprint jump boost
            if matches!(self.state, State::Jump) && is_sprinting {
                let facing = rotation * 0.017453292f32;

                self.vx -= self.mcsin(facing) as f64 * sj_boost;
                self.vz += self.mccos(facing) as f64 * sj_boost;
            }

            // BLOCKING
            if self.modifiers & Simulation::BLOCK as i32 != 0 {
                forward = (forward as f64 * 0.2f64) as f32;
                strafe = (strafe as f64 * 0.2f64) as f32;
            }

            // SNEAKING
            if (self.sneak_delay && self.previously_sneaking)
                || (!self.sneak_delay && is_sneaking)
            {
                forward = (forward as f64 * 0.3f64) as f32;
                strafe = (strafe as f64 * 0.3f64) as f32;
            }

            forward *= 0.98;
            strafe *= 0.98;

            // idk yet if this is the right place to put it relative to the other movement.
            let mut forward = forward as f64;
            let mut strafe = strafe as f64;

            let mut distance = strafe * strafe + forward * forward;

            if distance >= 1e-7 {
                // Normalize distance IF above 1
                let distancef32 = distance.sqrt() as f32;

                if distancef32 < 1.0f64 as f32 {
                    distance = 1.0f64 as f64;
                } else {
                    distance = distancef32 as f64 + 0.0000001125593117f64;
                }

                // Modifies strafe and forward to account for movement
                distance = m as f64 / distance;

                forward *= distance;
                strafe *= distance;

                // Adds rotated vectors to velocity
                let sin_yaw = self.mcsin(rotation * 0.017453292f64 as f32);
                let cos_yaw = self.mccos(rotation * 0.017453292f64 as f32);

                self.vx += strafe * cos_yaw as f64 - forward * sin_yaw as f64;
                self.vz += forward * cos_yaw as f64 + strafe * sin_yaw as f64;
            }

            // Not verified to be 1.14+ accurate yet
            if self.modifiers & Simulation::WEB as i32 != 0 {
                self.vx /= 4.0;
                self.vz /= 4.0;
            }

            if self.modifiers & Simulation::LADDER as i32 != 0 {
                self.vx = self.vx.clamp(-0.15, 0.15);
                self.vz = self.vz.clamp(-0.15, 0.15);
            }

            // Prep for next tick
            self.previous_slip = Some(slip);
            self.previously_sprinting = is_sprinting;
            self.previously_sneaking = is_sneaking;
            self.previously_in_web = (self.modifiers & Simulation::WEB as i32) != 0;

            self.last_turn = rotation - self.last_rotation;
            self.last_rotation = rotation;

            // Record possibilities and history
            self.possibilities_helper();
            self.inertialistener_helper();

            self.history.push(Tick::new(
                self.inputs.contains('w'),
                self.inputs.contains('a'),
                self.inputs.contains('s'),
                self.inputs.contains('d'),
                is_sneaking,
                is_sprinting,
                matches!(self.state, State::Jump),
                (self.modifiers & Simulation::BLOCK as i32) != 0,
                self.last_turn,
                Some(self.x as f32),
                Some(self.z as f32),
                Some(self.vx as f32),
                Some(self.vz as f32),
            ));
        }
    }

    pub fn get_inertia_speed(&self) -> f32 {
        // Get the speed of hitting inertia, depending on whether the player is
        // midair, on ground, and with what slipperiness.
        if matches!(self.state, State::Air) {
            self.inertia_threshold / 0.91f32
        } else {
            self.inertia_threshold / (0.91f32 * self.current_slip)
        }
    }

    /// Auxiliary function for dealing with `inertialistener()` functions.
    pub fn inertialistener_helper(&mut self) {
        if self.record_inertia.is_empty() {
            return;
        }

        let record_axis = self.record_inertia["type"].unwrap_axis();
        let inertia_speed = self.get_inertia_speed();
        let tolerance =
            self.record_inertia["tolerance"].unwrap_f64().abs() as f32 + inertia_speed;

        if record_axis == "x" || record_axis == "xz" {
            if self.vx.abs() <= tolerance as f64 {
                if self.vx.abs() <= inertia_speed as f64 {
                    self.add_to_output(
                        ExpressionType::XInertiaHit,
                        format!(
                            "Tick {} Vx (Hit)",
                            self.record_inertia["tick"].unwrap_tick()
                        ),
                        StringOrNum::Num(self.vx),
                        inertia_speed as f64,
                        true,
                    );
                } else {
                    self.add_to_output(
                        ExpressionType::XInertiaMiss,
                        format!(
                            "Tick {} Vx (Miss)",
                            self.record_inertia["tick"].unwrap_tick()
                        ),
                        StringOrNum::Num(self.vx),
                        inertia_speed as f64,
                        true,
                    );
                }
            }
        }

        if record_axis == "z" || record_axis == "xz" {
            if self.vz.abs() <= tolerance as f64 {
                if self.vz.abs() <= inertia_speed as f64 {
                    self.add_to_output(
                        ExpressionType::ZInertiaHit,
                        format!(
                            "Tick {} Vz (Hit)",
                            self.record_inertia["tick"].unwrap_tick()
                        ),
                        StringOrNum::Num(self.vz),
                        inertia_speed as f64,
                        true,
                    );
                } else {
                    self.add_to_output(
                        ExpressionType::ZInertiaMiss,
                        format!(
                            "Tick {} Vz (Miss)",
                            self.record_inertia["tick"].unwrap_tick()
                        ),
                        StringOrNum::Num(self.vz),
                        inertia_speed as f64,
                        true,
                    );
                }
            }
        }

        *self.record_inertia.get_mut("tick").unwrap().unwrap_tick_as_mut() += 1;
    }

    /// Auxiliary function for dealing with `possibilities()` functions.
    pub fn possibilities_helper(&mut self) {
        if self.record.is_empty() {
            return;
        }

        let record_axis = self.record["type"].unwrap_axis();

        let x_offset =
            self.record.get("x offset").unwrap_or(&PossibilitiesRecordParam::Xoffset(0.0f64)).unwrap_f64() * self.x.signum();

        let z_offset =
            self.record.get("z offset").unwrap_or(&PossibilitiesRecordParam::Zoffset(0.0f64)).unwrap_f64() * self.z.signum();

        let x_increment =
            self.record.get("x increment").unwrap_or(&PossibilitiesRecordParam::Xincrement(0.0f64)).unwrap_f64().copysign(self.x);

        let z_increment =
            self.record.get("z increment").unwrap_or(&PossibilitiesRecordParam::Xincrement(0.0f64)).unwrap_f64().copysign(self.z);

        let min_dist = self.record["min_distance"].unwrap_f64();
        let near_misses = self.record.get("miss");

        if record_axis == "z" {
            let z_distance = self.z + z_offset;
            let z_pixel_offset = z_distance % z_increment;

            if z_pixel_offset.abs() <= min_dist {
                self.add_to_output(
                    ExpressionType::ZLabel,
                    format!("Tick {}", self.record["tick"].unwrap_tick()),
                    StringOrNum::Num(z_distance),
                    z_distance - z_pixel_offset,
                    true,
                );
            } else {
                let z_offset_miss = z_increment - z_pixel_offset;

                if let Some(near_misses) = near_misses {
                    if z_offset_miss.abs() <= near_misses.unwrap_f64() {
                        self.add_to_output(
                            ExpressionType::ZLabel,
                            format!("Tick {}", self.record["tick"].unwrap_tick()),
                            StringOrNum::Num(z_distance + z_increment
                                - z_pixel_offset
                                - z_offset_miss),
                            z_distance + z_increment - z_pixel_offset,
                            true,
                        );
                    }
                }
            }
        } else if record_axis == "x" {
            let x_distance = self.x + x_offset;
            let x_pixel_offset = x_distance % x_increment;

            if x_pixel_offset.abs() <= min_dist {
                self.add_to_output(
                    ExpressionType::XLabel,
                    format!("Tick {}", self.record["tick"].unwrap_tick()),
                    StringOrNum::Num(x_distance),
                    x_distance - x_pixel_offset,
                    true,
                );
            } else {
                let x_offset_miss = x_increment - x_pixel_offset;

                if let Some(near_misses) = near_misses {
                    if x_offset_miss.abs() <= near_misses.unwrap_f64() {
                        self.add_to_output(
                            ExpressionType::XLabel,
                            format!("Tick {}", self.record["tick"].unwrap_tick()),
                            StringOrNum::Num(x_distance + x_increment
                                - x_pixel_offset
                                - x_offset_miss),
                            x_distance + x_increment - x_pixel_offset,
                            true,
                        );
                    }
                }
            }
        } else if record_axis == "xz" {
            let z_distance = self.z + z_offset;
            let z_pixel_offset = z_distance % z_increment;

            let x_distance = self.x + x_offset;
            let x_pixel_offset = x_distance % x_increment;

            if z_pixel_offset.abs() <= min_dist
                && x_pixel_offset.abs() <= min_dist
            {
                self.add_to_output(
                    ExpressionType::GeneralLabel,
                    format!("Tick {}", self.record["tick"].unwrap_tick()),
                    StringOrNum::Str("".to_string()),
                    0.0f64,
                    true,
                );

                self.add_to_output(
                    ExpressionType::XLabel,
                    "  X".to_string(),
                    StringOrNum::Num(x_distance),
                    x_distance - x_pixel_offset,
                    false,
                );

                self.add_to_output(
                    ExpressionType::ZLabel,
                    "  Z".to_string(),
                    StringOrNum::Num(z_distance),
                    z_distance - z_pixel_offset,
                    false,
                );
            } else {
                let z_offset_miss = z_increment - z_pixel_offset;
                let x_offset_miss = x_increment - x_pixel_offset;

                if let Some(near_misses) = near_misses {
                    if !(x_offset_miss.abs() > near_misses.unwrap_f64()
                        || z_offset_miss.abs() > near_misses.unwrap_f64())
                    {
                        self.add_to_output(
                            ExpressionType::GeneralLabel,
                            format!("Tick {}", self.record["tick"].unwrap_tick()),
                            StringOrNum::Str("".to_string()),
                            0.0f64,
                            true,
                        );

                        self.add_to_output(
                            ExpressionType::XLabel,
                            "  X".to_string(),
                            StringOrNum::Num(x_distance
                                + x_increment
                                - x_pixel_offset
                                - x_offset_miss),
                            x_distance
                                + x_increment
                                - x_pixel_offset,
                            false,
                        );

                        self.add_to_output(
                            ExpressionType::ZLabel,
                            "  Z".to_string(),
                            StringOrNum::Num(z_distance
                                + z_increment
                                - z_pixel_offset
                                - z_offset_miss),
                            z_distance
                                + z_increment
                                - z_pixel_offset,
                            false,
                        );
                    }
                }
            }
        }

        *self.record.get_mut("tick").unwrap().unwrap_tick_as_mut() += 1;
    }

    fn movement_multiplier_old(
        &self,
        slip: f32,
        is_sprinting: bool,
        speed: i32,
        slow: i32,
        state: State,
    ) -> f32 {
        /*
        Calculates and returns the movement multiplier `M`.

        See https://www.mcpk.wiki/wiki/Horizontal_Movement_Formulas
        */

        let mut m: f32;

        if (self.modifiers & Simulation::WATER as i32 != 0)
            || (self.modifiers & Simulation::LAVA as i32 != 0)
        {
            // It doesnt matter if you are in web
            m = 0.02;
        } else if matches!(state, State::Air) {
            m = 0.02;

            // In water, walk and sprint are the same, potion effects do not
            // affect water or air, shifting is different
            if (self.air_sprint_delay && self.previously_sprinting)
                || (!self.air_sprint_delay && is_sprinting)
            {
                m = (m as f64 + m as f64 * 0.3f64) as f32;
            }
        } else {
            // either on jump or on ground
            m = 0.1;

            // Deal with potion effects
            if speed > 0 {
                m = (m as f64 * (1.0f64 + (0.2f32 as f64) * (speed as f64))) as f32;
            }

            if slow > 0 {
                m = (m as f64 * (1.0f64 + (-0.15f32 as f64) * slow as f64).max(0.0f64)) as f32;
            }

            if is_sprinting {
                m = (m as f64 * (1.0 + 0.3f32 as f64)) as f32;
            }

            let drag = 0.91 * slip;
            m *= 0.16277136 / (drag * drag * drag);
        }

        m
    }

    fn movement_multiplier_new(
        &self,
        slip: f32,
        is_sprinting: bool,
        speed: i32,
        slow: i32,
        state: &State,
    ) -> f32 {
        /*
        Calculates and returns the movement multiplier `M`.

        See https://www.mcpk.wiki/wiki/Horizontal_Movement_Formulas
        */

        let mut m: f32;

        if (self.modifiers & Simulation::WATER as i32 != 0)
            || (self.modifiers & Simulation::LAVA as i32 != 0)
        {
            // It doesnt matter if you are in web
            m = 0.02;
        } else if matches!(state, State::Air) {
            m = 0.02;

            // In water, walk and sprint are the same, potion effects do not
            // affect water or air, shifting is different
            if (self.air_sprint_delay && self.previously_sprinting)
                || (!self.air_sprint_delay && is_sprinting)
            {
                m = (m as f64 + m as f64 * 0.3f64) as f32;
            }
        } else {
            // either on jump or on ground
            m = 0.1;

            // Deal with potion effects
            if speed > 0 {
                m = (m as f64 * (1.0f64 + (0.2f32 as f64) * speed as f64)) as f32;
            }

            if slow > 0 {
                m = (m as f64 * (1.0f64 + (-0.15f32 as f64) * slow as f64).max(0.0f64)) as f32;
            }

            if is_sprinting {
                m = (m as f64 * (1.0f64 + (0.30000010133f64 as f32) as f64)) as f32;
            }

            m *= (0.21600002f64 as f32) / (slip * slip * slip);
        }

        m
    }

    pub fn movement_values(&self) -> (f32, f32) {
        /*
        Returns two values `forward` and `strafe` either valued at
        -1, 0, or 1 based on `self.inputs`.
        */

        let forward = if self.inputs.contains('w') {
            1.0
        } else if self.inputs.contains('s') {
            -1.0
        } else {
            0.0
        };

        let strafe = if self.inputs.contains('a') {
            1.0
        } else if self.inputs.contains('d') {
            -1.0
        } else {
            0.0
        };

        (forward, strafe)
    }
}

enum StringOrNum {
    Str(String),
    Num(f64),
}

impl StringOrNum {
    fn get_as_string(&mut self) -> String {
        match self {
            StringOrNum::Str(string) => {
                string.to_string()
            }

            StringOrNum::Num(_) => {
                panic!();
            }
        }
    }

    fn get_as_float(&mut self) -> f64 {
        match self {
            StringOrNum::Str(_) => {
                panic!();
            }

            StringOrNum::Num(float) => {
                *float
            }
        }
    }
}


impl PlayerSimulationXZ {
    /// label = "", string_or_num = StringOrNum::Str(""), num2 = 0.0f64, strip_label = true
    /// 
    /// "".to_string(), StringOrNum::Str("".to_string()), 0.0f64, true
    fn add_to_output(
        &mut self,
        mut expression_type: ExpressionType,
        mut label: String,
        mut string_or_num: StringOrNum,
        num2: f64,
        strip_label: bool,
    ) -> Option<f64> {
        if strip_label {
            label = label.trim().to_string();
        }

        match expression_type {
            ExpressionType::ZLabel
            | ExpressionType::XLabel
            | ExpressionType::GeneralLabelWithNumber => {
                let num = string_or_num.get_as_float();

                if num2 != 0.0 {
                    expression_type =
                        unsafe { std::mem::transmute(expression_type as i32 + 1) };

                    let nn = num - num2;

                    self.output.push((
                        expression_type,
                        OutputExpression::GeneralLabelWithExpression(
                            label,
                            ": ",
                            self.truncate_number(num2),
                            if nn <= 0.0 { " - " } else { " + " },
                            self.truncate_number(nn.abs()),
                        ),
                    ));

                    return Some(nn);
                } else {
                    self.output.push((
                        expression_type,
                        OutputExpression::GeneralLabelWithNumber(
                            label,
                            ": ",
                            self.truncate_number(num),
                        ),
                    ));

                    return Some(num);
                }
            }

            ExpressionType::Text => {
                let mut string = string_or_num.get_as_string();

                if strip_label {
                    string = string.trim().to_string();
                }

                self.output.push((
                    expression_type,
                    OutputExpression::Text(string),
                ));
            }

            ExpressionType::Warning => {
                let string = string_or_num.get_as_string();

                self.output.push((
                    expression_type,
                    OutputExpression::Warning(
                        "Warning",
                        ": ",
                        string.trim().to_string(),
                    ),
                ));
            }

            ExpressionType::ZInertiaHit
            | ExpressionType::XInertiaHit
            | ExpressionType::ZInertiaMiss
            | ExpressionType::XInertiaMiss => {
                let num = string_or_num.get_as_float();
                let a = (num.abs() - num2.abs()).abs();

                self.output.push((
                    expression_type,
                    OutputExpression::GeneralInertiaLabel(
                        label,
                        ": ",
                        self.truncate_number(num),
                        " (",
                        self.truncate_number(a),
                        ")",
                    ),
                ));
            }

            ExpressionType::GeneralLabel => {
                self.output.push((
                    expression_type,
                    OutputExpression::GeneralLabel(label),
                ));
            }

            _ => {}
        }

        None
    }
}

impl PlayerSimulationXZ {

    // Equivalent to @record_to_call_stack
    pub fn repeat(
        &mut self,
        sequence: MothballSequence,
        count: i32,
    ) {
        self.call_stack.push("repeat".to_string());

        if count < 0 {
            panic!("repeat() must have a nonnegative argument 'count'");
        }

        if count == 0 {
            self.call_stack.pop();
            return;
        }

        let parsed_tokens = self.parse(sequence.deref(), None, None);

        let mut runnables = Vec::new();

        for token in parsed_tokens {
            let runnable = self.tokenize(
                token.as_str(),
                Some(self.local_vars.clone()),
            );

            self.run(runnable.clone());

            runnables.push(runnable);
        }

        // This is because if something like repeat(var(a,1) sj(a),2)
        // is run, tokenizer will fail to process a = 1 since it wasn't
        // actually run yet.
        for _ in 0..count - 1 {
            if self.stop_flag {
                panic!("Stopped execution");
            }

            for runnable in &runnables {
                self.run((*runnable).clone());
            }
        }

        self.call_stack.pop();
    }


    pub fn print(&mut self, mut string: String) {
        self.call_stack.push("print".to_string());

        if self.reverse {
            string = parser::formatted(self.local_vars.clone(), &string);

            string = string.chars().rev().collect();
        }

        self.add_to_output(
            ExpressionType::Text,
            "".to_string(),
            StringOrNum::Str(string),
            0.0,
            true,
        );

        self.call_stack.pop();
    }


    pub fn var(
        &mut self,
        variable_name: String,
        value: Option<MothballSequence>,
    ) {
        self.call_stack.push("var".to_string());

        let regex = regex::Regex::new(
            r"^([a-zA-Z_][a-zA-Z0-9_]*)$"
        ).unwrap();


        if !regex.is_match(&variable_name) {
            panic!(
                "'{}' is not a valid variable name",
                variable_name
            );
        }


        if self.has_function(variable_name.trim()) {
            panic!(
                "Cannot set variable name '{}' as it is a function name",
                variable_name.trim()
            );
        }


        if value.is_none() {
            self.local_vars.insert(
                variable_name,
                parser::Data::Float(self.last_returned_value),
            );

            self.call_stack.pop();
            return;
        }


        let value = value.unwrap();

        let mut final_value = value.to_string();


        match Ok::<f64, errors::RuntimeError>(expr_eval::evaluate(
            &final_value,
            self.local_vars.clone(),
        )) {
            Ok(result) => {
                final_value = result.to_string();
            }

            Err(_) => {
                let result = parser::safe_eval(
                    self.local_vars.clone(),
                    &final_value,
                    parser::DataType::Str,
                    self.local_vars.clone(),
                );

                if let Ok(parser::Data::Str(final_value)) = result {
                    ()
                } else {
                    panic!(
                        "Unable to deduce the value of '{}'",
                        final_value
                    );
                }

                println!("dub check here")
            }
        }


        self.local_vars.insert(
            variable_name,
            parser::Data::Str(final_value),
        );

        self.call_stack.pop();
    }



    pub fn setprecision(
        &mut self,
        decimal_places: i32,
    ) {
        if decimal_places < 0 || decimal_places > 16 {
            panic!(
                "precision() only takes integers between 0 to 16 inclusive, got {} instead.",
                decimal_places
            );
        }

        self.precision = decimal_places;
    }

    pub fn ballhelp(
        &mut self,
        _: MothballSequence,
    ) -> &str {
        self.call_stack.push("ballhelp".to_string());
        self.call_stack.pop();
        return "fuck you no im not doing this"
    }
}

impl PlayerSimulationXZ {
    fn parse(&self, string: &str, splitters: Option<Vec<char>>, strict_whitespace: Option<bool>) -> Vec<String> {
        let splitters_arg = match splitters {
            Some(splitter_vec) => splitter_vec,
            None => vec!['\n', ' ', '\r', '\t'],
        };

        let strict_whitespace_arg = match strict_whitespace {
            Some(boolean) => boolean,
            None => true,
        };

        let output = parser::parse(self.call_stack.clone(), string, splitters_arg, strict_whitespace_arg);
        match output {
            Ok(out) => out,
            Err(error) => panic!("{}", error)
        }
    }

    fn tokenize(&mut self, string: &str, locals: Option<IndexMap<String, parser::Data>>) -> parser::Tokenized {
        let output = parser::tokenize(self, string, locals);
        match output {
            Ok(out) => out,
            Err(error) => panic!("{}", error)
        }
    }
}

impl PlayerSimulationXZ {
    pub fn walk(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        self.move_player(
            duration,
            rotation,
            0.0f64,
            slip,
            false,
            false,
            speed,
            slow,
            State::Ground,
        );
    }

    pub fn walk45(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        self.move_player(
            duration,
            rotation,
            45.0,
            slip,
            false,
            false,
            speed,
            slow,
            State::Ground
        );
    }

    pub fn sprint(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        self.move_player(
            duration,
            rotation,
            0.0f64,
            slip,
            true,
            false,
            speed,
            slow,
            State::Ground,
        );
    }

    pub fn sprint45(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        self.move_player(
            duration,
            rotation,
            45.0,
            slip,
            true,
            false,
            speed,
            slow,
            State::Ground
        );
    }

    pub fn walkair(&mut self, duration: i32, rotation: Option<f32>) {
        self.move_player(
            duration,
            rotation,
            0.0f64,
            Some(1.0),
            false,
            false,
            None,
            None,
            State::Air
        );
    }

    pub fn walkair45(&mut self, duration: i32, rotation: Option<f32>) {
        self.move_player(
            duration,
            rotation,
            45.0,
            Some(1.0),
            false,
            false,
            None,
            None,
            State::Air
        );
    }

    pub fn sprintair(&mut self, duration: i32, rotation: Option<f32>) {
        self.move_player(
            duration,
            rotation,
            0.0f64,
            Some(1.0),
            true,
            false,
            None,
            None,
            State::Air
        );
    }

    pub fn sprintair45(&mut self, duration: i32, rotation: Option<f32>) {
        self.move_player(
            duration,
            rotation,
            45.0,
            Some(1.0),
            true,
            false,
            None,
            None,
            State::Air
        );
    }

    pub fn walkjump(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if duration > 0 {
            self.move_player(
                1,
                rotation,
                0.0f64,
                slip,
                false,
                false,
                speed,
                slow,
                State::Jump
            );
            self.walkair(duration - 1, rotation);
        }
    }

    pub fn walkjump45(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if duration > 0 {
            self.move_player(
                1,
                rotation,
                45.0,
                slip,
                false,
                false,
                speed,
                slow,
                State::Jump
            );
            self.walkair45(duration - 1, rotation);
        }
    }

    pub fn walkpessi(
        &mut self,
        duration: i32,
        delay: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
    ) {
        if delay == 0 {
            self.walkjump(duration, rotation, slip, None, None);
        } else if duration > 0 {
            let mut delay = delay;

            if delay > duration {
                delay = duration;
            }

            let input = self.inputs.clone();
            self.inputs = String::new();
            self.stopjump(delay, slip);
            self.inputs = input;
            self.walkair(duration - delay, rotation);
        }
    }

    pub fn walkpessi45(
        &mut self,
        duration: i32,
        delay: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
    ) {
        if delay == 0 {
            self.walkjump45(duration, rotation, slip, None, None);
        } else if duration > 0 {
            let delay = delay;

            if delay > duration {
                delay == duration;
            }

            let input = self.inputs.clone();
            self.inputs = String::new();
            self.stopjump(delay, slip);
            self.inputs = input;
            self.walkair45(duration - delay, rotation);
        }
    }

    pub fn sprintjump(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if duration > 0 {
            self.move_player(
                1,
                rotation,
                0.0f64,
                slip,
                true,
                false,
                speed,
                slow,
                State::Jump
            );
            self.sprintair(duration - 1, rotation);
        }
    }

    pub fn sprintjump45(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if duration > 0 {
            self.move_player(
                1,
                rotation,
                0.0f64,
                slip,
                true,
                false,
                speed,
                slow,
                State::Jump
            );
            self.sprintair45(duration - 1, rotation);
        }
    }

    pub fn sprintstrafejump(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if duration > 0 {
            self.inputs = "wa".to_string();

            self.move_player(
                1,
                rotation,
                self.get_optimal_strafe_jump_angle(speed, slow, slip, None),
                slip,
                true,
                false,
                speed,
                slow,
                State::Jump
            );

            self.inputs = "w".to_string();
            self.sprintair(duration - 1, rotation);
        }
    }

    pub fn sprintstrafejump45(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if duration > 0 {
            self.inputs = "wa".to_string();

            self.move_player(
                1,
                rotation,
                self.get_optimal_strafe_jump_angle(speed, slow, slip, None),
                slip,
                true,
                false,
                speed,
                slow,
                State::Jump
            );

            self.sprintair45(duration - 1, rotation);
        }
    }

    pub fn sprintpessi(
        &mut self,
        duration: i32,
        delay: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
    ) {
        if delay == 0 {
            self.sprintjump(duration, rotation, slip, None, None);
        } else if duration > 0 {
            let mut delay = delay;

            if delay > duration {
                delay = duration;
            }

            let input = self.inputs.clone();
            self.inputs = String::new();
            self.stopjump(delay, slip);
            self.inputs = input;
            self.sprintair(duration - delay, rotation);
        }
    }

    pub fn sprintpessi45(
        &mut self,
        duration: i32,
        delay: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
    ) {
        if delay == 0 {
            self.sprintjump(duration, rotation, slip, None, None);
        } else if duration > 0 {
            let mut delay = delay;

            if delay > duration {
                delay = duration;
            }

            let input = self.inputs.clone();
            self.inputs = String::new();
            self.stopjump(delay, slip);
            self.inputs = input;
            self.sprintair45(duration - delay, rotation);
        }
    }

    pub fn forcemomentum(
        &mut self,
        duration: i32,
        delay: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if delay < 0 {
            // pass // raise error
        } else if duration > 0 {
            if delay == 0 {
                self.sprintjump(duration, rotation, slip, speed, slow);
            } else {
                let mut delay = delay;

                if delay > duration {
                    delay = duration;
                }

                self.walkjump(delay, rotation, slip, speed, slow);
                self.sprintair(duration - delay, rotation);
            }
        }
    }

    pub fn forcemomentum45(
        &mut self,
        duration: i32,
        delay: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if delay < 0 {
            // pass // raise error
        } else if duration > 0 {
            if delay == 0 {
                self.sprintjump45(duration, rotation, slip, speed, slow);
            } else {
                let mut delay = delay;

                if delay > duration {
                    delay = duration;
                }

                self.walkjump45(delay, rotation, slip, speed, slow);
                self.sprintair45(duration - delay, rotation);
            }
        }
    }

    pub fn sneak(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        self.move_player(
            duration,
            rotation,
            0.0f64,
            slip,
            false,
            true,
            speed,
            slow,
            State::Ground
        );
    }

    pub fn sneak45(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        self.move_player(
            duration,
            rotation,
            45.0,
            slip,
            false,
            true,
            speed,
            slow,
            State::Ground
        );
    }

    pub fn sneakair(&mut self, duration: i32, rotation: Option<f32>) {
        self.move_player(
            duration,
            rotation,
            0.0f64,
            Some(1.0),
            false,
            true,
            None,
            None,
            State::Air
        );
    }

    pub fn sneakair45(&mut self, duration: i32, rotation: Option<f32>) {
        self.move_player(
            duration,
            rotation,
            45.0,
            Some(1.0),
            false,
            true,
            None,
            None,
            State::Air
        );
    }

    pub fn sneakjump(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if duration > 0 {
            self.move_player(
                1,
                rotation,
                0.0f64,
                slip,
                false,
                true,
                speed,
                slow,
                State::Jump
            );
            self.sneakair(duration - 1, rotation);
        }
    }

    pub fn sneakjump45(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if duration > 0 {
            self.move_player(
                1,
                rotation,
                45.0,
                slip,
                false,
                true,
                speed,
                slow,
                State::Jump
            );
            self.sneakair45(duration - 1, rotation);
        }
    }

    pub fn stop(&mut self, duration: i32, slip: Option<f32>) {
        self.move_player(
            duration,
            None,
            0.0f64,
            slip,
            false,
            false,
            None,
            None,
            State::Ground
        );
    }

    pub fn stopair(&mut self, duration: i32) {
        self.move_player(
            duration,
            None,
            0.0f64,
            Some(1.0),
            false,
            false,
            None,
            None,
            State::Air
        );
    }

    pub fn stopjump(&mut self, duration: i32, slip: Option<f32>) {
        if duration > 0 {
            self.move_player(
                1,
                None,
                0.0f64,
                slip,
                false,
                false,
                None,
                None,
                State::Jump
            );
            self.stopair(duration - 1);
        }
    }

    pub fn sneakstop(&mut self, duration: i32, slip: Option<f32>) {
        self.move_player(
            duration,
            None,
            0.0f64,
            slip,
            false,
            true,
            None,
            None,
            State::Ground
        );
    }

    pub fn sneakstopair(&mut self, duration: i32) {
        self.move_player(
            duration,
            None,
            0.0f64,
            Some(1.0),
            false,
            true,
            None,
            None,
            State::Air
        );
    }

    pub fn sneakstopjump(&mut self, duration: i32, slip: Option<f32>) {
        if duration > 0 {
            self.move_player(
                1,
                None,
                0.0f64,
                slip,
                false,
                true,
                None,
                None,
                State::Jump
            );
            self.stopair(duration - 1);
        }
    }

    pub fn sneaksprint(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        self.move_player(
            duration,
            rotation,
            0.0f64,
            slip,
            true,
            true,
            speed,
            slow,
            State::Ground
        );
    }

    pub fn sneaksprint45(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        self.move_player(
            duration,
            rotation,
            45.0,
            slip,
            true,
            true,
            speed,
            slow,
            State::Ground
        );
    }

    pub fn sneaksprintair(&mut self, duration: i32, rotation: Option<f32>) {
        self.move_player(
            duration,
            rotation,
            0.0f64,
            Some(1.0),
            true,
            true,
            None,
            None,
            State::Air
        );
    }

    pub fn sneaksprintair45(&mut self, duration: i32, rotation: Option<f32>) {
        self.move_player(
            duration,
            rotation,
            45.0,
            Some(1.0),
            true,
            true,
            None,
            None,
            State::Air
        );
    }

    pub fn sneaksprintjump(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if duration > 0 {
            self.move_player(
                1,
                rotation,
                0.0f64,
                slip,
                true,
                true,
                speed,
                slow,
                State::Jump
            );
            self.sneaksprintair(duration - 1, rotation);
        }
    }

    pub fn sneaksprintjump45(
        &mut self,
        duration: i32,
        rotation: Option<f32>,
        slip: Option<f32>,
        speed: Option<i32>,
        slow: Option<i32>,
    ) {
        if duration > 0 {
            self.inputs = "wa".to_string();

            self.move_player(
                1,
                rotation,
                self.get_optimal_strafe_jump_angle(speed, slow, slip, Some(true)),
                slip,
                true,
                true,
                speed,
                slow,
                State::Jump
            );

            self.sneaksprintair45(duration - 1, rotation);
        }
    }

    // PRIVATE FUNCTION
    fn get_optimal_strafe_jump_angle(
        &self,
        speed: Option<i32>,
        slow: Option<i32>,
        slip: Option<f32>,
        is_sneaking_arg: Option<bool>,
    ) -> f64 {
        let is_sneaking = handle_keyword_arg(is_sneaking_arg, false);
        let mut player = copy_player(self);

        player.x = 0.0;
        player.z = 0.0;
        player.vx = 0.0;
        player.vz = 0.0;
        player.angle_queue = VecDeque::new();
        player.rotation = 0.0;

        if let Some(speed) = speed {
            player.speed_effect = speed;
        }

        if let Some(slow) = slow {
            player.slow_effect = slow;
        }

        if let Some(slip) = slip {
            player.default_ground_slip = slip;
        }

        if is_sneaking {
            player.simulate("snsj.wa".to_string(), true, None, true);
        } else {
            player.simulate("sj.wa".to_string(), true, None, true);
        }

        // print(abs(deg(arctan(-player.vx, player.vz))))
        (-player.vx).atan2(player.vz).to_degrees().abs()
    }
}

impl PlayerSimulationXZ {
    pub fn outz(&mut self, centered_about: f32, label: &str) {
        self.last_returned_value = self.add_to_output(
            ExpressionType::ZLabel,
            label.to_string(),
            StringOrNum::Num(self.z),
            centered_about as f64,
            true,
        ).unwrap();
    }

    pub fn zmm(&mut self, centered_about: f32, label: &str) {
        self.last_returned_value = self.add_to_output(
            ExpressionType::ZLabel,
            label.to_string(),
            StringOrNum::Num(math::dist_to_mmf64(self.z)),
            centered_about as f64,
            true,
        ).unwrap();
    }

    pub fn zb(&mut self, centered_about: f32, label: &str) {
        self.last_returned_value = self.add_to_output(
            ExpressionType::ZLabel,
            label.to_string(),
            StringOrNum::Num(math::dist_to_blockf64(self.z)),
            centered_about as f64,
            true,
        ).unwrap();
    }

    pub fn outvz(&mut self, centered_about: f32, label: &str) {
        self.last_returned_value = self.add_to_output(
            ExpressionType::ZLabel,
            label.to_string(),
            StringOrNum::Num(self.vz),
            centered_about as f64,
            true
        ).unwrap();
    }

    pub fn outx(&mut self, centered_about: f32, label: &str) {
        self.last_returned_value = self.add_to_output(
            ExpressionType::XLabel,
            label.to_string(),
            StringOrNum::Num(self.x),
            centered_about as f64,
            true,
        ).unwrap();
    }

    pub fn xmm(&mut self, centered_about: f32, label: &str) {
        self.last_returned_value = self.add_to_output(
            ExpressionType::XLabel,
            label.to_string(),
            StringOrNum::Num(math::dist_to_mmf64(self.x)),
            centered_about as f64,
            true,
        ).unwrap();
    }

    pub fn xb(&mut self, centered_about: f32, label: &str) {
        self.last_returned_value = self.add_to_output(
            ExpressionType::XLabel,
            label.to_string(),
            StringOrNum::Num(math::dist_to_blockf64(self.x)),
            centered_about as f64,
            true
        ).unwrap();
    }

    pub fn outvx(&mut self, centered_about: f32, label: &str) {
        self.last_returned_value = self.add_to_output(
            ExpressionType::XLabel,
            label.to_string(),
            StringOrNum::Num(self.vx),
            centered_about as f64,
            true
        ).unwrap();
    }

    pub fn vec(&mut self) {
        self.add_to_output(
            ExpressionType::GeneralLabelWithNumber,
            "Speed".to_string(),
            StringOrNum::Num((self.vx.powi(2) + self.vz.powi(2)).sqrt()),
            0.0f64,
            true
        );

        self.add_to_output(
            ExpressionType::GeneralLabelWithNumber,
            "Angle".to_string(),
            StringOrNum::Num(((-self.vx).atan2(self.vz)).to_degrees()),
            0.0f64,
            true
        );
    }

    pub fn outangle(&mut self, centered_about: f32, label: &str) {
        self.last_returned_value = self.add_to_output(
            ExpressionType::GeneralLabelWithNumber,
            label.to_string(),
            StringOrNum::Num(self.rotation as f64),
            centered_about as f64,
            true,
        ).unwrap();
    }

    pub fn outturn(&mut self, centered_about: f32, label: &str) {
        self.last_returned_value = self.add_to_output(
            ExpressionType::GeneralLabelWithNumber,
            label.to_string(),
            StringOrNum::Num(self.last_turn as f64),
            centered_about as f64,
            true
        ).unwrap();
    }

    pub fn effectsmultiplier(&mut self, speed: Option<i32>, slow: Option<i32>) {
        let speed = speed.unwrap_or(self.speed_effect);
        let slow = slow.unwrap_or(self.slow_effect);

        if speed < 0 || speed > 256 {
            panic!(
                "argument 'speed' should be an integer between 0 and 256 inclusive, not {}",
                speed
            );
        }

        if slow < 0 || slow > 256 {
            panic!(
                "argument 'slow' should be an integer between 0 and 256 inclusive, not {}",
                slow
            );
        }

        let multiplier = ((1.0 + (0.2 * speed as f32)) * (1.0 - (0.15 * slow as f32)))
            .max(0.0)
            * 100.0;

        self.add_to_output(
            ExpressionType::GeneralLabel,
            format!(
                "Speed {} Slow {} ({}% base speed)",
                speed,
                slow,
                multiplier.round() as i32
            ),
            StringOrNum::Str("".to_string()),
            0.0f64,
            true
        );
    }

    pub fn angleinfo(&mut self, angle: f32) {
        let angle_rad = angle * Simulation::PI as f32 / 180.0f32;
        let sin_index = ((angle_rad * 10430.378f32) as i32 & 65535) as u64;
        let cos_index = (((angle_rad * 10430.378f32) + 16384.0f32) as i32 & 65535) as u64;

        let sin_value = (sin_index as f64 * Simulation::PI * 2.0 / 65536.0).sin();
        let cos_value = (cos_index as f64 * Simulation::PI * 2.0 / 65536.0).cos();

        let cos_index_adj = (cos_index as i32 - 16384).rem_euclid(65536);

        let sin_angle = sin_value.asin().to_degrees();
        let cos_angle = cos_value.asin().to_degrees();
        let normal = (sin_value.powi(2) + cos_value.powi(2)).sqrt();

        let a = self.truncate_number(angle as f64);
        let sv = self.truncate_number(sin_value);
        let sa = self.truncate_number(sin_angle);
        let cv = self.truncate_number(cos_value);
        let ca = self.truncate_number(cos_angle);
        let norm = self.truncate_number(normal);

        let padding1 = std::cmp::max(6, a.len());
        let padding2 = *[
            5,
            a.len(),
            sv.len(),
            sa.len(),
            cv.len(),
            ca.len(),
            norm.len(),
        ]
        .iter()
        .max()
        .unwrap();

        self.add_to_output(
            ExpressionType::Text,
            format!(
                "{:<padding1$} {:<padding2$} {:<padding2$} {:<padding2$}",
                a,
                "Value",
                "Angle",
                "Index",
                padding1 = padding1,
                padding2 = padding2,
            ),
            StringOrNum::Str("".to_string()),
            0.0f64,
            true
        );

        self.add_to_output(
            ExpressionType::Text,
            format!(
                "{:<padding1$} {:<padding2$} {:<padding2$} {} ",
                "Sin",
                sv,
                sa,
                sin_index,
                padding1 = padding1,
                padding2 = padding2,
            ),
            StringOrNum::Str("".to_string()),
            0.0f64,
            true
        );

        self.add_to_output(
            ExpressionType::Text,
            format!(
                "{:<padding1$} {:<padding2$} {:<padding2$} {} ({})",
                "Cos",
                cv,
                ca,
                cos_index_adj,
                cos_index,
                padding1 = padding1,
                padding2 = padding2,
            ),
            StringOrNum::Str("".to_string()),
            0.0f64,
            true
        );

        self.add_to_output(
            ExpressionType::Text,
            format!(
                "{:<padding1$} {:<padding2$}",
                "Normal",
                norm,
                padding1 = padding1,
                padding2 = padding2,
            ),
            StringOrNum::Str("".to_string()),
            0.0f64,
            true
        );
    }
}

impl PlayerSimulationXZ {
    pub fn face(&mut self, angle_in_degrees: f32) {
        self.rotation = angle_in_degrees;
    }

    pub fn turn(&mut self, angle_in_degrees: f32) {
        self.rotation += angle_in_degrees;
    }

    pub fn setposz(&mut self, value: f64) {
        self.z = value;
    }

    pub fn setvz(&mut self, value: f64) {
        self.vz = value;
    }

    pub fn setposx(&mut self, value: f64) {
        self.x = value;
    }

    pub fn setvx(&mut self, value: f64) {
        self.vx = value;
    }

    pub fn addposz(&mut self, value: f64) {
        self.z += value;
    }

    pub fn addposx(&mut self, value: f64) {
        self.x += value;
    }

    pub fn addvx(&mut self, value: f64) {
        self.vx += value;
    }

    pub fn addvz(&mut self, value: f64) {
        self.vz += value;
    }

    pub fn setslip(&mut self, value: f32) {
        self.default_ground_slip = value;
    }

    pub fn inertia(&mut self, value: f32, single_axis: Option<bool>) {
        self.inertia_threshold = value;

        if let Some(single_axis) = single_axis {
            if single_axis {
                self.inertia_axis = 1;
            } else {
                self.inertia_axis = 2;
            }
        }
    }

    /// `toggle` will toggle off if it is the string `"false"`, else it will assume true.
    ///
    /// Toggles the player's sprint air delay. If toggled, it takes 1 tick longer to activate sprint
    /// in midair if the previous tick was unsprinted midair.
    ///
    /// Versions 1.8 to 1.19 have a sprint air delay while later versions don't, so if you intend
    /// to calculate 1.20+ movement, set sprint air delay to `false`.
    pub fn sprintairdelay(&mut self, toggle: bool) {
        if toggle {
            self.air_sprint_delay = true;
        } else {
            self.air_sprint_delay = false;
        }
    }

    /// `toggle` will toggle true if it is the string `"true"`, else it will assume false.
    ///
    /// Toggles the player's sneak delay. If toggled, it takes 1 tick longer to activate sneak if
    /// the previous tick didn't sneak.
    ///
    /// Versions 1.8 to 1.19 dont have a sneak delay while later versions do, so if you intend to
    /// calculate 1.20+ movement, set sprint air delay to `true`.
    pub fn sneakdelay(&mut self, toggle: bool) {
        if toggle {
            self.sneak_delay = true;
        } else {
            self.sneak_delay = false;
        }
    }

    /// Set's inertia to affect individual axis.
    pub fn singleaxisinertia(&mut self, toggle: bool) {
        if toggle {
            self.inertia_axis = 1;
        } else {
            self.inertia_axis = 2;
        }
    }

    /// String should be in the form `1.n`, for example the minimum `1.8` is default.
    /// Max is currently `1.20`
    pub fn version(&mut self, string: &str) {
        let components: Vec<&str> = string.split('.').collect();

        let (one, version_number, patch_number) = if components.len() == 2 {
            (
                components[0].parse::<i32>().unwrap(),
                components[1].parse::<i32>().unwrap(),
                0,
            )
        } else if components.len() == 3 {
            (
                components[0].parse::<i32>().unwrap(),
                components[1].parse::<i32>().unwrap(),
                components[2].parse::<i32>().unwrap(),
            )
        } else {
            panic!("{} is not a valid version", string);
        };

        if one != 1 {
            panic!("{} is not a valid version", string);
        }

        if version_number > 8 {
            self.inertia(0.003, None);
        }

        if version_number > 13 {
            self.sneakdelay(true);
            self.version_computation = Simulation::NEW_COMPUTATION;
        } else {
            self.version_computation = Simulation::OLD_COMPUTATION;
        }

        if version_number > 19 || (version_number == 19 && patch_number > 3) {
            self.sprintairdelay(false);
        }

        if version_number == 21 && patch_number >= 5 {
            self.inertia_axis = 2;
        }
    }

    /// Gives the player speed, where `speed(0)` is equivalent to no speed effects and
    /// `speed(256)` is the maximum speed effect.
    ///
    /// `multiplier` is a positive integer from 0 to 256, raises ValueError if integer
    /// provided is not within this range.
    pub fn speed(&mut self, multiplier: i32) {
        if multiplier < 0 || multiplier > 256 {
            panic!(
                "speed() takes an integer between 0 and 256 inclusive, not {}",
                multiplier
            );
        }

        self.speed_effect = multiplier;
    }

    /// Gives the player slowness, where `slow(0)` is equivalent to no slow effects and
    /// `slow(7)` is the maximum slowness effect.
    ///
    /// `multiplier` is a positive integer from 0 to 256, raises ValueError if integer
    /// provided is not within this range.
    ///
    /// Slowness is calculated with `max(1 + (-0.15) * multiplier, 0)` so `slow(7)`
    /// already gives the maximum slowness effect which would scale ground velocity by
    /// `0`. This also means that
    ///
    /// `slow(7) = slow(8) = ... = slow(256)`
    ///
    /// If the slowness effect was unbounded, then `slow(7)` and stronger effects would
    /// result in moving backwards.
    pub fn slowness(&mut self, multiplier: i32) {
        if multiplier < 0 || multiplier > 256 {
            panic!(
                "slow() takes an integer between 0 and 256 inclusive, not {}",
                multiplier
            );
        }

        self.slow_effect = multiplier;
    }

    /// Adds `angles` to the rotation queue, each angle being used for 1 tick.
    ///
    /// In mothball syntax, `anglequeue(1,-2,3) walk(3)` is the same as
    /// `face(1) walk face(-2) walk face(3) walk`
    pub fn anglequeue(&mut self, angles: Vec<f32>) {
        for angle in angles {
            self.angle_queue.push_back(angle);
        }
    }

    /// Adds `angles` to the rotation queue, each tick turning `angle` degrees.
    ///
    /// In mothball syntax, `turnqueue(1,-2,3) walk(3)` is the same as
    /// `turn(1) walk turn(-2) walk turn(3) walk`
    pub fn turnqueue(&mut self, angles: Vec<f32>) {
        for angle in angles {
            self.turn_queue.push_back(angle);
        }
    }
}

impl PlayerSimulationXZ {
    /// Displays ticks where while `sequence` is run, the player reaches certain
    /// block milestones ONLY ON Z (as determined by `increment`, default
    /// 0.0625) less than or equal to `min_distance`, useful to check which
    /// tiers results in precise landings.
    ///
    /// Use `offset` to offset each tick accordingly. An offset of 0.6 (which is
    /// default) is for jumping from block to block. An offset of -0.6 is for z
    /// neos. An offset of 0 will just record pure distances, helpful for
    /// example doing "block avoid" jumps.
    ///
    /// `increment` is set to 0.0625 by default, but in modern versions you can
    /// set it to 0.03125 for useful results with newer blocks.
    ///
    /// if a float for `miss` is provided, it will also display ticks that are
    /// `miss` away from reaching a milestone.
    pub fn possibilities(
        &mut self,
        sequence: MothballSequence,
        min_distance: f32,
        offset: f32,
        increment: f32,
        miss: Option<f32>,
    ) {

        if self.record.is_empty() {
            self.record = HashMap::new();
            self.record.insert("type".to_string(), PossibilitiesRecordParam::Axis("z"));
            self.record.insert("tick".to_string(), PossibilitiesRecordParam::Tick(1));
            self.record.insert("min_distance".to_string(), PossibilitiesRecordParam::MinDist(min_distance as f64));
            self.record.insert("z offset".to_string(), PossibilitiesRecordParam::Zoffset(offset as f64));
            self.record.insert("z increment".to_string(), PossibilitiesRecordParam::Zincrement(increment as f64));
            self.record.insert("miss".to_string(), PossibilitiesRecordParam::Miss(Some(miss.unwrap() as f64)));
        } else {
            panic!("Nested possibilities functions are not allowed.");
        }

        self.simulate(sequence.to_string(), false, None, true);
        self.record = HashMap::new();
    }

    pub fn xpossibilities(
        &mut self,
        sequence: MothballSequence,
        min_distance: f32,
        offset: f32,
        increment: f32,
        miss: Option<f32>,
    ) {
        if self.record.is_empty() {
            self.record = HashMap::new();
            self.record.insert("type".to_string(), PossibilitiesRecordParam::Axis("x"));
            self.record.insert("tick".to_string(), PossibilitiesRecordParam::Tick(1));
            self.record.insert("min_distance".to_string(), PossibilitiesRecordParam::MinDist(min_distance as f64));
            self.record.insert("x offset".to_string(), PossibilitiesRecordParam::Xoffset(offset as f64));
            self.record.insert("x increment".to_string(), PossibilitiesRecordParam::Xincrement(increment as f64));
            self.record.insert("miss".to_string(), PossibilitiesRecordParam::Miss(Some(miss.unwrap() as f64)));
        } else {
            panic!("Nested possibilities functions are not allowed.");
        }

        self.simulate(sequence.to_string(), false, None, true);
        self.record = HashMap::new();
    }

    pub fn xzpossibilities(
        &mut self,
        sequence: MothballSequence,
        min_distance: f32,
        x_offset: f32,
        z_offset: f32,
        x_increment: f32,
        z_increment: f32,
        miss: Option<f32>,
    ) {
        if self.record.is_empty() {
            self.record = HashMap::new();
            self.record.insert("type".to_string(), PossibilitiesRecordParam::Axis("xz"));
            self.record.insert("tick".to_string(), PossibilitiesRecordParam::Tick(1));
            self.record.insert("min_distance".to_string(), PossibilitiesRecordParam::MinDist(min_distance as f64));
            self.record.insert("x offset".to_string(), PossibilitiesRecordParam::Xoffset(x_offset as f64));
            self.record.insert("x increment".to_string(), PossibilitiesRecordParam::Xincrement(x_increment as f64));
            self.record.insert("z offset".to_string(), PossibilitiesRecordParam::Zoffset(z_offset as f64));
            self.record.insert("z increment".to_string(), PossibilitiesRecordParam::Zincrement(z_increment as f64));
            self.record.insert("miss".to_string(), PossibilitiesRecordParam::Miss(Some(miss.unwrap() as f64)));
        } else {
            panic!("Nested possibilities functions are not allowed.");
        }

        self.simulate(sequence.to_string(), false, None, true);
        self.record = HashMap::new();
    }

    /// Displays ticks where while `sequence` is run, the player's velocity on
    /// EACH axis is within `tolerance` of hitting inertia, or has hit inertia.
    ///
    /// Inertia is determined.
    pub fn inertialistener(
        &mut self,
        sequence: MothballSequence,
        tolerance: f32,
    ) {
        if self.record_inertia.is_empty() {
            self.record_inertia = HashMap::new();
            self.record_inertia.insert("type".to_string(), InertiaRecordParam::Axis("xz"));
            self.record_inertia.insert("tick".to_string(), InertiaRecordParam::Tick(1));
            self.record_inertia.insert("tolerance".to_string(), InertiaRecordParam::Tolerance(tolerance as f64));
        } else {
            panic!("Nested inertia listener functions are not allowed.");
        }

        self.simulate(sequence.to_string(), false, None, true);
        self.record_inertia = HashMap::new();
    }

    /// Displays ticks where while `sequence` is run, the player's x-velocity is
    /// below the maximum velocity (as determined by `max_vel`, default 0.01),
    /// useful to check when players are close to hitting inertia in a sequence
    /// of ticks.
    ///
    /// `max_vel` is set to 0.01 by default, but you can set it higher for
    /// different types of inertia (eg. ground inertia).
    pub fn xinertialistener(
        &mut self,
        sequence: MothballSequence,
        tolerance: f32,
    ) {

        if self.record_inertia.is_empty() {
            self.record_inertia = HashMap::new();
            self.record_inertia.insert("type".to_string(), InertiaRecordParam::Axis("x"));
            self.record_inertia.insert("tick".to_string(), InertiaRecordParam::Tick(1));
            self.record_inertia.insert("tolerance".to_string(), InertiaRecordParam::Tolerance(tolerance as f64));
        } else {
            panic!("Nested inertia listener functions are not allowed.");
        }

        self.simulate(sequence.to_string(), false, None, true);
        self.record_inertia = HashMap::new();
    }

    /// Displays ticks where while `sequence` is run, the player's z-velocity is
    /// below the maximum velocity (as determined by `max_vel`, default 0.01),
    /// useful to check when players are close to hitting inertia in a sequence
    /// of ticks.
    ///
    /// `max_vel` is set to 0.01 by default, but you can set it higher for
    /// different types of inertia (eg. ground inertia).
    pub fn zinertialistener(
        &mut self,
        sequence: MothballSequence,
        tolerance: f32,
    ) {

        if self.record.is_empty() {
            self.record_inertia = HashMap::new();
            self.record_inertia.insert("type".to_string(), InertiaRecordParam::Axis("z"));
            self.record_inertia.insert("tick".to_string(), InertiaRecordParam::Tick(1));
            self.record_inertia.insert("tolerance".to_string(), InertiaRecordParam::Tolerance(tolerance as f64));
        } else {
            panic!("Nested inertia listener functions are not allowed.");
        }

        self.simulate(sequence.to_string(), false, None, true);
        self.record_inertia = HashMap::new();
    }
}

impl PlayerSimulationXZ {
        /// Returns information regarding `x` by `z` block jump. Note that `x` and
        /// `z` are in terms of blocks, that is, assuming you jump from one corner
        /// of the block to the other corner.
        ///
        /// In the future, there may be different functions that allow `x` and `z`
        /// in terms of displacement or momentum used.
    pub fn dimensions(&mut self, x: f64, z: f64) {

        self.add_to_output(ExpressionType::Text, "Jump Dimension Info".to_string(), StringOrNum::Str("".to_string()), 0.0f64, true);

        self.add_to_output(
            ExpressionType::Text,
            format!("\tA {} × {} block jump is equivalent to", x, z),
            StringOrNum::Str("".to_string()), 
            0.0f64, 
            true
        );

        self.add_to_output(
            ExpressionType::Text,
            format!(
                "\t{} block jump.",
                self.truncate_number(math::dist_to_blockf64(
                    (
                        math::block_to_distf64(x).powi(2)
                            + math::block_to_distf64(z).powi(2)
                    )
                    .sqrt(),
                ))
            ),
            StringOrNum::Str("".to_string()), 
            0.0f64, 
            true
        );

        self.add_to_output(
            ExpressionType::Text,
            format!(
                "\tAngle: {}",
                self.truncate_number(((
                    math::block_to_distf64(z)
                ).atan2(math::block_to_distf64(x))).to_degrees())
            ),
            StringOrNum::Str("".to_string()), 
            0.0f64, 
            true
        );
    }

    pub fn taps(&mut self) { // seq_or_num: &[MothballSequence]
        panic!("not implemented")
//        let mut d = ambiguous;
//        let mut last_seq = String::new();
//        let mut after_num = false;
//
//        for i in seq_or_num {
//            if i.isnumeric() {
//                if after_num || ambiguous {
//                    panic!(
//                        "Numbers must follow after a sequence. {}.",
//                        if after_num {
//                            format!("{} comes after a number", i)
//                        } else {
//                            format!("{} has no sequence to follow", i)
//                        }
//                    );
//                }
//
//                d.insert(last_seq.clone(), i.parse::<i32>().unwrap());
//                after_num = true;
//            } else if self.isfloat(i) {
//                panic!("Number should be an integer, not a float ({})", i);
//            } else {
//                last_seq = i.clone();
//                d.insert(last_seq.clone(), 1);
//                after_num = false;
//            }
//        }
//
//        let had_sneak_delay = self.sneak_delay;
//        self.sneak_delay = false;
//
//        for (k, v) in d.iter() {
//            for _ in 0..*v {
//                self.simulate(k, false);
//
//                let mut modifier_list = Vec::new();
//
//                for (i, j) in Simulation::MODIFIERS
//                    .iter()
//                    .zip(["water", "lava", "web", "block", "ladder"].iter())
//                {
//                    if self.modifiers & i != 0 {
//                        modifier_list.push(*j);
//                    }
//                }
//
//                let modifiers = modifier_list.join(",");
//
//                while self.vx != 0.0 || self.vz != 0.0 {
//                    self.inputs.clear();
//                    self.stop();
//                }
//            }
//        }
//
//        if had_sneak_delay {
//            self.sneak_delay = true;
//        }
//
//        // self.call_stack.pop()
//    }
    }
}

impl PlayerSimulationXZ {
    /// Attempts to find the speed such that executing `sequence` results in using `zmm`
    /// blocks of momentum on the Z axis. A warning is raised if the simulation using
    /// the calculated speed doesn't match `zmm`, meaning that inertia was encountered
    /// while simulating.
    pub fn bwmm(&mut self, zmm: f64, sequence: &MothballSequence) {
        let vz = self.optimize_z(
            0.0f64,
            zmm,
            sequence,
            math::mm_to_distf64,
        );

        self.simulate(
            format!(
                " z(0) vz({}) outvz(label=\"Vz Needed\") {} zmm(label=\"Zmm Used\") ",
                vz, sequence.to_string()
            ),
            false,
            None,
            true
        );

        if (math::dist_to_mmf64(self.z) - zmm).abs() > 1e-5 {
            self.add_to_output(
                ExpressionType::Warning,
                "encountered inertia on Z while optimizing!".to_string(),
                StringOrNum::Str("".to_string()), 
                0.0f64, 
                true
            );
        }
    }

    /// Attempts to find the speed such that executing `sequence` results in a displacement
    /// of `z` on the Z axis. A warning is raised if the simulation using the calculated
    /// speed doesn't match `z`, meaning that inertia was encountered while simulating.
    pub fn wall(&mut self, z: f64, sequence: &MothballSequence) {
        let vz = self.optimize_z(0.0f64, z, sequence, |x| x);

        self.simulate(
            format!(
                " z(0) vz({}) outvz(label=\"Vz Needed\") {} outz(label=\"Z dist\")",
                vz, sequence.to_string()
            ),
            true,
            None,
            true
        );

        if (self.z - z).abs() > 1e-5 {
            self.add_to_output(
                ExpressionType::Warning,
                "encountered inertia on Z while optimizing!".to_string(),
                StringOrNum::Str("".to_string()), 
                0.0f64, 
                true
            );
        }
    }

    /// Attempts to find the speed such that executing `sequence` results in traversing
    /// `zb` blocks on the Z axis. A warning is raised if the simulation using the
    /// calculated speed doesn't match `zb`, meaning that inertia was encountered
    /// while simulating.
    pub fn blocks(&mut self, zb: f64, sequence: &MothballSequence) {
        let vz = self.optimize_z(
            0.0f64,
            zb,
            sequence,
            math::block_to_distf64,
        );

        self.simulate(
            format!(
                " z(0) vz({}) outvz(label=\"Vz Needed\") {} zb(label=\"Z blocks\")",
                vz, sequence.to_string()
            ),
            true,
            None,
            true
        );

        if (math::dist_to_blockf64(self.z) - zb).abs() > 1e-5 {
            self.add_to_output(
                ExpressionType::Warning,
                "encountered inertia on Z while optimizing!".to_string(),
                StringOrNum::Str("".to_string()), 
                0.0f64, true

            );
        }
    }

    /// Attempts to find the speed such that executing `sequence` results in using `xmm`
    /// blocks of momentum on the X axis. A warning is raised if the simulation using
    /// the calculated speed doesn't match `xmm`, meaning that inertia was encountered
    /// while simulating.
    pub fn xbwmm(&mut self, xmm: f64, sequence: &MothballSequence) {
        let vx = self.optimize_x(
            xmm,
            0.0f64,
            sequence,
            math::mm_to_distf64,
        );

        self.simulate(
            format!(
                " x(0) vx({}) outvx(label=\"Vx Needed\") {} xmm(label=\"Xmm Used\") ",
                vx, sequence.to_string()
            ),
            false,
            None,
            true
        );

        if (math::dist_to_mmf64(self.x) - xmm).abs() > 1e-5 {
            self.add_to_output(
                ExpressionType::Warning,
                "encountered inertia on X while optimizing!".to_string(),
                StringOrNum::Str("".to_string()), 
                0.0f64, 
                true
            );
        }
    }

    /// Attempts to find the speed such that executing `sequence` results in a displacement
    /// of `x` on the X axis. A warning is raised if the simulation using the calculated
    /// speed doesn't match `x`, meaning that inertia was encountered while simulating.
    pub fn xwall(&mut self, x: f64, sequence: &MothballSequence) {
        let vx = self.optimize_x(x, 0.0f64, sequence, |x| x);

        self.simulate(
            format!(
                " x(0) vx({}) outvx(label=\"Vx Needed\") {} outx(label=\"X dist\")",
                vx, sequence.to_string()
            ),
            true,
            None,
            true
        );

        if (self.x - x).abs() > 1e-5 {
            self.add_to_output(
                ExpressionType::Warning,
                "encountered inertia on X while optimizing!".to_string(),
                StringOrNum::Str("".to_string()), 
                0.0f64, 
                true
            );
        }
    }

    /// Attempts to find the speed such that executing `sequence` results in traversing
    /// `xb` blocks on the X axis. A warning is raised if the simulation using the
    /// calculated speed doesn't match `xb`, meaning that inertia was encountered
    /// while simulating.
    pub fn xblocks(&mut self, xb: f64, sequence: &MothballSequence) {
        let vx = self.optimize_x(
            xb,
            0.0f64,
            sequence,
            math::block_to_distf64,
        );

        self.simulate(
            format!(
                " x(0) vx({}) outvx(label=\"Vx Needed\") {} xb(label=\"X blocks\")",
                vx, sequence.to_string()
            ),
            true,
            None,
            true
        );

        if (math::dist_to_blockf64(self.x) - xb).abs() > 1e-5 {
            self.add_to_output(
                ExpressionType::Warning,
                "encountered inertia on X while optimizing!".to_string(),
                StringOrNum::Str("".to_string()), 
                0.0f64, 
                true
            );
        }
    }

    pub fn macro_(&mut self) {
        panic!("This is not implemented")
//        let formatting = formatting.unwrap_or("mpk").to_lowercase().trim().to_string();
//
//        if formatting == "mpk" {
//            let mut lines = vec![
//                "X,Y,Z,YAW,PITCH,ANGLE_X,ANGLE_Y,W,A,S,D,SPRINT,SNEAK,JUMP,LMB,RMB,VEL_X,VEL_Y,VEL_Z"
//                    .to_string(),
//            ];
//
//            for i in &self.history {
//                lines.push(format!(
//                    "0.0,0.0,0.0,0.0,0.0,{:.3},0.0,{},{},{},{},{},{},{},false,{},0.0,0.0,0.0",
//                    i.last_turn,
//                    if i.w { "true" } else { "false" },
//                    if i.a { "true" } else { "false" },
//                    if i.s { "true" } else { "false" },
//                    if i.d { "true" } else { "false" },
//                    if i.sprint { "true" } else { "false" },
//                    if i.sneak { "true" } else { "false" },
//                    if i.space { "true" } else { "false" },
//                    if i.right_click { "true" } else { "false" },
//                ));
//            }
//
//            ambiguous;
//            // self.macros.insert(format!("{}.csv", name), lines.join("\n"));
//        } else if formatting == "cyv" {
//            // CYV: WASD sprint sneak jump angle
//
//            let mut lines: Vec<Vec<String>> = Vec::new();
//
//            for i in &self.history {
//                lines.push(vec![
//                    if i.w { "true" } else { "false" }.to_string(),
//                    if i.a { "true" } else { "false" }.to_string(),
//                    if i.s { "true" } else { "false" }.to_string(),
//                    if i.d { "true" } else { "false" }.to_string(),
//                    if i.space { "true" } else { "false" }.to_string(),
//                    if i.sprint { "true" } else { "false" }.to_string(),
//                    if i.sneak { "true" } else { "false" }.to_string(),
//                    format!("{:.3}", i.last_turn),
//                    "0.0".to_string(),
//                ]);
//            }
//
//            ambiguous;
//            // self.macros.insert(format!("{}.json", name), lines);
//        } else {
//            panic!(
//                "No such formatting {}, options are either 'mpk' or 'cyv'.",
//                formatting
//            );
//        }
    }
}

impl PlayerSimulationXZ {
    fn show_default_output(&mut self) {
        self.add_to_output(ExpressionType::ZLabel, "Z".to_string(), StringOrNum::Num(self.z), 0.0f64, true);
        self.add_to_output(ExpressionType::ZLabel,"VZ".to_string(), StringOrNum::Num(self.vz), 0.0f64, true);
        self.add_to_output(ExpressionType::XLabel,"X".to_string(), StringOrNum::Num(self.x), 0.0f64, true);
        self.add_to_output(ExpressionType::XLabel,"VX".to_string(), StringOrNum::Num(self.vx), 0.0f64, true);
    }

    pub fn show_output(&self) -> String {
        let mut merged_strings = Vec::new();
        for tup in self.output.iter() {
            let strings: Vec<String> = match &tup.1 {
                OutputExpression::ZLabel(s1, s2, s3) => {
                    vec![s1.clone(), s2.to_string(), s3.clone()]
                }
                OutputExpression::ZLabelWithExpression(s1, s2, s3) => {
                    vec![s1.clone(), s2.to_string(), s3.clone()]
                }
                OutputExpression::XLabel(s1, s2, s3) => {
                    vec![s1.clone(), s2.to_string(), s3.clone()]
                }
                OutputExpression::XLabelWithExpression(s1, s2, s3) => {
                    vec![s1.clone(), s2.to_string(), s3.clone()]
                }
                OutputExpression::GeneralLabel(s1) => {
                    vec![s1.clone()]
                }
                OutputExpression::GeneralLabelWithNumber(s1, s2, s3) => {
                    vec![s1.clone(), s2.to_string(), s3.clone()]
                }
                OutputExpression::GeneralLabelWithExpression(s1, s2, s3, s4, s5) => {
                    vec![
                        s1.clone(),
                        s2.to_string(),
                        s3.clone(),
                        s4.to_string(),
                        s5.clone(),
                    ]
                }
                OutputExpression::Warning(s1, s2, s3) => {
                    vec![s1.to_string(), s2.to_string(), s3.clone()]
                }
                OutputExpression::Text(s1) => {
                    vec![s1.clone()]
                }
                OutputExpression::ZInertiaMiss(s1, s2, s3, s4, s5) => {
                    vec![
                        s1.clone(),
                        s2.clone(),
                        s3.to_string(),
                        s4.clone(),
                        s5.clone(),
                    ]
                }
                OutputExpression::ZInertiaHit(s1, s2, s3, s4, s5) => {
                    vec![
                        s1.clone(),
                        s2.clone(),
                        s3.to_string(),
                        s4.clone(),
                        s5.clone(),
                    ]
                }
                OutputExpression::XInertiaHit(s1, s2, s3, s4, s5) => {
                    vec![
                        s1.clone(),
                        s2.clone(),
                        s3.to_string(),
                        s4.clone(),
                        s5.clone(),
                    ]
                }
                OutputExpression::XInertiaMiss(s1, s2, s3, s4, s5) => {
                    vec![
                        s1.clone(),
                        s2.clone(),
                        s3.to_string(),
                        s4.clone(),
                        s5.clone(),
                    ]
                }
                OutputExpression::GeneralInertiaLabel(s1, s2, s3, s4, s5, s6) => {
                    vec![
                        s1.clone(),
                        s2.to_string(),
                        s3.clone(),
                        s4.to_string(),
                        s5.clone(),
                        s6.to_string(),
                    ]
                }
            };


            let ss = strings.join(" ");
            println!("{}", ss);
            merged_strings.push(ss);
        }

        return merged_strings.join("\n")
    }
}

impl PlayerSimulationXZ {
    pub fn run(&mut self, token: Tokenized) {
        /*
        Runs the token. `token` is a struct equivalent to

        {
            function,
            inputs,
            modifiers,
            args,
            kwargs,
        }
        */

        let func = token.function;
        self.inputs = token.inputs;
        self.modifiers = token.modifiers as i32;
        let args = token.args;
        let mut kwargs = token.kwargs;

        func.run_func(self, args, &mut kwargs);
    }

    /// return_defaults = True, locals: dict = None, suppress_exception: bool = True
    /// 
    /// true, None, true
    pub fn simulate(
        &mut self,
        sequence: String,
        return_defaults: bool,
        locals: Option<IndexMap<String, parser::Data>>,
        suppress_exception: bool,
    ) {
        /*
        Execute Mothball Code. If no output was made and
        `return_defaults == true`, return the default output
        (see `show_default_output()`). `locals` is a dict of
        values for variables.
        */

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let parsed_tokens = self.parse(sequence.as_str(), None, None);

            for token in parsed_tokens {
                if self.stop_flag {
                    panic!("{}", errors::RuntimeError::InterruptedError("Stopped execution".to_string()));
                }

                let runnable = self.tokenize(&token, locals.clone());

                if self.stop_flag {
                    panic!("{}", errors::RuntimeError::InterruptedError("Stopped execution".to_string()));
                }
                self.run(runnable);
            }

            if return_defaults && self.output.is_empty() {
                self.show_default_output();
            }
        }));

        if let Err(e) = result {
            if suppress_exception {
                self.add_to_output(
                    ExpressionType::Text,
                    "Some error happened".to_string(),
                    StringOrNum::Str("".to_string()), 
                    0.0f64, 
                    true
                );
            } else {
                std::panic::resume_unwind(e);
            }
        }
    }
}