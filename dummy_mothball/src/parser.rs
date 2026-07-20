use crate::expr_eval;
use crate::functions::Argument::PositionalOnly;
use crate::player::{PlayerSimulationXZ, Simulation};
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::ops::Index;
use crate::counter;
use crate::errors;
use crate::sort;
use regex::Regex;
use crate::functions::{self, ArgumentValue, FullArgumentValue, argument_from_data};
use indexmap::IndexMap;
use pyo3::prelude::*;


#[derive(Debug, Clone)]
pub enum Data {
    Float(f64),
    Int(i64),
    F32(f32),
    Bool(bool),
    Str(String),
}

impl<'a, 'py> FromPyObject<'a, 'py> for Data {
    type Error = PyErr;
    fn extract(obj: pyo3::Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        // Order matters: bool is also an int in Python
        if let Ok(v) = obj.extract::<bool>() {
            return Ok(Data::Bool(v));
        }

        if let Ok(v) = obj.extract::<i64>() {
            return Ok(Data::Int(v));
        }

        if let Ok(v) = obj.extract::<f64>() {
            return Ok(Data::Float(v));
        }

        if let Ok(v) = obj.extract::<String>() {
            return Ok(Data::Str(v));
        }

        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected bool, int, float, or str",
        ))
    }
}

impl Data {
    pub fn get_as_int(&self) -> &i64 {
        match self {
            Data::Float(_) => { panic!() }
            Data::Int(int) => { int }
            Data::F32(_) => { panic!() }
            Data::Bool(_) => { panic!() }
            Data::Str(_) => { panic!() }
        }
    }

    pub fn get_as_f64(&self) -> f64 {
        match self {
            Data::Float(float) => { *float }
            Data::Int(int) => { *int as f64 }
            Data::F32(float) => { *float as f64 }
            Data::Bool(_) => { panic!() }
            Data::Str(_) => { panic!() }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DataType {
    Float,
    Int,
    F32,
    Bool,
    Str,
}

impl DataType {
    pub fn cast_f64(&self, value: f64) -> Data {
        match self {
            DataType::Float => {
                Data::Float(value)
            }

            DataType::Int => {
                Data::Int(value as i64)
            }

            DataType::F32 => {
                Data::F32(value as f32)
            }

            DataType::Bool => {
                if value == 0.0f64 {
                    Data::Bool(false)
                } else {
                    Data::Bool(true)
                }
            }

            DataType::Str => {
                panic!("Cant do that!");
            }
        }
    }
}

pub fn safe_eval(
    local_vars: IndexMap<String, Data>,
    expr: &str,
    datatype: DataType,
    locals_dict: IndexMap<String, Data>,
) -> Result<Data, errors::RuntimeError> {
    /*
        Evaluate and convert expr to datatype.
        If datatype = str, returns expr as normal.
    */

    if matches!(datatype, DataType::Float)
        || matches!(datatype, DataType::Int)
        || matches!(datatype, DataType::F32)
        || matches!(datatype, DataType::Bool)
    {
        if matches!(datatype, DataType::Bool) {
            if expr.trim().to_lowercase() == "true" {
                return Ok(Data::Bool(true));
            } else if expr.trim().to_lowercase() == "false" {
                return Ok(Data::Bool(true));
            } else {
                panic!("idk what to do here");
            }
        }

        let result = expr_eval::evaluate(expr, locals_dict);

        let converted_value = datatype.cast_f64(result);

        return Ok(converted_value);
    }

    else if matches!(datatype, DataType::Str) {
        let mut result = Vec::<String>::new();

        let mut in_string = false;
        let mut follows_slash = false;


        for char in expr.chars() {
            if char == '\\' {
                follows_slash = true;
                continue;
            }


            if char == '"' && !follows_slash {
                in_string = !in_string;
            }

            else if (char == '{' || char == '}') && follows_slash {
                result.push("\\".to_string());
                result.push(char.to_string());
            }

            else if char == 'n' && follows_slash {
                result.push("\n".to_string());
            }

            else if in_string {
                result.push(char.to_string());
            }

            else if !in_string && !char.is_whitespace() {
                panic!(
                    "Invalid string: {}\n\
                    Strings must be delimited by double quotes, e.g. \"hello\".\n\
                    To insert values, place them inside curly brackets {{}}.",
                    expr
                );
            }


            follows_slash = false;
        }

        return Ok(Data::Str(formatted(local_vars, &result.join(""))));
    }

    else {
        Ok(Data::Str(expr.to_string()))
    }
}

pub fn formatted(local_vars: IndexMap<String, Data>, string: &str) -> String {
    /*
        Formats string just like an f-string.
    */

    let mut formatted_string = String::new();

    let mut item_to_eval = String::new();
    let mut in_expr = false;
    let mut depth: i32 = 0;
    let mut follows_slash = false;

    for char in string.chars() {
        if char == '\\' {
            follows_slash = true;
            continue;
        }

        if char == '{' && !follows_slash {
            in_expr = !in_expr;

            if !in_expr {
                item_to_eval.clear();
                formatted_string.push(char);
            } else {
                item_to_eval.push(char);
            }

            depth += 1;
        }

        else if char == '}' && !follows_slash {
            if depth == 0 {
                panic!("Unmatched Brackets");
            }

            depth -= 1;

            if in_expr {
                item_to_eval.push(char);

                // Equivalent to Python:
                // item_to_eval = item_to_eval[1:len(item_to_eval)-1]
                item_to_eval =
                    item_to_eval[1..item_to_eval.len() - 1].to_string();

                if !item_to_eval.is_empty() {
                    let x = expr_eval::evaluate(
                        &item_to_eval,
                        local_vars.clone(),
                    );

                    formatted_string.push_str(&x.to_string());
                }

                item_to_eval.clear();
            }

            else {
                formatted_string.push_str(&item_to_eval);
                formatted_string.push(char);

                item_to_eval.clear();
            }

            in_expr = !in_expr;
        }

        else if in_expr {
            item_to_eval.push(char);
        }

        else {
            formatted_string.push(char);
        }

        follows_slash = false;
    }

    if depth != 0 {
        panic!("Unmatched Brackets");
    }

    formatted_string
}

pub fn get_suggestions(alias_to_id_map: &HashMap<String, i32>, string: &str) -> Vec<String> {
    // Return a list of suggestions from all available mothball commands that best matches `string`.
    // For example, if `sprintsn` was inputted, a possible suggestion is `sneaksprint`.

    let mut matches_start: Vec<String> = Vec::new();
    let mut matches_part: Vec<String> = Vec::new(); // If string in word
    let mut matches_char_count: HashMap<String, i32> = HashMap::new();

    for command in alias_to_id_map.keys() {
        if command.starts_with(string) {
            // 1. Matches start
            matches_start.push(command.clone());
        } else if command.contains(string) {
            // 2. Matches part
            matches_part.push(command.clone());
        } else {
            // 3. Count chars
            let cmd_count = counter::Counter::from_iter(command.chars());
            let str_count = counter::Counter::from_iter(string.chars());

            let mut off_by = 0;

            for (char, count) in str_count.iter() {
                let cmd_char_count = cmd_count.get(char) as i32; 
                if cmd_count.keys().collect::<Vec<_>>().contains(&char) {
                    if *count as i32 == cmd_char_count {
                        off_by -= 1;
                    } else {
                        off_by += *count as i32 - cmd_char_count;
                    }
                } else {
                    // not a character
                    off_by += 1;
                }
            }

            off_by += (command.len() as i32 - string.len() as i32).abs();

            if off_by < command.len() as i32 {
                matches_char_count.insert(command.clone(), off_by);
            }
        }
    }

    let mut matches_char_count: Vec<String> = 
    sort::sorted_by_key(matches_char_count.into_iter().collect::<Vec<(String, i32)>>().into_iter(),
                         |(_, value)| *value,
                        false)
        .into_iter()
        .map(|(key, _)| key)
        .collect();

    matches_start.append(&mut matches_part);
    matches_start.append(&mut matches_char_count);

    matches_start
}

pub fn remove_comments_and_check_strings(string: &str) -> Result<String, errors::RuntimeError> {
    // Removes comments delimited by `#`

    let mut result: Vec<char> = Vec::new();
    let mut in_comment = false;
    let mut follows_slash = false;
    let mut in_string = false;

    for char in string.chars() {
        if char == '"' && !follows_slash {
            in_string = !in_string;
        }

        if char == '#' && !follows_slash && !in_string {
            in_comment = !in_comment;
            continue;
        }

        if !in_comment {
            result.push(char);
        }

        if char == '\\' && !follows_slash {
            follows_slash = true;
        } else {
            follows_slash = false;
        }
    }

    if in_string {
        return Err(errors::RuntimeError::SyntaxError("Unmatched quotes (\")".to_string()));
    }

    Ok(result.into_iter().collect())
}

pub fn parse(
    call_stack: Vec<String>,
    string: &str,
    splitters: Vec<char>,
    strict_whitespace: bool,
) -> Result<Vec<String>, errors::RuntimeError> {
    /*
    Splits the string at any of the splitters that are outside of parenthesis.
    Returns the first layer list of strings (or tokens), raises `SyntaxError`
    if there are missing spaces, or parenthesis are unmatched.

    Comments are delimited by the `#` symbol. Anything between comments will be ignored.

    `repeat(sprintjump(12), 3) sprint(2) outz(16)` parses into
    `[repeat(sprintjump(12), 3), sprint(2), outz(16)]`
    */

    let mut result: Vec<String> = Vec::new();
    let mut token = String::new();
    let mut stack: Vec<char> = Vec::new(); // parenthesis
    let mut current: usize = 0;
    let mut string = remove_comments_and_check_strings(string)?;
    let high = string.len();
    let mut expecting_whitespace = false;

    let matches_next_element = |curr_stack: &mut Vec<char>, e: char| {
        (e == ')' && curr_stack[curr_stack.len() - 1] == '(')
            || (e == ']' && curr_stack[curr_stack.len() - 1] == '[')
    };

    let mut follows_slash = false;
    let mut in_string = false;

    // Regex to change '||' into 'x(0) z(0) vx(0) vz(0)'
    let replace_double_bar_regex = Regex::new(r"(\|\|)").unwrap();
    string = replace_double_bar_regex
        .replace_all(&string, " x(0) z(0) vx(0) vz(0) ")
        .to_string();

    // Regex to change '|' into 'x(0) z(0)'
    let replace_bar_regex = Regex::new(r"(\|)").unwrap();
    string = replace_bar_regex
        .replace_all(&string, " x(0) z(0) ")
        .to_string();

    let mut chars: Vec<char> = string.chars().collect();
    chars.push(splitters[0]);

    for char in chars {
        if strict_whitespace {
            if expecting_whitespace && !char.is_whitespace() {
                if char == ')' || char == ']' {
                    return Err(errors::RuntimeError::SyntaxError(format!(
                        "Unmatched brackets at character {}: {}",
                        current,
                        &string[std::cmp::max(0, current.saturating_sub(5))
                            ..std::cmp::min(high, current + 5)]
                    )));
                } else {
                    let mut msg = format!("Space needed at character {}", current);

                    if !call_stack.is_empty() {
                        msg += &format!(" (inside {})", call_stack.join(", "));
                    }

                    msg += &format!(
                        ": {}",
                        &string[std::cmp::max(0, current.saturating_sub(7))
                            ..std::cmp::min(high, current + 7)]
                    );

                    return Err(errors::RuntimeError::SyntaxError(msg));
                }
            } else {
                expecting_whitespace = false;
            }
        }

        if char == '\\' {
            follows_slash = true;
            token.push(char);
            continue;
        } else if char == '"' && !follows_slash {
            in_string = !in_string;
        } else if (char == '(' || char == '[') && !follows_slash && !in_string {
            stack.push(char);
        } else if (char == ')' || char == ']') && !follows_slash && !in_string {
            if stack.is_empty() {
                return Err(errors::RuntimeError::SyntaxError(format!(
                    "Unmatched brackets at character {}: {}",
                    current,
                    &string[std::cmp::max(0, current.saturating_sub(5))
                        ..std::cmp::min(high, current + 5)]
                )));
            }

            if !matches_next_element(&mut stack, char) {
                return Err(errors::RuntimeError::SyntaxError(format!(
                    "Unmatched brackets at character {}: {}",
                    current,
                    &string[std::cmp::max(0, current.saturating_sub(5))
                        ..std::cmp::min(high, current + 5)]
                )));
            }

            stack.pop();

            if stack.is_empty() {
                token.push(char);
                follows_slash = false;

                if char == ')' {
                    expecting_whitespace = true;
                }

                current += 1;
                continue;
            }
        }

        let is_splitter = splitters.contains(&char);

        if is_splitter && stack.is_empty() && !follows_slash && !in_string {
            token = token.trim().to_string();

            if !token.is_empty() {
                result.push(token.clone());
            }

            token.clear();
        } else {
            token.push(char);
            current += 1;
        }

        follows_slash = false;
        expecting_whitespace = false;
    }

    if !stack.is_empty() {
        return Err(errors::RuntimeError::SyntaxError("Unmatched open parethesis".to_string()));
    }

    Ok(result)
}

#[derive(Clone, Debug)]
pub struct Tokenized {
    pub function: functions::Function,
    pub inputs: String,
    pub modifiers: u32,
    pub args: Vec<functions::FullArgumentValue>,
    pub kwargs: IndexMap<String, functions::FullArgumentValue>
}

pub fn tokenize(
    player: &mut PlayerSimulationXZ,
    string: &str,
    locals: Option<IndexMap<String, Data>>,
) -> Result<Tokenized, errors::RuntimeError> {
    /*
    Tokenizes the string to a dictionary containing the function,
    positional arguments, and keyword arguments of appropriate types.

    Returns in the form

    {
        "function": function,
        "inputs": str,
        "modifiers": int,
        "args": list,
        "kwargs": dict
    }

    `inputs` is a 1-2 char string determining how key presses determine
    game movement, examples `wa` or `s`.

    `modifiers` is an int which uses bit flags to indicate which modifiers
    are on or off. See the attribute `MODIFIERS`

    Raises `SyntaxError` if a positional argument follows a keyword argument.
    Raises `NameError` if a given function doesn't exist.
    Raises `TypeError` if these functions: (`stop`-related, 45 movement,
    or non-movement functions) receives an input.
    Raises any other error encountered while converting datatypes.
    */

    // tokenize_regex = r'(\W)?([^.\(\-)]+)(?:\.([^\(\.]+))?(?:\[(.*)\])?(?:\((.*)\))?(.+)?'

    let e1 = r"(\W)?";
    let func = r"([^.\[\(\-\)\]]+)";
    let inputs = r"(?:\.([wasdWASD]+))?";
    let modifiers = r"(?:\[(.*)\])?";
    let args = r"(?:\((.*)\))?";
    let e2 = r"(.+)?";

    let tokenize_regex = format!(
        "{}{}{}{}{}{}",
        e1, func, inputs, modifiers, args, e2
    );

    let regex = Regex::new(&tokenize_regex).unwrap();
    let captures = regex.captures(string).unwrap();

    let error1 = captures.get(1).map_or("", |m| m.as_str());
    let func_name = captures.get(2).map_or("", |m| m.as_str());
    let mut inputs = captures.get(3).map_or("", |m| m.as_str()).to_string();
    let modifiers = captures.get(4).map_or("", |m| m.as_str());
    let args = captures.get(5).map_or("", |m| m.as_str());
    let error2 = captures.get(6).map_or("", |m| m.as_str());

    if !error1.is_empty() && error1 != "-" {
        return Err(errors::RuntimeError::SyntaxError(format!(
            "Unknown item {} in {}",
            error1, string
        )));
    } else if !error2.is_empty() {
        return Err(errors::RuntimeError::SyntaxError(format!(
            "Unknown item {} in {}",
            error2, string
        )));
    }

    if string.chars().next() == Some('-') {
        player.reverse = true;
    } else {
        player.reverse = false;
    }

    let func = player.maybe_get_function(func_name);

    if func.is_none() {
        let mut error_msg = if !player.call_stack.is_empty() {
            format!("In {} -> ", player.call_stack.join(", "))
        } else {
            String::new()
        };

        error_msg += &format!("{} is not a valid function. ", func_name);

        let mut suggestions = get_suggestions(&player.alias_to_id_map, func_name);

        if !suggestions.is_empty() {
            suggestions = suggestions[0..std::cmp::min(4, suggestions.len())].to_vec();
            error_msg += &format!("Did you mean {}?", suggestions.join(", "));
        }

        return Err(errors::RuntimeError::NameError(error_msg));
    }

    let func = func.unwrap();

    let mut positional_args: Vec<String> = Vec::new();
    let mut keyword_args: IndexMap<String, String> = IndexMap::new();

    let args = parse(player.call_stack.clone(), args, vec![','], false)?;

    let keyword_regex = Regex::new(r"^\s*?(\w+)\s*=\s*(.+)\s*$").unwrap();
    let mut after_keyword = false;

    for arg in args {
        let result = keyword_regex.captures(&arg);

        if let Some(result) = result {
            // keyword
            let key = result.get(1).unwrap().as_str();
            let value = result.get(2).unwrap().as_str();

            if keyword_args.contains_key(key) {
                return Err(errors::RuntimeError::SyntaxError(format!(
                    "Repeated keyword argument {}",
                    key
                )));
            }

            keyword_args.insert(key.trim().to_string(), value.trim().to_string());
            after_keyword = true;
        } else {
            // positional
            if after_keyword {
                return Err(errors::RuntimeError::SyntaxError(
                    "Positional argument cannot follow keyword arguments".to_string(),
                ));
            }

            positional_args.push(arg);
        }
    }

    let (positional_args, keyword_args) =
        check_types(player, func.clone(), positional_args, keyword_args, locals)?;

    let modifiers = if !modifiers.is_empty() {
        validate_modifiers(
            modifiers
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        )?
    } else {
        0
    };

    if !Simulation::CAN_HAVE_INPUT.contains(&func.name())
    {
        if !inputs.is_empty() {
            return Err(errors::RuntimeError::TypeError(format!(
                "{}() cannot be modified by an input",
                func.name()
            )));
        }

        if Simulation::FORTYFIVE_METHODS.contains(&func.name())
        {
            inputs = "w".to_string();
        }
    } else if inputs.is_empty() {
        inputs = "w".to_string();
    } else if ![
        "w", "wa", "wd", "s", "sa", "sd", "a", "d",
    ]
    .contains(&inputs.as_str())
    {
        return Err(errors::RuntimeError::ValueError(format!(
            "function {} received bad input '{}', it can only be w, s, a, d, wa, wd, sa, wd.",
            func_name,
            inputs
        )));
    }

    if !Simulation::CAN_HAVE_MODIFIERS.contains(&func.name()) && modifiers != 0 {
        return Err(errors::RuntimeError::TypeError(format!(
            "{}() cannot be modified by a modifier",
            func.name()
        )));
    }

    Ok(Tokenized {
        function: func.clone(),
        inputs,
        modifiers,
        args: positional_args,
        kwargs: keyword_args,
    })
}

pub fn validate_modifiers(
    modifiers: Vec<String>,
) -> Result<u32, errors::RuntimeError> {
    let mut m = 0;

    for (_i, modify) in modifiers.iter().enumerate() {
        let a = Simulation::alias_to_modifier()[modify.trim()];

        m = m | a;

        if !Simulation::MODIFIERS.contains(&a) {
            return Err(errors::RuntimeError::TypeError(format!(
                "No such modifier '{}'",
                a
            )));
        }
    }

    Ok(m)
}

pub fn check_types(
    player: &PlayerSimulationXZ,
    func: functions::Function,
    args: Vec<String>,
    kwargs: IndexMap<String, String>,
    locals: Option<IndexMap<String, Data>>,
) -> Result<(Vec<functions::FullArgumentValue>, IndexMap<String, functions::FullArgumentValue>), errors::RuntimeError> {
    /*
    Type checks each argument in `args` and `kwargs` according to the
    annotations in `func`. If successful, returns a list of positional args
    and a dict of keyword args.

    Raises any appropriate error encountered while converting strings
    to the necessary datatypes.
    */

    // print(self.local_vars)

    let locals = if let Some(locals) = locals.clone() {
        locals
    } else {
        player.local_vars.clone()
    };

    let mut converted_args: Vec<functions::FullArgumentValue> = Vec::new();
    let mut converted_kwargs: IndexMap<String, functions::FullArgumentValue> = IndexMap::new();

    let mut positional_only: IndexMap<&String, &functions::ArgumentValue> = IndexMap::new();
    let mut positional_or_keyword: IndexMap<&String, &functions::ArgumentValue> = IndexMap::new();
    let mut keyword_only: IndexMap<&String, &functions::ArgumentValue> = IndexMap::new();
    let mut var_positional: IndexMap<&String, &functions::ArgumentValue> = IndexMap::new();
    let mut required_positionals: IndexMap<&String, &functions::ArgumentValue> = IndexMap::new();

    for arg in func.arguments().iter() {
        match arg {
            functions::Argument::PositionalOnly(name, value, is_required) => {
                if *is_required {
                    positional_only.insert(name, value);
                    required_positionals.insert(name, value);
                } else {
                    positional_only.insert(name, value);
                }
            },
            functions::Argument::PositionalOrKeyword(name, value, _) => { positional_or_keyword.insert(name, value); },
            functions::Argument::KeywordOnly(name, value, _) => { keyword_only.insert(name, value); },
            functions::Argument::VarPositional(name, value, _) => { var_positional.insert(name, value); },
        }
    }

    let mut can_be_positional: IndexMap<&String, &functions::ArgumentValue> = positional_only.into_iter().chain(positional_or_keyword).collect();

    if required_positionals.len() > args.len() {
        let number_of_missing = required_positionals.len() - args.len();

        return Err(errors::RuntimeError::TypeError(format!(
            "{} missing {} positional-only argument{}: {}",
            func.name(),
            number_of_missing,
            if number_of_missing > 1 { "s" } else { "" },
            required_positionals.keys().skip(args.len()).map(|key| String::as_str(*key)).collect::<Vec<_>>().join(", ")
        )));
    }

    for i in 0..std::cmp::min(args.len(), can_be_positional.len()) {
        let datatype = can_be_positional.values().map(|d| *d).collect::<Vec<_>>()[i];
        let dataname = can_be_positional.keys().map(|d| *d).collect::<Vec<_>>()[i];
        let mut converted_value;

//        if matches!(datatype, ArgumentValue::Empty( .. )) { This check should never succeed, well not this one but the actual equivalent of the check in the python version
//            converted_value = match safe_eval(player.local_vars.clone(), &args[i], DataType::Float, locals.clone()) {
//                Ok(v) => v,
//                Err(_) => safe_eval(player.local_vars.clone(), &args[i], DataType::Str, locals.clone())?,
//            };
//        } else {
        converted_value = safe_eval(player.local_vars.clone(), &args[i], datatype.datatype(), locals.clone())?;
//        }

        if dataname == "duration" {
            if *converted_value.get_as_int() < 0 {
                return Err(errors::RuntimeError::ValueError(
                    "Positional argument 'duration' should be a non-negative integer".to_string(),
                ));
            } else if matches!(datatype, ArgumentValue::Empty( .. )){
                println!("do not remove");
                println!("converted value (duration) assumed none");
                converted_value = Data::Int(1);
            }
        }
        // else if list(can_be_positional)[i] == "label" {
        //     if converted_value == None {
        //         converted_value = func.__name__;
        //     }
        // }

        converted_args.push(functions::argument_from_data(converted_value));
    }

    if args.len() < can_be_positional.len() {
        let a = args.len();

        can_be_positional = IndexMap::from_iter(
            can_be_positional.keys().map(|s| *s).collect::<Vec<_>>()[a..]
                .iter()
                .map(|x| (*x, can_be_positional[x])),
        );

    } else if !var_positional.is_empty() && args.len() > can_be_positional.len() {
        for j in &args[can_be_positional.len()..] {
            let c = safe_eval(
                player.local_vars.clone(),
                j,
                (*var_positional.values().next().unwrap()).datatype(),
                locals.clone(),
            )?;

            converted_args.push(argument_from_data(c));
        }
    } else if var_positional.is_empty() && args.len() > can_be_positional.len() {
        return Err(errors::RuntimeError::TypeError(format!(
            "{} accepts at most {} positional arguments, got {} instead",
            func.name(),
            can_be_positional.len(),
            args.len()
        )));
    }

    let can_be_keyword: IndexMap<&String, &functions::ArgumentValue> = can_be_positional.into_iter().chain(keyword_only).collect();

    // ### Check the keyword args ###

    for (kw, value) in kwargs.iter() {
        let datatype = can_be_keyword.get(kw);

        if datatype.is_none() {
            return Err(errors::RuntimeError::TypeError(format!(
                "{} has no keyword argument '{}'",
                func.name(),
                kw
            )));
        }

        converted_kwargs.insert(
            kw.clone(),
            argument_from_data(safe_eval(player.local_vars.clone(), value, (*datatype.unwrap()).datatype(), locals.clone())?),
        );

        // else {
        //     converted_kwargs.insert(
        //         kw.clone(),
        //         datatype(value)
        //     );
        // }
    }

    Ok((converted_args, converted_kwargs))
}