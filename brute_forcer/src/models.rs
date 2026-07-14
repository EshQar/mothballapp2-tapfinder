pub trait Position: Copy {}
impl Position for f32 {}
impl Position for (f32, f32) {}


pub trait Conglomerate {
    type Pos: Position;
    fn advance(&mut self) -> bool;
    fn pos(&self) -> Self::Pos;
    fn weight_state(&self) -> Vec<&[isize]>;
}

pub trait Goal<P: Position> {
    type Dist;
    fn is_satisfied(&self, pos: P) -> bool;
    fn get_quality(&self, pos: P) -> f32;
    fn get_dists(&self, pos: P) -> Self::Dist;
}