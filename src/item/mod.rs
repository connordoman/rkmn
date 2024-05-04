pub mod ball;
pub mod berry;
pub mod collectible;
pub mod key;
pub mod mail;

pub trait Item {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn value(&self) -> u16;
}
