use std::collections::{HashMap};
use std::fmt::Display;
use std::hash::Hash;
use serenity::all::CreateEmbed;
use strum::EnumProperty;
use crate::entities::armour::ArmourParts;
use crate::entities::GearQuality;
use crate::entities::jewelry::Jewelries;
use crate::entities::materials::{ArmourTraitMaterials, ExtractAmount, JewelryTraitMaterials, PartMaterials, QualityMaterials, WeaponTraitMaterials};
use crate::entities::weapon::Weapons;
use crate::sets::{GearPiece, GearSet};
use crate::prelude::*;

pub struct GearRequest {
    set: GearSet,
    quality: Option<GearQuality>,
    armour: Vec<GearPiece<ArmourParts>>,
    weapons: Vec<GearPiece<Weapons>>,
    jewelries: Vec<GearPiece<Jewelries>>
}

impl GearRequest {
    pub fn new(set: GearSet, ) -> Self {
        GearRequest {
            set, quality: None, armour: vec![], weapons: vec![], jewelries: vec![]
        }
    }

    pub fn with_quality(&mut self, quality: GearQuality) {
        self.quality = Some(quality);
    }

    pub fn set_weapons(&mut self, weapons: Vec<GearPiece<Weapons>>) {
        self.weapons = weapons;
    }

    pub fn set_armour(&mut self, armour: Vec<GearPiece<ArmourParts>>) {
        self.armour = armour;
    }

    pub fn set_jewelry(&mut self, jewelry: Vec<GearPiece<Jewelries>>) {
        self.jewelries = jewelry;
    }


    pub fn to_embed_preview(&self) -> CreateEmbed {
        let embed = CreateEmbed::new()
            .for_set(&self.set)
            .with_gear("Armadura", &self.armour)
            .with_gear("Joyeria", &self.jewelries)
            .with_gear("Armas", &self.weapons);

        if let Some(quality) = &self.quality {
            embed.with_quality(quality)
        } else {
            embed
        }
    }

    pub fn to_embed_cost(&self) -> Result<CreateEmbed> {
        let armor_cost: Vec<PartMaterials> = self.armour.iter()
            .map(|a| a.part.calculate_cost())
            .collect();
        let armor_trait_cost = self.armour.iter()
            .map(|a| a.gear_trait.armour_cost())
            .collect::<Result<Vec<ArmourTraitMaterials>>>()?
            .into_iter()
            .fold(HashMap::new(), |mut acc, mat| {
                let counter = acc.entry(mat).or_insert(0_u32);
                *counter += 1;
                acc
            });

        let weapon_cost: Vec<PartMaterials> = self.weapons.iter()
            .map(|a| a.part.calculate_cost())
            .collect();
        let weapon_trait_cost = self.weapons.iter()
            .map(|a| a.gear_trait.weapon_cost())
            .collect::<Result<Vec<WeaponTraitMaterials>>>()?
            .into_iter()
            .fold(HashMap::new(), |mut acc, mat| {
                let counter = acc.entry(mat).or_insert(0_u32);
                *counter += 1;
                acc
            });

        let jewelry_cost: Vec<PartMaterials> = self.jewelries.iter()
            .map(|a| a.part.calculate_cost())
            .collect();
        let jewelry_trait_cost = self.jewelries.iter()
            .map(|a| a.gear_trait.jewelry_cost())
            .collect::<Result<Vec<JewelryTraitMaterials>>>()?
            .into_iter()
            .fold(HashMap::new(), |mut acc, mat| {
                let counter = acc.entry(mat).or_insert(0_u32);
                *counter += 1;
                acc
            });

        let part_costs = to_hashmap(vec![armor_cost, weapon_cost, jewelry_cost]);

        let mut embed = CreateEmbed::new()
            .title("Materiales")
            .description("Coste de materiales")
            .field(
                ":construction_site: Materiales de Construccion",
                part_costs.into_iter().map(|(c, n)| format!("{} x{n}", c.to_string()))
                    .collect::<Vec<String>>().join("\n"),
                true
            );

        if !armor_trait_cost.is_empty() {
            embed = embed.field(
                ":shield: Rasgos de armadura",
                armor_trait_cost.into_iter().map(|(c, n)| format!("{} x{n}", c.to_string()))
                    .collect::<Vec<String>>().join("\n"),
                true
            );
        }

        if !jewelry_trait_cost.is_empty() {
            embed = embed.field(
                ":ring: Rasgos de joyeria",
                jewelry_trait_cost.into_iter().map(|(c, n)| format!("{} x{n}", c.to_string()))
                    .collect::<Vec<String>>().join("\n"),
                true
            )
        }

        if !weapon_trait_cost.is_empty() {
            embed = embed.field(
                ":crossed_swords: Rasgos de armas",
                weapon_trait_cost.into_iter().map(|(c, n)| format!("{} x{n}", c.to_string()))
                    .collect::<Vec<String>>().join("\n"),
                true
            )
        }

        if let Some(quality) = &self.quality {
            let armor_quality_cost: Vec<QualityMaterials> = self.armour.iter()
                .map(|a| a.part.calculate_quality_cost(quality))
                .flatten()
                .collect();
            let weapon_quality_cost: Vec<QualityMaterials> = self.weapons.iter()
                .map(|a| a.part.calculate_quality_cost(quality))
                .flatten()
                .collect();
            let jewelry_quality_cost: Vec<QualityMaterials> = self.jewelries.iter()
                .map(|a| a.part.calculate_quality_cost(quality))
                .flatten()
                .collect();

            let quality_cost = to_hashmap(vec![armor_quality_cost, weapon_quality_cost, jewelry_quality_cost]);
            embed = embed.field(
                ":gem: Mejoras",
                quality_cost.into_iter().map(|(c, n)| format!("{} x{n}", c.to_string()))
                    .collect::<Vec<String>>().join("\n"),
                true
            );
        }

        Ok(embed)
    }
}

fn to_hashmap<T: ExtractAmount + Eq + Hash>(src: Vec<Vec<T>>) -> HashMap<T, u32> {
    src.into_iter()
        .flatten()
        .fold(HashMap::new(), |mut acc, quality| {
            let amount = quality.get_amount();
            let counter = acc.entry(quality).or_insert(0);
            *counter += amount;
            acc
        })
}

trait SetEmbed {
    fn for_set(&self, set: &GearSet) -> Self;
    fn with_quality(&self, quality: &GearQuality) -> Self;
    fn with_gear<T: Display>(&self, label: &str, parts: &Vec<GearPiece<T>>) -> Self;
}

impl SetEmbed for CreateEmbed {
    fn for_set(&self, gear_set: &GearSet) -> Self {
        self.clone()
            .title(format!("Set: {gear_set}"))
            .description("Configura la peticion de equipo")
    }

    fn with_quality(&self, quality: &GearQuality) -> Self {
        self.clone()
            .field(
                "Calidad del set",
                format!("{} {}", quality.get_str("Emoji").unwrap(), quality.to_string()),
                false
            )
    }

    fn with_gear<T: Display>(&self, label: &str, parts: &Vec<GearPiece<T>>) -> Self {
        if parts.is_empty() {
            self.clone()
        } else {
            self.clone()
                .field(
                    label,
                    parts
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join("\n"),
                    false
                )
        }
    }
}