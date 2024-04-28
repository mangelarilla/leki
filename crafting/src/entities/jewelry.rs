use strum::{Display, EnumIter, EnumMessage, EnumString};
use crate::entities::{GearQuality, get_jewelry_quality_cost};
use crate::entities::materials::{PartMaterials, QualityMaterials};

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, EnumString, Display, EnumMessage)]
pub enum Jewelries {
    /// Solo uno
    #[strum(serialize = "Collar")]
    Necklace,
    /// Se asumen dos anillos
    #[strum(serialize = "Anillo")]
    Ring
}

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, EnumString, Display, EnumMessage)]
pub enum JewelryEnchantments {
    /// Añade daño de arma y hechizo, y recuperación de aguante
    #[strum(serialize = "Glifo de aumento de daño físico")]
    IncreasePhysicalHarm,
    /// Añade daño de arma y hechizo, y recuperación de magia
    #[strum(serialize = "Glifo de aumento de daño mágico")]
    IncreaseMagicalHarm,
    /// Añade recuperación de salud
    #[strum(serialize = "Glifo de regeneración de salud")]
    HealthRecovery,
    /// Añade recuperación de magia
    #[strum(serialize = "Glifo de regeneración de magia")]
    MagickaRecovery,
    /// Añade recuperación de aguante
    #[strum(serialize = "Glifo de regeneración de aguante")]
    StaminaRecovery,
    /// Reduce el coste de magia de las habilidades
    #[strum(serialize = "Glifo de reducción de coste de magia")]
    ReduceSpellCost,
    /// Reduce el coste de aguante de las habilidades
    #[strum(serialize = "Glifo de reducción de coste de aguante")]
    ReduceFeatCost,
    /// Reduce el coste de bloquear
    #[strum(serialize = "Glifo de bloqueo")]
    Shielding,
    /// Añade daño a tus ataques de aporreo
    #[strum(serialize = "Glifo de percusión")]
    Bashing,
    /// Añade resistencia física
    #[strum(serialize = "Glifo de resistencia al daño físico")]
    DecreasePhysicalHarm,
    /// Añade resistencia a los hechizos
    #[strum(serialize = "Glifo de resistencia al daño mágico")]
    DecreaseSpellHarm,
    /// Añade resistencia a las llamas
    #[strum(serialize = "Glifo de resistencia al fuego")]
    FlameResist,
    /// Añade resistencia a la escarcha
    #[strum(serialize = "Glifo de resistencia a la congelación")]
    FrostResist,
    /// Añade resistencia a descargas eléctricas
    #[strum(serialize = "Glifo de resistencia a las descargas")]
    ShockResist,
    /// Añade resistencia a venenos
    #[strum(serialize = "Glifo de resistencia al veneno")]
    PoisonResist,
    /// Añade resistencia a enfermedades
    #[strum(serialize = "Glifo de resistencia a las enfermedades")]
    DiseaseResist,
    /// Aumenta la duración de los efectos de las pociones
    #[strum(serialize = "Glifo de amplificación alquímica")]
    PotionResist,
    /// Reduce la reutilización de las pociones
    #[strum(serialize = "Glifo de aceleración alquímica")]
    PotionBoost,
    /// Reduce el coste de salud, magia y aguante de las habilidades
    #[strum(serialize = "Glifo de reducción de coste de habilidades")]
    ReduceSkillCost,
    /// Añade recuperación de magia, salud y aguante
    #[strum(serialize = "Glifo de regeneración prismática")]
    PrismaticRecovery
}

impl Jewelries {
    pub fn calculate_cost(&self) -> PartMaterials {
        match *self {
            Jewelries::Necklace => PartMaterials::PlatinumOunces(150),
            Jewelries::Ring => PartMaterials::PlatinumOunces(100)
        }
    }

    pub fn calculate_quality_cost(&self, quality: &GearQuality) -> Vec<QualityMaterials> {
        get_jewelry_quality_cost(quality)
            .into_iter()
            .map(|c| QualityMaterials::Jewelry(c))
            .collect()
    }
}

// impl MaterialCost for JewelryEnchantments {
//     fn cost(&self) -> Vec<(i32, String)> {
//         match *self {
//             JewelryEnchantments::IncreasePhysicalHarm => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Taderi.to_string())],
//             JewelryEnchantments::IncreaseMagicalHarm => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Makderi.to_string())],
//             JewelryEnchantments::HealthRecovery => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Okoma.to_string())],
//             JewelryEnchantments::MagickaRecovery => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Makkoma.to_string())],
//             JewelryEnchantments::StaminaRecovery => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Denima.to_string())],
//             JewelryEnchantments::ReduceSpellCost => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Makkoma.to_string())],
//             JewelryEnchantments::ReduceFeatCost => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Denima.to_string())],
//             JewelryEnchantments::Shielding => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Kaderi.to_string())],
//             JewelryEnchantments::Bashing => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Kaderi.to_string())],
//             JewelryEnchantments::DecreasePhysicalHarm => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Taderi.to_string())],
//             JewelryEnchantments::DecreaseSpellHarm => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Makderi.to_string())],
//             JewelryEnchantments::FlameResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Rakeipa.to_string())],
//             JewelryEnchantments::FrostResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Dekeipa.to_string())],
//             JewelryEnchantments::ShockResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Meip.to_string())],
//             JewelryEnchantments::PoisonResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Kuoko.to_string())],
//             JewelryEnchantments::DiseaseResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Haoko.to_string())],
//             JewelryEnchantments::PotionResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Oru.to_string())],
//             JewelryEnchantments::PotionBoost => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Oru.to_string())],
//             JewelryEnchantments::ReduceSkillCost => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Indeko.to_string())],
//             JewelryEnchantments::PrismaticRecovery => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Indeko.to_string())],
//         }
//     }
// }