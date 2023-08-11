use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Type {
    #[default]
    Minion,
    Spell,
    Weapon,
}

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CardClass {
    Druid,
    Hunter,
    Mage,
    #[default]
    Neutral,
    Paladin,
    Priest,
    Rogue,
    Shaman,
    Warlock,
    Warrior,
}

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Race {
    #[default]
    #[serde(rename = "")]
    None,
    Beast,
    Demon,
    Dragon,
    Mechanical,
    Murloc,
    Pirate,
    Totem,
    Undead,
}

#[derive(PartialEq, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Rarity {
    #[default]
    Free,
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: Type,
    pub card_class: CardClass,
    pub text: String,
    pub cost: i32,
    pub attack: i32,
    pub health: i32,
    pub durability: i32,
    pub race: Race,
    pub rarity: Rarity,
    pub flavor: String,
}
