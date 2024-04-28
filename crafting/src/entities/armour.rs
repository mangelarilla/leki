use strum::{Display, EnumIter, EnumMessage, EnumString};
use crate::entities::{GearQuality, get_blacksmith_quality_cost, get_tailoring_quality_cost};
use crate::entities::armour::ArmourParts::*;
use crate::entities::materials::{PartMaterials, QualityMaterials};

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

impl ArmourParts {
    pub fn calculate_cost(&self) -> PartMaterials {
        match self {
            LightHead | LightShoulder | LightHands | LightWaist| LightFeet => PartMaterials::AncestorSilk(130),
            LightBody => PartMaterials::AncestorSilk(150),
            LightLegs => PartMaterials::AncestorSilk(140),

            MediumHead | MediumShoulder | MediumHands | MediumWaist| MediumFeet => PartMaterials::RubedoLeather(130),
            MediumBody => PartMaterials::RubedoLeather(150),
            MediumLegs => PartMaterials::RubedoLeather(140),

            HeavyHead | HeavyShoulder | HeavyHands | HeavyWaist | HeavyFeet => PartMaterials::RubediteIngots(130),
            HeavyBody => PartMaterials::RubediteIngots(150),
            HeavyLegs => PartMaterials::RubediteIngots(140),
        }
    }

    pub fn calculate_quality_cost(&self, quality: &GearQuality) -> Vec<QualityMaterials> {
        match self {
            HeavyFeet | HeavyHands | HeavyBody | HeavyHead |
            HeavyLegs | HeavyShoulder | HeavyWaist => get_blacksmith_quality_cost(quality)
                .into_iter().map(|b| QualityMaterials::Blacksmith(b)).collect(),
            _ => get_tailoring_quality_cost(quality)
                .into_iter().map(|b| QualityMaterials::Tailoring(b)).collect()
        }
    }
}