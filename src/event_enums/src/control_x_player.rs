use utils::{Player};

#[derive(Debug)]
pub enum ControlToPlayer {
    Up(f64, Player),
    Down(f64, Player),
    Left(f64, Player),
    Right(f64, Player),
}

#[derive(Debug)]
pub enum ControlFromPlayer {

}
