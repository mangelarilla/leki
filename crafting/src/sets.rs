pub mod armor;
pub mod jewelry;

use std::fmt::{Display, Formatter};
use serenity::all::{AutocompleteChoice, ComponentInteraction, Context, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, Message, ReactionType};
use serenity::builder::CreateEmbed;
use strum::{EnumProperty, IntoEnumIterator};
use crate::entities::{GearQuality, MaterialCost};
use crate::entities::armour::ArmourParts;
use crate::entities::jewelry::Jewelries;
use crate::entities::traits::GearTraits;
use crate::entities::weapon::WeaponKind;
use crate::prelude::{enum_list_to_options, Error, get_selected_gear};

pub struct GearPiece<T: Display> {
    pub part: T,
    pub gear_trait: GearTraits
}

struct GearRequest {
    set: GearSet,
    quality: GearQuality,
    armour: Vec<GearPiece<ArmourParts>>,
    weapons: Vec<GearPiece<WeaponKind>>,
    jewelries: Vec<GearPiece<Jewelries>>
}

#[derive(Debug)]
pub struct GearSet {
    name: String,
    name_es: String,
}

pub trait SetEmbed {
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

impl<T: Display> Display for GearPiece<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", &self.part.to_string(), &self.gear_trait.to_string())
    }
}

impl TryFrom<String> for GearSet {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        gear_sets().into_iter()
            .find(|gs| gs.name == value)
            .ok_or(Error::InvalidGearSet(value))
    }
}

impl Display for GearSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {}", self.name, self.name_es)
    }
}

impl GearSet {
    fn new(name: &str, name_es: &str) -> Self {
        GearSet {name: name.to_string(), name_es: name_es.to_string()}
    }

    pub(crate) fn matches(&self, value: impl Into<String>) -> bool {
        let name = unidecode::unidecode(&self.name).to_lowercase();
        let name_es = unidecode::unidecode(&self.name_es).to_lowercase();

        let value: String = value.into();
        name.contains(&value.to_lowercase()) ||
            name_es.contains(&value.to_lowercase())
    }

    pub(crate) fn to_autocomplete_choice(&self) -> AutocompleteChoice {
        AutocompleteChoice::new(&self.name, self.name.to_string())
            .add_localized_name("es-ES", &self.name_es)
    }
}

pub fn request_menu() -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .button(CreateButton::new("crafting_armour_set").label("Armadura").emoji(ReactionType::Unicode("🛡️".to_string())))
        .button(CreateButton::new("crafting_weapon_set").label("Armas").emoji(ReactionType::Unicode("⚔️".to_string())))
        .button(CreateButton::new("crafting_jewelry_set").label("Joyeria").emoji(ReactionType::Unicode("💎".to_string())))
}

pub fn quality_options() -> CreateSelectMenu {
    let quality = CreateSelectMenuKind::String {
        options: GearQuality::iter()
            .map(|opt| CreateSelectMenuOption::new(opt.to_string(), opt.to_string())
                .emoji(ReactionType::Unicode(opt.get_str("Emoji").unwrap().to_string())))
            .collect()
    };

    CreateSelectMenu::new("armour_quality", quality)
        .placeholder("Calidad del set")
}

async fn select_trait(message: &Message, interaction: &ComponentInteraction, ctx: &Context, part: String, traits: Vec<GearTraits>) -> crate::prelude::Result<(ComponentInteraction, GearTraits)> {
    let armour_trait = CreateSelectMenuKind::String {options: enum_list_to_options::<GearTraits>(traits)};

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .select_menu(
                CreateSelectMenu::new("armour_trait", armour_trait)
                    .placeholder(format!("Rasgo para {part}"))
            )
    )).await?;

    let interaction = message.await_component_interaction(&ctx).await.ok_or(Error::Timeout)?;
    let selected = get_selected_gear::<GearTraits>(&interaction).pop().unwrap();
    Ok((interaction, selected))
}

pub fn gear_sets() -> Vec<GearSet> {
    vec![
        GearSet::new("Adept Rider", "Jinete Adepto"),
        GearSet::new("Aetherial Ascension", "Ascensión aetérica"),
        GearSet::new("Alessia's Bulwark", "Baluarte de Alessia"),
        GearSet::new("Ancient Dragonguard", "La Guardia del Dragón de antaño"),
        GearSet::new("Armor Master", "Maestro armero"),
        GearSet::new("Armor of the Seducer", "Armadura de la seductora"),
        GearSet::new("Ashen Grip", "Yugo de ceniza"),
        GearSet::new("Assassin's Guile", "Astucia de asesino"),
        GearSet::new("Chimera's Rebuke", "Reprensión de quimera"),
        GearSet::new("Claw of the Forest Wraith", "Garra del espectro del bosque"),
        GearSet::new("Clever Alchemist", "Alquimista astuto"),
        GearSet::new("Coldharbour's Favorite", "Elegido de Puerto Gélido"),
        GearSet::new("Critical Riposte", "Estocada crítica"),
        GearSet::new("Daedric Trickery", "Engaño daédrico"),
        GearSet::new("Daring Corsair", "Corsario aventurado"),
        GearSet::new("Dauntless Combatant", "Combatiente indómito"),
        GearSet::new("Deadlands Demolisher", "Demoledor de la Tierras Muertas"),
        GearSet::new("Death's Wind", "Viento de la muerte"),
        GearSet::new("Diamond's Victory", "Victoria del diamante"),
        GearSet::new("Dragon's Appetite", "Apetito de dragón"),
        GearSet::new("Druid's Braid", "Trenza del druida"),
        GearSet::new("Eternal Hunt", "Caza Eterna"),
        GearSet::new("Eyes of Mara", "Ojos de Mara"),
        GearSet::new("Fortified Brass", "Latón reforzado"),
        GearSet::new("Grave-Stake Collector", "Coleccionista de estacas sepulcrales"),
        GearSet::new("Heartland Conqueror", "Conquistador de las Tierras Centrales"),
        GearSet::new("Hist Bark", "Corteza del hist"),
        GearSet::new("Hist Whisperer", "Susurrador del hist"),
        GearSet::new("Hunding's Rage", "Rabia de Hunding"),
        GearSet::new("Innate Axiom", "Axioma innato"),
        GearSet::new("Iron Flask", "Frasco de hierro"),
        GearSet::new("Kagrenac's Hope", "Esperanza de Kagrenac"),
        GearSet::new("Kvatch Gladiator", "Gladiador de Kvatch"),
        GearSet::new("Law of Julianos", "Ley de Julianos"),
        GearSet::new("Legacy of Karth", "Legado de Karth"),
        GearSet::new("Magnus' Gift", "Don de Magnus"),
        GearSet::new("Mechanical Acuity", "Agudeza mecánica"),
        GearSet::new("Might of the Lost Legion", "Poder de la Legión Perdida"),
        GearSet::new("Morkuldin", "Morkuldin"),
        GearSet::new("Naga Shaman", "Chamán Naga"),
        GearSet::new("New Moon Acolyte", "Acólito de la Luna Nueva"),
        GearSet::new("Night Mother's Gaze", "Mirada de la Madre Noche"),
        GearSet::new("Night's Silence", "Silencio de la noche"),
        GearSet::new("Noble's Conquest", "Conquista del noble"),
        GearSet::new("Nocturnal's Favor", "Favor de Nocturnal"),
        GearSet::new("Oblivion's Foe", "Enemigo de Oblivion"),
        GearSet::new("Old Growth Brewer", "Artesano del bosque maduro"),
        GearSet::new("Order's Wrath", "Cólera de la Orden"),
        GearSet::new("Orgnum's Scales", "Escamas de Orgnum"),
        GearSet::new("Pelinal's Wrath", "Ira de Pelinal"),
        GearSet::new("Red Eagle's Fury", "Furia del Águila Roja"),
        GearSet::new("Redistributor", "Redistribuidor"),
        GearSet::new("Seeker Synthesis", "Síntesis de buscador"),
        GearSet::new("Senche-Raht's Grit", "Osadía del senche-raht"),
        GearSet::new("Serpent's Disdain", "Desdén de la serpiente"),
        GearSet::new("Shacklebreaker", "Rompecadenas"),
        GearSet::new("Shalidor's Curse", "Maldición de Shalidor"),
        GearSet::new("Shattered Fate", "Destino fragmentado"),
        GearSet::new("Sload's Semblance", "Semblanza del sload"),
        GearSet::new("Song of Lamae", "Canción de Lamae"),
        GearSet::new("Spectre's Eye", "Ojo de espectro"),
        GearSet::new("Spell Parasite", "Parásito arcano"),
        GearSet::new("Stuhn's Favor", "Favor de Stuhn"),
        GearSet::new("Tava's Favor", "Favor de Tava"),
        GearSet::new("Telvanni Efficiency", "Eficacia de los Telvanni"),
        GearSet::new("Torug's Pact", "Pacto de Torug"),
        GearSet::new("Trial by Fire", "Prueba de fuego"),
        GearSet::new("Twice-Born Star", "Estrella renacida"),
        GearSet::new("Twilight's Embrace", "Abrazo del crepúsculo"),
        GearSet::new("Unchained Aggressor", "Agresor desencadenado"),
        GearSet::new("Vampire's Kiss", "Beso vampírico"),
        GearSet::new("Varen's Legacy", "Legado de Varen"),
        GearSet::new("Vastarie's Tutelage", "Tutela de Vastarie"),
        GearSet::new("Way of the Arena", "Senda de la Arena"),
        GearSet::new("Whitestrake's Retribution", "Castigo de la Descarga Blanca"),
        GearSet::new("Willow's Path", "Sendero del sauce"),
        GearSet::new("Wretched Vitality", "Vitalidad precaria"),
    ]
}

