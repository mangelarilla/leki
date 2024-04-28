use serenity::all::CreateSelectMenuOption;
use strum::{Display, EnumIter, EnumMessage, EnumString, IntoEnumIterator};
use crate::entities::{GearQuality, get_blacksmith_quality_cost, get_woodworking_quality_cost};
use crate::entities::materials::{PartMaterials, QualityMaterials};
use crate::entities::weapon::Weapons::*;

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, EnumMessage)]
pub enum Weapons {
    #[strum(serialize = "(1 Mano) Maza")]
    OneHandedMace,
    #[strum(serialize = "(1 Mano) Daga")]
    OneHandedDagger,
    #[strum(serialize = "(1 Mano) Espada")]
    OneHandedSword,
    #[strum(serialize = "(1 Mano) Hacha")]
    OneHandedAxe,
    #[strum(serialize = "Escudo")]
    OneHandedShield,
    #[strum(serialize = "(2 Manos) Mazo")]
    TwoHandedMace,
    #[strum(serialize = "(2 Manos) Mandoble")]
    TwoHandedSword,
    #[strum(serialize = "(2 Manos) Hacha de combate")]
    TwoHandedAxe,
    #[strum(serialize = "Bastón glacial")]
    TwoHandedFrostStaff,
    #[strum(serialize = "Bastón infernal")]
    TwoHandedFireStaff,
    #[strum(serialize = "Bastón eléctrico")]
    TwoHandedLightningStaff,
    #[strum(serialize = "Bastón de restauración")]
    TwoHandedRestorationStaff,
    #[strum(serialize = "Arco")]
    TwoHandedBow
}

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, EnumMessage)]
pub enum WeaponEnchantments {
    /// Inflige daño de llamas
    #[strum(serialize = "Glifo de fuego")]
    Fire,
    /// Inflige daño de escarcha
    #[strum(serialize = "Glifo de escarcha")]
    Frost,
    /// Inflige daño de descarga eléctica
    #[strum(serialize = "Glifo de descarga")]
    Shock,
    /// Inflige daño de veneno
    #[strum(serialize = "Glifo de veneno")]
    Poison,
    /// Inflige daño de enfermedad
    #[strum(serialize = "Glifo de podredumbre")]
    Foulness,
    /// Inflige daño de Oblivion usando la salud máxima del enemigo
    #[strum(serialize = "Glifo de disminución de salud")]
    DecreaseHealth,
    /// Otorga un escudo de daño que protege del daño
    #[strum(serialize = "Glifo de robustez")]
    Hardening,
    /// Inglige daño de magia y restablece salud
    #[strum(serialize = "Glifo de absorción de salud")]
    AbsorbHealth,
    /// Inflige daño de magia y recuperas magia
    #[strum(serialize = "Glifo de absorción de magia")]
    AbsorbMagicka,
    /// Inflige daño físico y recuperas aguante
    #[strum(serialize = "Glifo de absorción de aguante")]
    AbsorbStamina,
    /// Aumenta el daño de arma y hechizo
    #[strum(serialize = "Glifo de daño por arma")]
    WeaponDamage,
    /// Reduce el daño de arma y hechizo del objetivo
    #[strum(serialize = "Glifo de debilidad")]
    Weakening,
    /// Reduce la resistencia física y a hechizos del objetivo
    #[strum(serialize = "Glifo de aplastamiento")]
    Crushing,
    /// Inflige daño de magia y restablece salud, magia y aguante
    #[strum(serialize = "Glifo de asalto prismático")]
    PrismaticOnslaught
}

impl Weapons {
    pub fn select_options() -> Vec<CreateSelectMenuOption> {
        Weapons::iter()
            .map(|i| if i.to_string().starts_with("(1 Mano)") {
                vec![
                    CreateSelectMenuOption::new(i.to_string(), format!("{}_1", i.to_string())),
                    CreateSelectMenuOption::new(i.to_string(), format!("{}_2", i.to_string()))
                ]
            } else {
                vec![CreateSelectMenuOption::new(i.to_string(), i.to_string())]
            })
            .flatten()
            .collect()
    }

    pub fn calculate_cost(&self) -> PartMaterials {
        match self {
            OneHandedDagger => PartMaterials::RubediteIngots(100),
            OneHandedShield => PartMaterials::SandedRubyAsh(140),
            OneHandedAxe | OneHandedMace | OneHandedSword => PartMaterials::RubediteIngots(110),
            TwoHandedFrostStaff | TwoHandedFireStaff | TwoHandedLightningStaff |
            TwoHandedRestorationStaff | TwoHandedBow => PartMaterials::SandedRubyAsh(120),
            _ => PartMaterials::RubediteIngots(140)
        }
    }

    pub fn calculate_quality_cost(&self, quality: &GearQuality) -> Vec<QualityMaterials> {
        match *self {
            TwoHandedFrostStaff | TwoHandedFireStaff | TwoHandedLightningStaff |
            TwoHandedRestorationStaff | TwoHandedBow => get_woodworking_quality_cost(quality)
                .into_iter()
                .map(|c| QualityMaterials::Woodworking(c))
                .collect(),
            _ => get_blacksmith_quality_cost(quality)
                .into_iter()
                .map(|c| QualityMaterials::Blacksmith(c))
                .collect()
        }
    }
}

// impl MaterialCost for WeaponEnchantments {
//     fn cost(&self) -> Vec<(i32, String)> {
//         match *self {
//             WeaponEnchantments::Fire => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Rakeipa.to_string())],
//             WeaponEnchantments::Frost => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Dekeipa.to_string())],
//             WeaponEnchantments::Shock => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Meip.to_string())],
//             WeaponEnchantments::Poison => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Kuoko.to_string())],
//             WeaponEnchantments::Foulness => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Haoko.to_string())],
//             WeaponEnchantments::DecreaseHealth => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Okoma.to_string())],
//             WeaponEnchantments::Hardening => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Deteri.to_string())],
//             WeaponEnchantments::AbsorbHealth => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Oko.to_string())],
//             WeaponEnchantments::AbsorbMagicka => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Makko.to_string())],
//             WeaponEnchantments::AbsorbStamina => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Deni.to_string())],
//             WeaponEnchantments::WeaponDamage => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Okori.to_string())],
//             WeaponEnchantments::Weakening => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Okori.to_string())],
//             WeaponEnchantments::Crushing => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Deteri.to_string())],
//             WeaponEnchantments::PrismaticOnslaught => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Hakeijo.to_string())],
//         }
//     }
// }