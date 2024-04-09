use strum::{Display, EnumString};

#[derive(EnumString, Display)]
pub enum PartMaterials {
    #[strum(serialize = "Seda ancestral (Ancestor Silk)")] AncestorSilk,
    #[strum(serialize = "Cuero rubedo (Rubedo Leather)")] RubedoLeather,
    #[strum(serialize = "Lingote de rubedita (Rubedite Ingots)")] RubediteIngots,
    #[strum(serialize = "Madera de fresno rubí lijado (Sanded Ruby Ash)")] SandedRubyAsh,
    #[strum(serialize = "Onza de platino (Platinum Ounces)")] PlatinumOunces
}

#[derive(EnumString, Display)]
pub enum RuneQualityMaterials {
    Ta, Jejota, Denata, Rekuta, Kuta
}

#[derive(EnumString, Display)]
pub enum TailoringQualityMaterials {
    #[strum(serialize = "Hilo de coser (Hemming)")] Hemming,
    #[strum(serialize = "Bordado (Embroidery)")] Embroidery,
    #[strum(serialize = "Revestimiento elegante (Elegant Lining)")] ElegantLining,
    #[strum(serialize = "Cera de dreugh (Dreugh Wax)")] DreughWax
}

#[derive(EnumString, Display)]
pub enum BlacksmithQualityMaterials {
    #[strum(serialize = "Piedra de esmeril (Honing Stone)")] HoningStone,
    #[strum(serialize = "Aceite enano (Dwarven Oil)")] DwarvenOil,
    #[strum(serialize = "Disolvente granulado (Grain Solvent)")] GrainSolvent,
    #[strum(serialize = "Aleación de temple (Tempering Alloy)")] TemperingAlloy
}

#[derive(EnumString, Display)]
pub enum WoodworkingQualityMaterials {
    #[strum(serialize = "Brea (Pitch)")] Pitch,
    #[strum(serialize = "Turpen")] Turpen,
    #[strum(serialize = "Masilla (Mastic)")] Mastic,
    #[strum(serialize = "Colofonia (Rosin)")] Rosin
}

#[derive(EnumString, Display)]
pub enum JewelryQualityMaterials {
    #[strum(serialize = "Chapado de terne (Terne Plating)")] TernePlating,
    #[strum(serialize = "Chapado de iridio (Iridium Plating)")] IridiumPlating,
    #[strum(serialize = "Chapado de circón (Zircon Plating)")] ZirconPlating,
    #[strum(serialize = "Chapado de cromo (Chromium Plating)")] ChromiumPlating
}

#[derive(EnumString, Display)]
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

#[derive(EnumString, Display)]
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

#[derive(EnumString, Display)]
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

