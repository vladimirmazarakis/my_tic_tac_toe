use std::fmt;

use crate::player::Player;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell{
    Empty,
    Player(Player) 
}

impl fmt::Display for Cell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self{
            Cell::Player(Player::Circle) => write!(f,"Circle"),
            Cell::Player(Player::Cross) => write!(f,"Cross"),
            Cell::Empty => write!(f,"Empty")
        }
    }
}