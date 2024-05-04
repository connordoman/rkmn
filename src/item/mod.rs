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

pub enum RkmnItem {
    Ball(ball::RkBall),
    Berry(berry::Berry),
    Collectible(collectible::CollectibleItem),
    Key(key::KeyItem),
    Mail(mail::Mail),
}

impl From<ball::RkBall> for RkmnItem {
    fn from(item: ball::RkBall) -> Self {
        RkmnItem::Ball(item)
    }
}

impl From<berry::Berry> for RkmnItem {
    fn from(item: berry::Berry) -> Self {
        RkmnItem::Berry(item)
    }
}

impl From<collectible::CollectibleItem> for RkmnItem {
    fn from(item: collectible::CollectibleItem) -> Self {
        RkmnItem::Collectible(item)
    }
}

impl From<key::KeyItem> for RkmnItem {
    fn from(item: key::KeyItem) -> Self {
        RkmnItem::Key(item)
    }
}

impl From<mail::Mail> for RkmnItem {
    fn from(item: mail::Mail) -> Self {
        RkmnItem::Mail(item)
    }
}
