

pub fn mcsin(total_angles: i32, pi: f32, rad: f32) -> f32 {
    if total_angles == -1 {
        return rad.sin();
    } else if total_angles == 65536 {
        let index = (rad * 10430.378_f32) as i32 & 65535;
        return (index as f32 * pi * 2.0 / total_angles as f32).sin() as f32;
    } else {
        let index = ((1.0 / (2.0 * pi) * total_angles as f32 * rad) as i32)
            & (total_angles - 1);
        return (index as f32 * pi * 2.0 / total_angles as f32).sin() as f32;
    }
}

pub fn mccos(total_angles: i32, pi: f32, rad: f32) -> f32 {
    if total_angles == -1 {
        return rad.cos();
    } else if total_angles == 65536 {
        let index = ((rad * 10430.378_f32 + 16384.0_f32) as i32) & 65535;
        return (index as f32 * pi * 2.0 / total_angles as f32).sin() as f32;
    } else {
        let index = ((1.0 / (2.0 * pi) * total_angles as f32 * rad
            + total_angles as f32 / 4.0) as i32)
            & (total_angles - 1);
        return (index as f32 * pi * 2.0 / total_angles as f32).sin() as f32;
    }
}

pub fn truncate_number(precision: i32, value: f64) -> String {
    // Round decimals to `self.precision` decimal places
    format!("{:.*}", precision as usize, value)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

pub fn mm_to_distf32(mm: f32) -> f32 {
    mm + 0.6_f32.copysign(mm)
}

pub fn dist_to_blockf32(mm: f32) -> f32 {
    mm + 0.6_f32.copysign(mm)
}

pub fn dist_to_mmf32(dist: f32) -> f32 {
    dist - 0.6_f32.copysign(dist)
}

pub fn block_to_distf32(dist: f32) -> f32 {
    dist - 0.6_f32.copysign(dist)
}

pub fn mm_to_distf64(mm: f64) -> f64 {
    mm + 0.6_f64.copysign(mm)
}

pub fn dist_to_blockf64(mm: f64) -> f64 {
    mm + 0.6_f64.copysign(mm)
}

pub fn dist_to_mmf64(dist: f64) -> f64 {
    dist - 0.6_f64.copysign(dist)
}

pub fn block_to_distf64(dist: f64) -> f64 {
    dist - 0.6_f64.copysign(dist)
}
