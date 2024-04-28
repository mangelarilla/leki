use std::fmt::{Display, Formatter};
use std::hash::{Hash};
use strum::{Display, EnumString};
use derivative::Derivative;

#[derive(EnumString, Display, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub enum PartMaterials {
    #[strum(serialize = "Seda ancestral (Ancestor Silk)")] AncestorSilk(#[derivative(PartialEq="ignore")]
                                                                        #[derivative(Hash="ignore")]u32),
    #[strum(serialize = "Cuero rubedo (Rubedo Leather)")] RubedoLeather(#[derivative(PartialEq="ignore")]
                                                                        #[derivative(Hash="ignore")]u32),
    #[strum(serialize = "Lingote de rubedita (Rubedite Ingots)")] RubediteIngots(#[derivative(PartialEq="ignore")]
                                                                                 #[derivative(Hash="ignore")]u32),
    #[strum(serialize = "Madera de fresno rubí lijado (Sanded Ruby Ash)")] SandedRubyAsh(#[derivative(PartialEq="ignore")]
                                                                                         #[derivative(Hash="ignore")]u32),
    #[strum(serialize = "Onza de platino (Platinum Ounces)")] PlatinumOunces(#[derivative(PartialEq="ignore")]
                                                                             #[derivative(Hash="ignore")]u32)
}

#[derive(EnumString, Display)]
pub enum RuneQualityMaterials {
    Ta, Jejota, Denata, Rekuta, Kuta
}

#[derive(PartialEq, Eq, Hash)]
pub enum QualityMaterials {
    Tailoring(TailoringQualityMaterials),
    Blacksmith(BlacksmithQualityMaterials),
    Woodworking(WoodworkingQualityMaterials),
    Jewelry(JewelryQualityMaterials)
}

#[derive(EnumString, Display, PartialEq, Eq, Hash)]
pub enum TailoringQualityMaterials {
    #[strum(serialize = "Hilo de coser (Hemming)")] Hemming(u32),
    #[strum(serialize = "Bordado (Embroidery)")] Embroidery(u32),
    #[strum(serialize = "Revestimiento elegante (Elegant Lining)")] ElegantLining(u32),
    #[strum(serialize = "Cera de dreugh (Dreugh Wax)")] DreughWax(u32)
}

#[derive(EnumString, Display, PartialEq, Eq, Hash)]
pub enum BlacksmithQualityMaterials {
    #[strum(serialize = "Piedra de esmeril (Honing Stone)")] HoningStone(u32),
    #[strum(serialize = "Aceite enano (Dwarven Oil)")] DwarvenOil(u32),
    #[strum(serialize = "Disolvente granulado (Grain Solvent)")] GrainSolvent(u32),
    #[strum(serialize = "Aleación de temple (Tempering Alloy)")] TemperingAlloy(u32)
}

#[derive(EnumString, Display, PartialEq, Eq, Hash)]
pub enum WoodworkingQualityMaterials {
    #[strum(serialize = "Brea (Pitch)")] Pitch(u32),
    #[strum(serialize = "Turpen")] Turpen(u32),
    #[strum(serialize = "Masilla (Mastic)")] Mastic(u32),
    #[strum(serialize = "Colofonia (Rosin)")] Rosin(u32)
}

#[derive(EnumString, Display, PartialEq, Eq, Hash)]
pub enum JewelryQualityMaterials {
    #[strum(serialize = "Chapado de terne (Terne Plating)")] TernePlating(u32),
    #[strum(serialize = "Chapado de iridio (Iridium Plating)")] IridiumPlating(u32),
    #[strum(serialize = "Chapado de circón (Zircon Plating)")] ZirconPlating(u32),
    #[strum(serialize = "Chapado de cromo (Chromium Plating)")] ChromiumPlating(u32)
}

#[derive(EnumString, Display, PartialEq, Hash, Eq)]
pub enum ArmourTraitMaterials {
    #[strum(serialize = "Zafiro (Sapphire)")] Sapphire,
    #[strum(serialize = "Diamante (Diamond)")] Diamond,
    #[strum(serialize = "Piedra de sangre (Bloodstone)")] Bloodstone,
    #[strum(serialize = "Granate (Garnet)")] Garnet,
    #[strum(serialize = "Nirncrux fortificado (Fortified Nirncrux)")] FortifiedNirncrux,
    #[strum(serialize = "Sardónice (Sardonyx)")] Sardonyx,
    #[strum(serialize = "Cuarzo (Quartz)")] Quartz,
    #[strum(serialize = "Esmeralda (Emerald)")] Emerald,
    #[strum(serialize = "Almandino (Almandine)")] Almandine
}

#[derive(EnumString, Display, PartialEq, Hash, Eq)]
pub enum WeaponTraitMaterials {
    #[strum(serialize = "Amatista (Amethyst)")] Amethyst,
    #[strum(serialize = "Citrina (Citrine)")] Citrine,
    #[strum(serialize = "Turquesa (Turquoise)")] Turquoise,
    Jade,
    #[strum(serialize = "Temple de Nirn (Potent Nirncrux)")] PotentNirncrux,
    #[strum(serialize = "Crisolita (Chysolite)")] Chysolite,
    #[strum(serialize = "Rubí (Ruby)")] Ruby,
    #[strum(serialize = "Ópalo de fuego (Fire Opal)")] FireOpal,
    #[strum(serialize = "Cornalina (Carnelian)")] Carnelian
}

#[derive(EnumString, Display, PartialEq, Hash, Eq)]
pub enum JewelryTraitMaterials {
    #[strum(serialize = "Cobalto (Cobalt)")] Cobalt,
    #[strum(serialize = "Piedra masacre (Slaughterstone)")] Slaughterstone,
    #[strum(serialize = "Dibelio (Dibellium)")] Dibellium,
    #[strum(serialize = "Antimonio (Antimony)")] Antimony,
    #[strum(serialize = "Ámbar aúrbico (AurbicAmber)")] AurbicAmber,
    #[strum(serialize = "Titanio (Titanium)")] Titanium,
    #[strum(serialize = "Cinc (Zinc)")] Zinc,
    #[strum(serialize = "Cera dorada (Gilding Wax)")] GildingWax,
    #[strum(serialize = "Prisma del alba (Dawn Prism)")] DawnPrism
}

#[derive(EnumString, Display)]
pub enum PotencyRunes {
    Repora, Itade
}

#[derive(EnumString, Display)]
pub enum EssenceRunes {
    Dekeipa, Deni, Denima, Deteri, Hakeijo, Haoko, Indeko, Kaderi, Kuoko,
    Makderi, Makko, Makkoma, Meip, Oko, Okoma, Okori, Oru, Rakeipa, Taderi
}

pub trait ExtractAmount {
    fn get_amount(&self) -> u32;
}

impl Display for QualityMaterials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let format = match self {
            QualityMaterials::Tailoring(n) => n.to_string(),
            QualityMaterials::Blacksmith(n) => n.to_string(),
            QualityMaterials::Woodworking(n) => n.to_string(),
            QualityMaterials::Jewelry(n) => n.to_string(),
        };

        write!(f, "{format}")
    }
}

impl ExtractAmount for QualityMaterials {
    fn get_amount(&self) -> u32 {
        match self {
            QualityMaterials::Tailoring(t) => t.get_amount(),
            QualityMaterials::Blacksmith(b) => b.get_amount(),
            QualityMaterials::Woodworking(w) => w.get_amount(),
            QualityMaterials::Jewelry(j) => j.get_amount(),
        }
    }
}

impl ExtractAmount for PartMaterials {
    fn get_amount(&self) -> u32 {
        match *self {
            PartMaterials::AncestorSilk(n) => n,
            PartMaterials::RubedoLeather(n) => n,
            PartMaterials::RubediteIngots(n) => n,
            PartMaterials::SandedRubyAsh(n) => n,
            PartMaterials::PlatinumOunces(n) => n,
        }
    }
}

impl ExtractAmount for TailoringQualityMaterials {
    fn get_amount(&self) -> u32 {
        match *self {
            TailoringQualityMaterials::Hemming(n) => n,
            TailoringQualityMaterials::Embroidery(n) => n,
            TailoringQualityMaterials::ElegantLining(n) => n,
            TailoringQualityMaterials::DreughWax(n) => n,
        }
    }
}

impl ExtractAmount for WoodworkingQualityMaterials {
    fn get_amount(&self) -> u32 {
        match *self {
            WoodworkingQualityMaterials::Pitch(n) => n,
            WoodworkingQualityMaterials::Turpen(n) => n,
            WoodworkingQualityMaterials::Mastic(n) => n,
            WoodworkingQualityMaterials::Rosin(n) => n,
        }
    }
}

impl ExtractAmount for BlacksmithQualityMaterials {
    fn get_amount(&self) -> u32 {
        match *self {
            BlacksmithQualityMaterials::HoningStone(n) => n,
            BlacksmithQualityMaterials::DwarvenOil(n) => n,
            BlacksmithQualityMaterials::GrainSolvent(n) => n,
            BlacksmithQualityMaterials::TemperingAlloy(n) => n,
        }
    }
}

impl ExtractAmount for JewelryQualityMaterials {
    fn get_amount(&self) -> u32 {
        match *self {
            JewelryQualityMaterials::TernePlating(n) => n,
            JewelryQualityMaterials::IridiumPlating(n) => n,
            JewelryQualityMaterials::ZirconPlating(n) => n,
            JewelryQualityMaterials::ChromiumPlating(n) => n,
        }
    }
}

