use std::fmt::{Display, Formatter};
use strum::{Display, EnumIter, EnumMessage, EnumString};
use std::string::ToString;
use crate::entities::{GearQuality, get_blacksmith_quality_cost, get_enchantment_quality_cost, get_woodworking_quality_cost, MaterialCost};
use crate::entities::materials::{EssenceRunes, PartMaterials, PotencyRunes, WeaponTraitMaterials};

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, EnumMessage)]
pub enum OneHandedWeapons {
    /// A una mano
    #[strum(serialize = "Maza")]
    Mace,
    /// A una mano
    #[strum(serialize = "Daga")]
    Dagger,
    /// A una mano
    #[strum(serialize = "Espada")]
    Sword,
    /// A una mano
    #[strum(serialize = "Hacha")]
    Axe,
    /// A una mano
    #[strum(serialize = "Escudo")]
    Shield
}

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, EnumMessage)]
pub enum TwoHandedWeapons {
    /// A dos manos
    #[strum(serialize = "Mazo")]
    Mace,
    /// A dos manos
    #[strum(serialize = "Mandoble")]
    Sword,
    /// A dos manos
    #[strum(serialize = "Hacha de combate")]
    Axe,
    /// A dos manos
    #[strum(serialize = "Bastón glacial")]
    FrostStaff,
    /// A dos manos
    #[strum(serialize = "Bastón infernal")]
    FireStaff,
    /// A dos manos
    #[strum(serialize = "Bastón eléctrico")]
    LightningStaff,
    /// A dos manos
    #[strum(serialize = "Bastón de restauración")]
    RestorationStaff,
    /// A dos manos
    #[strum(serialize = "Arco")]
    Bow
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, EnumMessage)]
pub enum WeaponKind {
    OneHanded(OneHandedWeapons),
    TwoHanded(TwoHandedWeapons)
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

impl std::str::FromStr for WeaponKind {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(w) = OneHandedWeapons::from_str(s) {
            Ok(WeaponKind::OneHanded(w))
        } else if let Ok(w) = TwoHandedWeapons::from_str(s) {
            Ok(WeaponKind::TwoHanded(w))
        } else {
            Err(strum::ParseError::VariantNotFound)
        }
    }
}

impl Display for WeaponKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponKind::OneHanded(w) => write!(f, "{}", w),
            WeaponKind::TwoHanded(w) => write!(f, "{}", w),
        }
    }
}

// impl MaterialCost for OneHandedWeapons {
//     fn cost(&self) -> Vec<(i32, String)> {
//         match *self {
//             OneHandedWeapons::Dagger => vec![(100, PartMaterials::RubediteIngots.to_string())],
//             OneHandedWeapons::Shield => vec![(140, PartMaterials::SandedRubyAsh.to_string())],
//             _ => vec![(110, PartMaterials::RubediteIngots.to_string())],
//         }
//     }
// }
//
// impl MaterialCost for TwoHandedWeapons {
//     fn cost(&self) -> Vec<(i32, String)> {
//         match *self {
//             TwoHandedWeapons::Bow |
//             TwoHandedWeapons::FireStaff |
//             TwoHandedWeapons::LightningStaff |
//             TwoHandedWeapons::RestorationStaff => vec![(120, PartMaterials::SandedRubyAsh.to_string())],
//             _ => vec![(140, PartMaterials::RubediteIngots.to_string())]
//         }
//     }
// }

// impl MaterialCost for WeaponKind {
//     fn cost(&self) -> Vec<(i32, String)> {
//         match self {
//             WeaponKind::OneHanded(w) => w.cost(),
//             WeaponKind::TwoHanded(w) => w.cost()
//         }
//     }
// }

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

fn get_quality_mats(weapon: &WeaponKind, quality: &GearQuality) -> Vec<(i32, String)> {
    match weapon {
        WeaponKind::OneHanded(w) => match w {
            OneHandedWeapons::Shield => get_woodworking_quality_cost(quality),
            _ => get_blacksmith_quality_cost(quality)
        }
        WeaponKind::TwoHanded(w) => match w {
            TwoHandedWeapons::Mace | TwoHandedWeapons::Sword | TwoHandedWeapons::Axe => get_blacksmith_quality_cost(quality),
            _ => get_woodworking_quality_cost(quality)
        }
    }
}

// impl MaterialCost for Weapon {
//     fn cost(&self) -> Vec<(i32, String)> {
//         let mut vec = Vec::new();
//         vec.append(&mut self.kind.cost());
//         vec.append(&mut self.weapon_trait.cost());
//         if let Some(e) = &self.enchantment {
//             vec.append(&mut e.cost());
//             vec.append(&mut get_enchantment_quality_cost(&self.quality));
//         }
//         vec.append(&mut get_quality_mats(&self.kind, &self.quality));
//         vec
//     }
// }