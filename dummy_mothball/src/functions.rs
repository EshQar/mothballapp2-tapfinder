use indexmap::IndexMap;

use crate::{parser::{Data, DataType}, player::{MothballSequence, PlayerSimulationXZ}};

#[derive(Clone, Debug)]
pub enum FullArgumentValue {
    Float(f64),
    Int(i64),
    F32(f32),
    Bool(bool),
    Str(String),
    None,
}

pub fn empty_get_wrapped_f32(arg: Option<FullArgumentValue>) -> Option<f32> {
    match arg {
        Some(value) => value.empty_get_f32(),
        None => None
    }
}

pub fn empty_get_wrapped_i64(arg: Option<FullArgumentValue>) -> Option<i64> {
    match arg {
        Some(value) => value.empty_get_i64(),
        None => None
    }
}

pub fn empty_get_wrapped_i32(arg: Option<FullArgumentValue>) -> Option<i32> {
    match arg {
        Some(value) => value.empty_get_i32(),
        None => None
    }
}

pub fn empty_get_wrapped_bool(arg: Option<FullArgumentValue>) -> Option<bool> {
    match arg {
        Some(value) => value.empty_get_bool(),
        None => None
    }
}

pub fn empty_get_wrapped_str(arg: Option<FullArgumentValue>) -> Option<String> {
    match arg {
        Some(value) => value.empty_get_string(),
        None => None
    }
}

//pub fn empty_get_wrapped_sequence(arg: Option<FullArgumentValue>) -> Option<MothballSequence> {
//    match arg {
//        Some(value) => Some(MothballSequence(value.get_str().to_string())),
//        None => None
//    }
//}

pub fn empty_get_wrapped_f64(arg: Option<FullArgumentValue>) -> Option<f64> {
    match arg {
        Some(value) => Some(value.get_f64()),
        None => None
    }
}
pub fn get_wrapped_f32(arg: Option<FullArgumentValue>) -> Option<f32> {
    match arg {
        Some(value) => Some(value.get_f32()),
        None => None
    }
}

pub fn get_wrapped_i64(arg: Option<FullArgumentValue>) -> Option<i64> {
    match arg {
        Some(value) => Some(value.get_i64()),
        None => None
    }
}

pub fn get_wrapped_i32(arg: Option<FullArgumentValue>) -> Option<i32> {
    match arg {
        Some(value) => Some(value.get_i32()),
        None => None
    }
}

pub fn get_wrapped_bool(arg: Option<FullArgumentValue>) -> Option<bool> {
    match arg {
        Some(value) => Some(value.get_bool()),
        None => None
    }
}

pub fn get_wrapped_str(arg: Option<FullArgumentValue>) -> Option<String> {
    match arg {
        Some(value) => Some(value.get_str().to_string()),
        None => None
    }
}

pub fn get_wrapped_sequence(arg: Option<FullArgumentValue>) -> Option<MothballSequence> {
    match arg {
        Some(value) => Some(MothballSequence(value.get_str().to_string())),
        None => None
    }
}

impl FullArgumentValue {
    pub fn get_f64(&self) -> f64 {
        match self {
            Self::Float(val) => *val,
            Self::F32(val) => { println!("unexpected type but it was castable"); *val as f64 }
            Self::Int(_)
            |Self::Bool(_)
            |Self::Str(_)
            |Self::None => panic!()
        }
    }
    pub fn get_f32(&self) -> f32 {
        match self {
            Self::Float(val) => *val as f32,
            Self::F32(val) => { println!("unexpected type but it was castable"); *val }
            Self::Int(_)
            |Self::Bool(_)
            |Self::Str(_)
            |Self::None => panic!()
        }
    }

    pub fn get_i64(&self) -> i64 {
        match self {
            Self::Int(val) => *val,
            Self::Float(_)
            |Self::F32(_)
            |Self::Bool(_)
            |Self::Str(_)
            |Self::None => panic!()
        }
    }

    pub fn get_i32(&self) -> i32 {
        match self {
            Self::Int(val) => *val as i32,
            Self::Float(_)
            |Self::F32(_)
            |Self::Bool(_)
            |Self::Str(_)
            |Self::None => panic!()
        }
    }

    pub fn get_bool(&self) -> bool {
        match self {
            Self::Bool(val) => *val,
            Self::Int(_)
            |Self::F32(_)
            |Self::Float(_)
            |Self::Str(_)
            |Self::None => panic!()
        }
    }

    pub fn get_str(&self) -> &str {
        match self {
            Self::Str(val) => val,
            Self::Int(_)
            |Self::F32(_)
            |Self::Bool(_)
            |Self::Float(_)
            |Self::None => panic!()
        }
    }

    pub fn empty_get_f32(&self) -> Option<f32> {
        match self {
            Self::Float(val) => Some(*val as f32),
            Self::F32(val) => { println!("unexpected type but it was castable"); Some(*val) }
            Self::Int(_)
            |Self::Bool(_)
            |Self::Str(_) => panic!(),
            Self::None => None
        }
    }

    pub fn empty_get_i64(&self) -> Option<i64> {
        match self {
            Self::Int(val) => Some(*val),
            Self::Float(_)
            |Self::F32(_)
            |Self::Bool(_)
            |Self::Str(_) => panic!(),
            Self::None => None
        }
    }

    pub fn empty_get_i32(&self) -> Option<i32> {
        match self {
            Self::Int(val) => Some(*val as i32),
            Self::Float(_)
            |Self::F32(_)
            |Self::Bool(_)
            |Self::Str(_) => panic!(),
            Self::None => None
        }
    }

    pub fn empty_get_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(val) => Some(*val),
            Self::Int(_)
            |Self::F32(_)
            |Self::Float(_)
            |Self::Str(_) => panic!(),
            Self::None => None
        }
    }

    pub fn empty_get_str(&self) -> Option<&str> {
        match self {
            Self::Str(val) => Some(val),
            Self::Int(_)
            |Self::F32(_)
            |Self::Bool(_)
            |Self::Float(_) => panic!(),
            Self::None => None
        }
    }

    pub fn empty_get_string(&self) -> Option<String> {
        match self {
            Self::Str(val) => Some(val.to_string()),
            Self::Int(_)
            |Self::F32(_)
            |Self::Bool(_)
            |Self::Float(_) => panic!(),
            Self::None => None
        }
    }
}

#[derive(Clone, Debug)]
pub enum ArgumentValue {
    Empty(DataType),
    HasValue(DataType, FullArgumentValue),
}

impl ArgumentValue {
    pub fn datatype(&self) -> DataType {
        match self {
            ArgumentValue::Empty(data) => *data,
            ArgumentValue::HasValue(data, _) => *data
        }
    }

    pub fn get_full_value(&self) -> FullArgumentValue {
        match self {
            ArgumentValue::Empty(_) => panic!(),
            ArgumentValue::HasValue(_, value) => value.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub enum Argument {
    PositionalOnly(String, ArgumentValue, bool),
    PositionalOrKeyword(String, ArgumentValue, bool),
    KeywordOnly(String, ArgumentValue, bool),
    VarPositional(String, ArgumentValue, bool),
}

impl Argument {
    pub fn name(&self) -> &String {
        match self {
            Argument::PositionalOnly(name, _, _) => name,
            Argument::PositionalOrKeyword(name, _, _) => name,
            Argument::KeywordOnly(name, _, _) => name,
            Argument::VarPositional(name, _, _) => name,
        }
    }

    pub fn value(self) -> ArgumentValue {
        match self {
            Argument::PositionalOnly(_, value, _) => value,
            Argument::PositionalOrKeyword(_, value, _) => value,
            Argument::KeywordOnly(_, value, _) => value,
            Argument::VarPositional(_, value, _) => value,
        }
    }
}

pub fn argument_from_data(data: Data) -> FullArgumentValue {
    match data {
        Data::Float(value) => { FullArgumentValue::Float(value) },
        Data::Int(value) => { FullArgumentValue::Int(value) },
        Data::F32(value) => { FullArgumentValue::F32(value) },
        Data::Bool(value) => { FullArgumentValue::Bool(value) },
        Data::Str(value) => { FullArgumentValue::Str(value) },
    }
}

#[derive(Clone, Debug)]
pub struct FunctionData {
    pub id: i32,
    pub common_name: String,
    pub aliases: Vec<String>,
    pub arguments: Vec<Argument>,
}
    
impl FunctionData {
    pub fn new(id: i32, common_name: String, aliases: Vec<String>, arguments: Vec<Argument>) -> Self {
        Self { id, common_name, aliases, arguments }
    }
}

#[derive(Clone, Debug)]
pub enum Function {
    Walk(FunctionData),
    Sprint(FunctionData),
    WalkAir(FunctionData),
    SprintAir(FunctionData),
    WalkJump(FunctionData),
    SprintJump(FunctionData),
    Sneak(FunctionData),
    SneakAir(FunctionData),
    SneakJump(FunctionData),
    SneakSprint(FunctionData),
    SneakSprintAir(FunctionData),
    SneakSprintJump(FunctionData),
    SprintStrafeJump(FunctionData),
    Stop(FunctionData),
    StopAir(FunctionData),
    StopJump(FunctionData),
    SneakStop(FunctionData),
    SneakStopAir(FunctionData),
    SneakStopJump(FunctionData),
    WalkPessi(FunctionData),
    SprintPessi(FunctionData),
    ForceMomentum(FunctionData),

    Walk45(FunctionData),
    Sprint45(FunctionData),
    WalkAir45(FunctionData),
    SprintAir45(FunctionData),
    WalkJump45(FunctionData),
    SprintJump45(FunctionData),
    Sneak45(FunctionData),
    SneakAir45(FunctionData),
    SneakJump45(FunctionData),
    SprintStrafeJump45(FunctionData),
    SneakSprint45(FunctionData),
    SneakSprintAir45(FunctionData),
    SneakSprintJump45(FunctionData),
    WalkPessi45(FunctionData),
    SprintPessi45(FunctionData),
    ForceMomentum45(FunctionData),

    OutZ(FunctionData),
    Zmm(FunctionData),
    Zb(FunctionData),
    OutVz(FunctionData),
    OutX(FunctionData),
    Xmm(FunctionData),
    Xb(FunctionData),
    OutVx(FunctionData),
    Vec(FunctionData),

    OutAngle(FunctionData),
    OutTurn(FunctionData),
    EffectsMultiplier(FunctionData),
    AngleInfo(FunctionData),

    Face(FunctionData),
    Turn(FunctionData),

    SetPosZ(FunctionData),
    SetVz(FunctionData),
    SetPosX(FunctionData),
    SetVx(FunctionData),

    AddVx(FunctionData),
    AddVz(FunctionData),
    AddPosX(FunctionData),
    AddPosZ(FunctionData),

    SetSlip(FunctionData),
    Inertia(FunctionData),

    SprintAirDelay(FunctionData),
    SneakDelay(FunctionData),
    SingleAxisInertia(FunctionData),

    Version(FunctionData),
    Speed(FunctionData),
    Slowness(FunctionData),

    AngleQueue(FunctionData),
    TurnQueue(FunctionData),

    Possibilities(FunctionData),
    XPossibilities(FunctionData),
    XZPossibilities(FunctionData),

    Dimensions(FunctionData),

    Taps(FunctionData),

    Bwmm(FunctionData),
    XBwmm(FunctionData),

    Wall(FunctionData),
    XWall(FunctionData),

    Blocks(FunctionData),
    XBlocks(FunctionData),

    InertiaListener(FunctionData),
    XInertiaListener(FunctionData),
    ZInertiaListener(FunctionData),

    Macro(FunctionData),

    Print(FunctionData),
    Repeat(FunctionData),
    SetPrecision(FunctionData),
    BallHelp(FunctionData),
    Var(FunctionData),
}

impl Function {
    pub fn run_func(&self, player: &mut PlayerSimulationXZ, args: Vec<FullArgumentValue>, kwargs: &mut IndexMap<String, FullArgumentValue>) {
        println!("args: {:?}\n\nkwargs: {:?}", args, kwargs);
        let count_args = args.len();
        let mut reorganized_args: Vec<FullArgumentValue> = Vec::with_capacity(args.len() + kwargs.keys().len());
        let can_be_empty_by_default = vec!["rotation", "slip", "speed", "slow", "single_axis"];

        let function_args = self.data().arguments.iter();
        let passed_args = args.into_iter();
        println!("all func args {:?}\n\n", function_args.clone().collect::<Vec<_>>());
        for (function_argument, passed_argument) in function_args.zip(passed_args) {
            println!("func arg {:?}", function_argument);
            println!("passed arg {:?}", passed_argument);
            match function_argument {
                Argument::PositionalOnly(_, _, _) => { reorganized_args.push(passed_argument) }
                Argument::PositionalOrKeyword(_, _, _) => reorganized_args.push(passed_argument),
                Argument::KeywordOnly(_, _, _) =>  panic!(),
                Argument::VarPositional(_, _, _) => reorganized_args.push(passed_argument),
            }
        }

        let functions_args_excluding_positionals = self.data().arguments.iter().skip(count_args);
        println!("args {:?}", reorganized_args);
        println!("all func args {:?}\n\n", functions_args_excluding_positionals.clone().collect::<Vec<_>>());

        for missing_function_arg in functions_args_excluding_positionals {
            println!("missing {:?}", missing_function_arg);
            let next_arg = kwargs.get(missing_function_arg.name());
            match next_arg {
                Some(arg) => reorganized_args.push(arg.clone()),
                None => {
                    let arg_name = missing_function_arg.name();
                    if can_be_empty_by_default.contains(&arg_name.as_str()) {
                        reorganized_args.push(FullArgumentValue::None)
                    } else {
                        reorganized_args.push(missing_function_arg.clone().value().get_full_value())
                    }
                }
            }
 //           reorganized_args.push(kwargs.swap_remove(missing_function_arg.name()).unwrap())
        }
        println!("args {:?}", reorganized_args);
        let mut arguments = reorganized_args.into_iter();
        match self {
            Self::Walk(_) => player.walk(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::Sprint(_) => player.sprint(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::WalkAir(_) => player.walkair(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::SprintAir(_) => player.sprintair(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::WalkJump(_) => player.walkjump(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::SprintJump(_) => player.sprintjump(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::Sneak(_) => player.sneak(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::SneakAir(_) => player.sneakair(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::SneakJump(_) => player.sneakjump(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::SneakSprint(_) => player.sneaksprint(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::SneakSprintAir(_) => player.sneaksprintair(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::SneakSprintJump(_) => player.sneaksprintjump(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::SprintStrafeJump(_) => player.sprintstrafejump(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::Stop(_) => player.stop(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::StopAir(_) => player.stopair(arguments.next().unwrap().get_i32()),
            Self::StopJump(_) => player.stopjump(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::SneakStop(_) => player.sneakstop(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::SneakStopAir(_) => player.sneakstopair(arguments.next().unwrap().get_i32()),
            Self::SneakStopJump(_) => player.sneakstopjump(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::WalkPessi(_) => player.walkpessi(arguments.next().unwrap().get_i32(), arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next())),
            Self::SprintPessi(_) => player.sprintpessi(arguments.next().unwrap().get_i32(), arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next())),
            Self::ForceMomentum(_) => player.forcemomentum(arguments.next().unwrap().get_i32(), arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),

            Self::Walk45(_) => player.walk45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::Sprint45(_) => player.sprint45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::WalkAir45(_) => player.walkair45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::SprintAir45(_) => player.sprintair45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::WalkJump45(_) => player.walkjump45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::SprintJump45(_) => player.sprintjump45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::Sneak45(_) => player.sneak45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::SneakAir45(_) => player.sneakair45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::SneakJump45(_) => player.sneakjump45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::SprintStrafeJump45(_) => player.sprintstrafejump45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::SneakSprint45(_) => player.sneaksprint45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::SneakSprintAir45(_) => player.sneaksprintair45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next())),
            Self::SneakSprintJump45(_) => player.sneaksprintjump45(arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),
            Self::WalkPessi45(_) => player.walkpessi45(arguments.next().unwrap().get_i32(), arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next())),
            Self::SprintPessi45(_) => player.sprintpessi45(arguments.next().unwrap().get_i32(), arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next())),
            Self::ForceMomentum45(_) => player.forcemomentum45(arguments.next().unwrap().get_i32(), arguments.next().unwrap().get_i32(), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_f32(arguments.next()), empty_get_wrapped_i32(arguments.next()), empty_get_wrapped_i32(arguments.next())),

            Self::OutZ(_) => player.outz( arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_str()),
            Self::Zmm(_) => player.zmm( arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_str()),
            Self::Zb(_) => player.zb( arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_str()),
            Self::OutVz(_) => player.outvz( arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_str()),
            Self::OutX(_) => player.outx( arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_str()),
            Self::Xmm(_) => player.xmm( arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_str()),
            Self::Xb(_) => player.xb( arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_str()),
            Self::OutVx(_) => player.outvx( arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_str()),
            Self::Vec(_) => player.vec(),

            Self::OutAngle(_) => player.outangle( arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_str()),
            Self::OutTurn(_) => player.outturn( arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_str()),
            Self::EffectsMultiplier(_) => player.effectsmultiplier(get_wrapped_i32(arguments.next()), get_wrapped_i32(arguments.next())),
            Self::AngleInfo(_) => player.angleinfo(arguments.next().unwrap().get_f32()),

            Self::Face(_) => player.face(arguments.next().unwrap().get_f32()),
            Self::Turn(_) => player.turn(arguments.next().unwrap().get_f32()),

            Self::SetPosZ(_) => player.setposz(arguments.next().unwrap().get_f64()),
            Self::SetVz(_) => player.setvz(arguments.next().unwrap().get_f64()),
            Self::SetPosX(_) => player.setposx(arguments.next().unwrap().get_f64()),
            Self::SetVx(_) => player.setvx(arguments.next().unwrap().get_f64()),

            Self::AddVx(_) => player.addvx(arguments.next().unwrap().get_f64()),
            Self::AddVz(_) => player.addvz(arguments.next().unwrap().get_f64()),
            Self::AddPosX(_) => player.addposx(arguments.next().unwrap().get_f64()),
            Self::AddPosZ(_) => player.addposz(arguments.next().unwrap().get_f64()),

            Self::SetSlip(_) => player.setslip(arguments.next().unwrap().get_f32()),
            Self::Inertia(_) => player.inertia(arguments.next().unwrap().get_f32(), get_wrapped_bool(arguments.next())),

            Self::SprintAirDelay(_) => player.sprintairdelay(arguments.next().unwrap().get_bool()),
            Self::SneakDelay(_) => player.sneakdelay(arguments.next().unwrap().get_bool()),
            Self::SingleAxisInertia(_) => player.singleaxisinertia(arguments.next().unwrap().get_bool()),

            Self::Version(_) => player.version(arguments.next().unwrap().get_str()),
            Self::Speed(_) => player.speed(arguments.next().unwrap().get_i32()),
            Self::Slowness(_) => player.slowness(arguments.next().unwrap().get_i32()),

            Self::AngleQueue(_) => player.anglequeue(arguments.map(|arg| arg.get_f32()).collect::<Vec<_>>().to_vec()),
            Self::TurnQueue(_) => player.turnqueue(arguments.map(|arg| arg.get_f32()).collect::<Vec<_>>().to_vec()),

            Self::Possibilities(_) => player.possibilities(MothballSequence(arguments.next().unwrap().get_str().to_string()), arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_f32(), get_wrapped_f32(arguments.next())),
            Self::XPossibilities(_) => player.xpossibilities(MothballSequence(arguments.next().unwrap().get_str().to_string()), arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_f32(), get_wrapped_f32(arguments.next())),
            Self::XZPossibilities(_) => player.xzpossibilities(MothballSequence(arguments.next().unwrap().get_str().to_string()), arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_f32(), arguments.next().unwrap().get_f32(), get_wrapped_f32(arguments.next())),

            Self::Dimensions(_) => player.dimensions(arguments.next().unwrap().get_f64(), arguments.next().unwrap().get_f64()),

            Self::Taps(_) => player.taps(),

            Self::Bwmm(_) => player.bwmm(arguments.next().unwrap().get_f64(), &MothballSequence(arguments.next().unwrap().get_str().to_string())),
            Self::XBwmm(_) => player.xbwmm(arguments.next().unwrap().get_f64(), &MothballSequence(arguments.next().unwrap().get_str().to_string())),

            Self::Wall(_) => player.wall(arguments.next().unwrap().get_f64(), &MothballSequence(arguments.next().unwrap().get_str().to_string())),
            Self::XWall(_) => player.xwall(arguments.next().unwrap().get_f64(), &MothballSequence(arguments.next().unwrap().get_str().to_string())),

            Self::Blocks(_) => player.blocks(arguments.next().unwrap().get_f64(), &MothballSequence(arguments.next().unwrap().get_str().to_string())),
            Self::XBlocks(_) => player.xblocks(arguments.next().unwrap().get_f64(), &MothballSequence(arguments.next().unwrap().get_str().to_string())),

            Self::InertiaListener(_) => player.inertialistener(MothballSequence(arguments.next().unwrap().get_str().to_string()), arguments.next().unwrap().get_f32()),
            Self::XInertiaListener(_) => player.xinertialistener(MothballSequence(arguments.next().unwrap().get_str().to_string()), arguments.next().unwrap().get_f32()),
            Self::ZInertiaListener(_) => player.zinertialistener(MothballSequence(arguments.next().unwrap().get_str().to_string()), arguments.next().unwrap().get_f32()),

            Self::Macro(_) => player.macro_(),

            Self::Print(_) => player.print(arguments.next().unwrap().get_str().to_string()),
            Self::Repeat(_) => player.repeat(MothballSequence(arguments.next().unwrap().get_str().to_string()), arguments.next().unwrap().get_i32()),
            Self::SetPrecision(_) => player.setprecision(arguments.next().unwrap().get_i32()),
            Self::BallHelp(_) => { player.ballhelp(MothballSequence(arguments.next().unwrap().get_str().to_string())); },
            Self::Var(_) => player.var(arguments.next().unwrap().get_str().to_string(), get_wrapped_sequence(arguments.next())),


        }
    }

    fn data(&self) -> &FunctionData {
        match self {
            Self::Walk(data)
            | Self::Sprint(data)
            | Self::WalkAir(data)
            | Self::SprintAir(data)
            | Self::WalkJump(data)
            | Self::SprintJump(data)
            | Self::Sneak(data)
            | Self::SneakAir(data)
            | Self::SneakJump(data)
            | Self::SneakSprint(data)
            | Self::SneakSprintAir(data)
            | Self::SneakSprintJump(data)
            | Self::SprintStrafeJump(data)
            | Self::Stop(data)
            | Self::StopAir(data)
            | Self::StopJump(data)
            | Self::SneakStop(data)
            | Self::SneakStopAir(data)
            | Self::SneakStopJump(data)
            | Self::WalkPessi(data)
            | Self::SprintPessi(data)
            | Self::ForceMomentum(data)

            | Self::Walk45(data)
            | Self::Sprint45(data)
            | Self::WalkAir45(data)
            | Self::SprintAir45(data)
            | Self::WalkJump45(data)
            | Self::SprintJump45(data)
            | Self::Sneak45(data)
            | Self::SneakAir45(data)
            | Self::SneakJump45(data)
            | Self::SprintStrafeJump45(data)
            | Self::SneakSprint45(data)
            | Self::SneakSprintAir45(data)
            | Self::SneakSprintJump45(data)
            | Self::WalkPessi45(data)
            | Self::SprintPessi45(data)
            | Self::ForceMomentum45(data)

            | Self::OutZ(data)
            | Self::Zmm(data)
            | Self::Zb(data)
            | Self::OutVz(data)
            | Self::OutX(data)
            | Self::Xmm(data)
            | Self::Xb(data)
            | Self::OutVx(data)
            | Self::Vec(data)

            | Self::OutAngle(data)
            | Self::OutTurn(data)
            | Self::EffectsMultiplier(data)
            | Self::AngleInfo(data)

            | Self::Face(data)
            | Self::Turn(data)

            | Self::SetPosZ(data)
            | Self::SetVz(data)
            | Self::SetPosX(data)
            | Self::SetVx(data)

            | Self::AddVx(data)
            | Self::AddVz(data)
            | Self::AddPosX(data)
            | Self::AddPosZ(data)

            | Self::SetSlip(data)
            | Self::Inertia(data)

            | Self::SprintAirDelay(data)
            | Self::SneakDelay(data)
            | Self::SingleAxisInertia(data)

            | Self::Version(data)
            | Self::Speed(data)
            | Self::Slowness(data)

            | Self::AngleQueue(data)
            | Self::TurnQueue(data)

            | Self::Possibilities(data)
            | Self::XPossibilities(data)
            | Self::XZPossibilities(data)

            | Self::Dimensions(data)

            | Self::Taps(data)

            | Self::Bwmm(data)
            | Self::XBwmm(data)

            | Self::Wall(data)
            | Self::XWall(data)

            | Self::Blocks(data)
            | Self::XBlocks(data)

            | Self::InertiaListener(data)
            | Self::XInertiaListener(data)
            | Self::ZInertiaListener(data)

            | Self::Macro(data)

            | Self::Print(data)
            | Self::Repeat(data)
            | Self::SetPrecision(data)
            | Self::BallHelp(data)
            | Self::Var(data) => data,
        }
    }

    pub fn id(&self) -> i32 {
        self.data().id
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.data().arguments
    }

    pub fn aliases(&self) -> &Vec<String> {
        &self.data().aliases
    }

    pub fn name(&self, ) -> &str {
        &self.data().common_name
    }
}