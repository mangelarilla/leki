use strum::{Display, EnumIter, EnumProperty, EnumString};
use crate::entities::materials::{BlacksmithQualityMaterials, JewelryQualityMaterials, RuneQualityMaterials, TailoringQualityMaterials, WoodworkingQualityMaterials};

pub mod armour;
pub mod weapon;
pub mod jewelry;
pub mod materials;
pub mod traits;

#[derive(EnumIter, Clone, Ord, PartialOrd, Eq, PartialEq, EnumString, Display, EnumProperty)]
pub enum GearQuality {
    #[strum(serialize = "Blanco")]
    #[strum(props(Emoji = "⚪"))]
    White,
    #[strum(serialize = "Verde")]
    #[strum(props(Emoji = "🟢"))]
    Green,
    #[strum(serialize = "Azul")]
    #[strum(props(Emoji = "🔵"))]
    Blue,
    #[strum(serialize = "Morada")]
    #[strum(props(Emoji = "🟣"))]
    Purple,
    #[strum(serialize = "Amarilla")]
    #[strum(props(Emoji = "🟡"))]
    Yellow
}

fn get_enchantment_quality_cost(quality: &GearQuality) -> Vec<(i32, String)> {
    match quality {
        GearQuality::White => vec![(1, RuneQualityMaterials::Ta.to_string())],
        GearQuality::Green => vec![(1, RuneQualityMaterials::Jejota.to_string())],
        GearQuality::Blue => vec![(1, RuneQualityMaterials::Denata.to_string())],
        GearQuality::Purple => vec![(1, RuneQualityMaterials::Rekuta.to_string())],
        GearQuality::Yellow => vec![(1, RuneQualityMaterials::Kuta.to_string())],
    }
}

fn get_blacksmith_quality_cost(quality: &GearQuality) -> Vec<BlacksmithQualityMaterials> {
    let green = BlacksmithQualityMaterials::HoningStone(2);
    let blue = BlacksmithQualityMaterials::DwarvenOil(3);
    let purple = BlacksmithQualityMaterials::GrainSolvent(4);
    let yellow = BlacksmithQualityMaterials::TemperingAlloy(8);

    get_quality_scale(quality, green, blue, purple, yellow)
}

fn get_tailoring_quality_cost(quality: &GearQuality) -> Vec<TailoringQualityMaterials> {
    let green = TailoringQualityMaterials::Hemming(2);
    let blue = TailoringQualityMaterials::Embroidery(3);
    let purple = TailoringQualityMaterials::ElegantLining(4);
    let yellow = TailoringQualityMaterials::DreughWax(8);

    get_quality_scale(quality, green, blue, purple, yellow)
}

fn get_woodworking_quality_cost(quality: &GearQuality) -> Vec<WoodworkingQualityMaterials> {
    let green = WoodworkingQualityMaterials::Pitch(2);
    let blue = WoodworkingQualityMaterials::Turpen(3);
    let purple = WoodworkingQualityMaterials::Mastic(4);
    let yellow = WoodworkingQualityMaterials::Rosin(8);

    get_quality_scale(quality, green, blue, purple, yellow)
}

fn get_jewelry_quality_cost(quality: &GearQuality) -> Vec<JewelryQualityMaterials> {
    let green = JewelryQualityMaterials::TernePlating(2);
    let blue = JewelryQualityMaterials::IridiumPlating(3);
    let purple = JewelryQualityMaterials::ZirconPlating(4);
    let yellow = JewelryQualityMaterials::ChromiumPlating(8);

    get_quality_scale(quality, green, blue, purple, yellow)
}

fn get_quality_scale<T>(quality: &GearQuality, green: T, blue: T, purple: T, yellow: T) -> Vec<T> {
    match quality {
        GearQuality::White => vec![],
        GearQuality::Green => vec![green],
        GearQuality::Blue => vec![green, blue],
        GearQuality::Purple => vec![green, blue, purple],
        GearQuality::Yellow => vec![green, blue, purple, yellow],
    }
}