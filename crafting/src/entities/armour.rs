use std::string::ToString;
use strum::{Display, EnumIter, EnumMessage, EnumString};
use crate::entities::{GearQuality, get_blacksmith_quality_cost, get_tailoring_quality_cost, MaterialCost};
use crate::entities::materials::{ArmourTraitMaterials, EssenceRunes, PartMaterials, PotencyRunes};

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, EnumMessage)]
pub enum ArmourParts {
    #[strum(serialize = "(Ligera) Cabeza")]
    LightHead,
    #[strum(serialize = "(Ligera) Hombros")]
    LightShoulder,
    #[strum(serialize = "(Ligera) Cuerpo")]
    LightBody,
    #[strum(serialize = "(Ligera) Manos")]
    LightHands,
    #[strum(serialize = "(Ligera) Cintura")]
    LightWaist,
    #[strum(serialize = "(Ligera) Piernas")]
    LightLegs,
    #[strum(serialize = "(Ligera) Pies")]
    LightFeet,

    #[strum(serialize = "(Media) Cabeza")]
    MediumHead,
    #[strum(serialize = "(Media) Hombros")]
    MediumShoulder,
    #[strum(serialize = "(Media) Cuerpo")]
    MediumBody,
    #[strum(serialize = "(Media) Manos")]
    MediumHands,
    #[strum(serialize = "(Media) Cintura")]
    MediumWaist,
    #[strum(serialize = "(Media) Piernas")]
    MediumLegs,
    #[strum(serialize = "(Media) Pies")]
    MediumFeet,

    #[strum(serialize = "(Pesada) Cabeza")]
    HeavyHead,
    #[strum(serialize = "(Pesada) Hombros")]
    HeavyShoulder,
    #[strum(serialize = "(Pesada) Cuerpo")]
    HeavyBody,
    #[strum(serialize = "(Pesada) Manos")]
    HeavyHands,
    #[strum(serialize = "(Pesada) Cintura")]
    HeavyWaist,
    #[strum(serialize = "(Pesada) Piernas")]
    HeavyLegs,
    #[strum(serialize = "(Pesada) Pies")]
    HeavyFeet,
}

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, EnumMessage)]
pub enum ArmourEnchantments {
    /// Aumenta la salud máxima
    #[strum(serialize = "Glifo de salud")]
    Health,
    /// Aumenta la magia máxima
    #[strum(serialize = "Glifo de magia")]
    Magicka,
    /// Aumenta el aguante máximo
    #[strum(serialize = "Glifo de aguante")]
    Stamina,
    /// Aumente la magia, salud y aguante máximos
    #[strum(serialize = "Glifo de defensa prismática")]
    PrismaticDefense
}

// impl MaterialCost for ArmourEnchantments {
//     fn cost(&self) -> Vec<(i32, String)> {
//         match *self {
//             ArmourEnchantments::Health => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Oko.to_string())],
//             ArmourEnchantments::Magicka => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Makko.to_string())],
//             ArmourEnchantments::Stamina => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Deni.to_string())],
//             ArmourEnchantments::PrismaticDefense => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Hakeijo.to_string())],
//         }
//     }
// }

fn get_quality_mats(part: &ArmourParts, quality: &GearQuality) -> Vec<(i32, String)> {
    match part {
        ArmourParts::HeavyFeet | ArmourParts::HeavyHands | ArmourParts::HeavyBody | ArmourParts::HeavyHead |
        ArmourParts::HeavyLegs | ArmourParts::HeavyShoulder | ArmourParts::HeavyWaist => get_blacksmith_quality_cost(quality),
        _ => get_tailoring_quality_cost(quality)
    }
}

fn get_part_mats(part: &ArmourParts) -> Vec<(i32, String)> {
    match part {
        ArmourParts::LightHead => vec![(130, PartMaterials::AncestorSilk.to_string())],
        ArmourParts::LightShoulder => vec![(130, PartMaterials::AncestorSilk.to_string())],
        ArmourParts::LightBody => vec![(150, PartMaterials::AncestorSilk.to_string())],
        ArmourParts::LightHands => vec![(130, PartMaterials::AncestorSilk.to_string())],
        ArmourParts::LightWaist => vec![(130, PartMaterials::AncestorSilk.to_string())],
        ArmourParts::LightLegs => vec![(140, PartMaterials::AncestorSilk.to_string())],
        ArmourParts::LightFeet => vec![(130, PartMaterials::AncestorSilk.to_string())],
        ArmourParts::MediumHead => vec![(130, PartMaterials::RubedoLeather.to_string())],
        ArmourParts::MediumShoulder => vec![(130, PartMaterials::RubedoLeather.to_string())],
        ArmourParts::MediumBody => vec![(150, PartMaterials::RubedoLeather.to_string())],
        ArmourParts::MediumHands => vec![(130, PartMaterials::RubedoLeather.to_string())],
        ArmourParts::MediumWaist => vec![(130, PartMaterials::RubedoLeather.to_string())],
        ArmourParts::MediumLegs => vec![(140, PartMaterials::RubedoLeather.to_string())],
        ArmourParts::MediumFeet => vec![(130, PartMaterials::RubedoLeather.to_string())],
        ArmourParts::HeavyHead => vec![(130, PartMaterials::RubediteIngots.to_string())],
        ArmourParts::HeavyShoulder => vec![(130, PartMaterials::RubediteIngots.to_string())],
        ArmourParts::HeavyBody => vec![(150, PartMaterials::RubediteIngots.to_string())],
        ArmourParts::HeavyHands => vec![(130, PartMaterials::RubediteIngots.to_string())],
        ArmourParts::HeavyWaist => vec![(130, PartMaterials::RubediteIngots.to_string())],
        ArmourParts::HeavyLegs => vec![(140, PartMaterials::RubediteIngots.to_string())],
        ArmourParts::HeavyFeet => vec![(130, PartMaterials::RubediteIngots.to_string())],
    }
}

// impl MaterialCost for Armour {
//     fn cost(&self) -> Vec<(i32, String)> {
//         let mut vec = Vec::new();
//         vec.append(&mut get_part_mats(&self.kind));
//         vec.append(&mut self.armour_trait.cost());
//
//         vec.append(&mut get_quality_mats(&self.kind, &self.quality));
//         vec
//     }
// }