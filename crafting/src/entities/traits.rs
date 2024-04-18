use strum::{Display, EnumIter, EnumMessage, EnumString};
use crate::entities::{MaterialCost};
use crate::entities::materials::{ArmourTraitMaterials, JewelryTraitMaterials, WeaponTraitMaterials};
use crate::entities::weapon::{OneHandedWeapons, WeaponKind};

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, EnumMessage)]
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

// impl GearTraits {
//     pub fn for_piece(part: Gear) -> Vec<GearTraits> {
//         match part {
//             Gear::Weapon(weapon) => match weapon {
//                 WeaponKind::OneHanded(one_handed) => match one_handed {
//                     OneHandedWeapons::Shield => armour_traits(),
//                     _ => weapon_traits()
//                 }
//                 WeaponKind::TwoHanded(_) => weapon_traits()
//             },
//             Gear::Armour(_) => armour_traits(),
//             Gear::Jewelry(_) => jewelry_traits()
//         }
//     }
// }

// impl MaterialCost for GearTraits {
//     fn cost(&self) -> Vec<(i32, String)> {
//         match *self {
//             GearTraits::Divines => vec![(1, ArmourTraitMaterials::Sapphire.to_string())],
//             GearTraits::Invigorating => vec![(1, ArmourTraitMaterials::Garnet.to_string())],
//             GearTraits::Impenetrable => vec![(1, ArmourTraitMaterials::Diamond.to_string())],
//             GearTraits::Reinforced => vec![(1, ArmourTraitMaterials::Sardonyx.to_string())],
//             GearTraits::Sturdy => vec![(1, ArmourTraitMaterials::Quartz.to_string())],
//             GearTraits::WellFitted => vec![(1, ArmourTraitMaterials::Almandine.to_string())],
//             GearTraits::Charged => vec![(1, WeaponTraitMaterials::Amethyst.to_string())],
//             GearTraits::Defending => vec![(1, WeaponTraitMaterials::Turquoise.to_string())],
//             GearTraits::Powered => vec![(1, WeaponTraitMaterials::Chysolite.to_string())],
//             GearTraits::Infused => match kind {
//                 Gear::Weapon(_) => vec![(1, WeaponTraitMaterials::Jade.to_string())],
//                 Gear::Armour(_) => vec![(1, ArmourTraitMaterials::Bloodstone.to_string())],
//                 Gear::Jewelry(_) => vec![(1, JewelryTraitMaterials::AurbicAmber.to_string())],
//             }
//             GearTraits::Nirnhoned => if let Gear::Weapon(_) = kind {
//                 vec![(1, WeaponTraitMaterials::PotentNirncrux.to_string())]
//             } else {
//                 vec![(1, ArmourTraitMaterials::FortifiedNirncrux.to_string())]
//             }
//             GearTraits::Precise => vec![(1, WeaponTraitMaterials::Ruby.to_string())],
//             GearTraits::Sharpened => vec![(1, WeaponTraitMaterials::FireOpal.to_string())],
//             GearTraits::Training => if let Gear::Weapon(_) = kind {
//                 vec![(1, WeaponTraitMaterials::Carnelian.to_string())]
//             } else {
//                 vec![(1, ArmourTraitMaterials::Emerald.to_string())]
//             }
//             GearTraits::Decisive => vec![(1, WeaponTraitMaterials::Citrine.to_string())],
//             GearTraits::Arcane => vec![(1, JewelryTraitMaterials::Cobalt.to_string())],
//             GearTraits::Bloodthirsty => vec![(1, JewelryTraitMaterials::Slaughterstone.to_string())],
//             GearTraits::Harmony => vec![(1, JewelryTraitMaterials::Dibellium.to_string())],
//             GearTraits::Healthy => vec![(1, JewelryTraitMaterials::Antimony.to_string())],
//             GearTraits::Protective => vec![(1, JewelryTraitMaterials::Titanium.to_string())],
//             GearTraits::Robust => vec![(1, JewelryTraitMaterials::Zinc.to_string())],
//             GearTraits::Swift => vec![(1, JewelryTraitMaterials::GildingWax.to_string())],
//             GearTraits::Triune => vec![(1, JewelryTraitMaterials::DawnPrism.to_string())],
//         }
//     }
// }

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