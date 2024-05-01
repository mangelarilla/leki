use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumMessage, EnumString};
use crate::entities::materials::{ArmourTraitMaterials, JewelryTraitMaterials, WeaponTraitMaterials};
use crate::prelude::*;

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, EnumMessage, Serialize, Deserialize)]
pub enum GearTraits {
    #[strum(serialize = "Imbuición")] Infused,
    #[strum(serialize = "Temple de Nirn")] Nirnhoned,
    #[strum(serialize = "Entrenamiento")] Training,

    #[strum(serialize = "Divinidad")] Divines,
    #[strum(serialize = "Vigorización")] Invigorating,
    #[strum(serialize = "Impenetrabilidad")] Impenetrable,
    #[strum(serialize = "Refuerzo")] Reinforced,
    #[strum(serialize = "Solidez")] Sturdy,
    #[strum(serialize = "Buen ajuste")] WellFitted,

    #[strum(serialize = "Arcanidad")] Arcane,
    #[strum(serialize = "Sed de sangre")] Bloodthirsty,
    #[strum(serialize = "Armonía")] Harmony,
    #[strum(serialize = "Saludable")] Healthy,
    #[strum(serialize = "Protección")] Protective,
    #[strum(serialize = "Robustez")] Robust,
    #[strum(serialize = "Agilidad")] Swift,
    #[strum(serialize = "Trinidad")] Triune,

    #[strum(serialize = "Carga")] Charged,
    #[strum(serialize = "Defensa")] Defending,
    #[strum(serialize = "Potencia")] Powered,
    #[strum(serialize = "Precisión")] Precise,
    #[strum(serialize = "Filo")] Sharpened,
    #[strum(serialize = "Decisivo")] Decisive
}

impl GearTraits {
    pub fn armour_cost(&self) -> Result<ArmourTraitMaterials> {
        match *self {
            GearTraits::Infused => Ok(ArmourTraitMaterials::Bloodstone),
            GearTraits::Divines => Ok(ArmourTraitMaterials::Sapphire),
            GearTraits::Invigorating => Ok(ArmourTraitMaterials::Garnet),
            GearTraits::Impenetrable => Ok(ArmourTraitMaterials::Diamond),
            GearTraits::Nirnhoned => Ok(ArmourTraitMaterials::FortifiedNirncrux),
            GearTraits::Reinforced => Ok(ArmourTraitMaterials::Sardonyx),
            GearTraits::Sturdy => Ok(ArmourTraitMaterials::Quartz),
            GearTraits::Training => Ok(ArmourTraitMaterials::Emerald),
            GearTraits::WellFitted => Ok(ArmourTraitMaterials::Almandine),
            _ => Err(Error::InvalidTrait)
        }
    }

    pub fn weapon_cost(&self) -> Result<WeaponTraitMaterials> {
        match *self {
            GearTraits::Charged => Ok(WeaponTraitMaterials::Amethyst),
            GearTraits::Defending => Ok(WeaponTraitMaterials::Turquoise),
            GearTraits::Powered => Ok(WeaponTraitMaterials::Chysolite),
            GearTraits::Infused => Ok(WeaponTraitMaterials::Jade),
            GearTraits::Nirnhoned => Ok(WeaponTraitMaterials::PotentNirncrux),
            GearTraits::Precise => Ok(WeaponTraitMaterials::Ruby),
            GearTraits::Sharpened => Ok(WeaponTraitMaterials::FireOpal),
            GearTraits::Training => Ok(WeaponTraitMaterials::Carnelian),
            GearTraits::Decisive => Ok(WeaponTraitMaterials::Citrine),
            _ => Err(Error::InvalidTrait)
        }
    }

    pub fn jewelry_cost(&self) -> Result<JewelryTraitMaterials> {
        match *self {
            GearTraits::Infused => Ok(JewelryTraitMaterials::AurbicAmber),
            GearTraits::Arcane => Ok(JewelryTraitMaterials::Cobalt),
            GearTraits::Bloodthirsty => Ok(JewelryTraitMaterials::Slaughterstone),
            GearTraits::Harmony => Ok(JewelryTraitMaterials::Dibellium),
            GearTraits::Healthy => Ok(JewelryTraitMaterials::Antimony),
            GearTraits::Protective => Ok(JewelryTraitMaterials::Titanium),
            GearTraits::Robust => Ok(JewelryTraitMaterials::Zinc),
            GearTraits::Swift => Ok(JewelryTraitMaterials::GildingWax),
            GearTraits::Triune => Ok(JewelryTraitMaterials::DawnPrism),
            _ => Err(Error::InvalidTrait)
        }
    }
}

pub fn armour_traits() -> Vec<GearTraits> {
    vec![
        GearTraits::Infused,
        GearTraits::Divines,
        GearTraits::Invigorating,
        GearTraits::Impenetrable,
        GearTraits::Nirnhoned,
        GearTraits::Reinforced,
        GearTraits::Sturdy,
        GearTraits::Training,
        GearTraits::WellFitted,
    ]
}

pub fn weapon_traits() -> Vec<GearTraits> {
    vec![
        GearTraits::Charged,
        GearTraits::Defending,
        GearTraits::Powered,
        GearTraits::Infused,
        GearTraits::Nirnhoned,
        GearTraits::Precise,
        GearTraits::Sharpened,
        GearTraits::Training,
        GearTraits::Decisive
    ]
}

pub fn jewelry_traits() -> Vec<GearTraits> {
    vec![
        GearTraits::Infused,
        GearTraits::Arcane,
        GearTraits::Bloodthirsty,
        GearTraits::Harmony,
        GearTraits::Healthy,
        GearTraits::Protective,
        GearTraits::Robust,
        GearTraits::Swift,
        GearTraits::Triune,
    ]
}