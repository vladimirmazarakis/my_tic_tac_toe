use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player{
    Cross,
    Circle
}


impl Display for Player{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Circle => {
                print!("Circle");
                return Ok(());
            },
            Self::Cross => {
                print!("Cross");
                return Ok(());
            }
        };
    }
}
