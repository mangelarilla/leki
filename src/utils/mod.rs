use chrono::{Weekday};
use duration_string::DurationString;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::builder::CreateEmbed;
use serenity::model::application::component::ActionRowComponent::InputText;
use serenity::model::prelude::component::ActionRow;
use serenity::model::prelude::Message;
use serenity::model::Timestamp;
use serenity::utils::Colour;
use crate::prelude::*;

pub fn get_text(components: &Vec<ActionRow>, idx: usize) -> String {
    let input_text = components.get(idx)
        .map(|row| row.components.get(0))
        .flatten().unwrap();

    if let InputText(input) = input_text {
        input.value.to_string()
    } else {
        String::new()
    }
}

pub fn to_weekday(day: &str) -> Option<Weekday> {
    match day {
        "lunes" => Some(Weekday::Mon),
        "martes"=> Some(Weekday::Tue),
        "miercoles" => Some(Weekday::Wed),
        "jueves" => Some(Weekday::Thu),
        "viernes" => Some(Weekday::Fri),
        "sabado" => Some(Weekday::Sat),
        "domingo" => Some(Weekday::Sun),
        _ => None
    }
}

#[derive(Debug)]
pub struct TrialData {
    pub title: String,
    pub description: Option<String>,
    pub datetime: Option<String>,
    pub duration: DurationString,
    pub leader: String,
    pub guides: String,
    pub addons: String,
    pub tanks: Vec<(String, String)>,
    pub dds: Vec<(String, String)>,
    pub healers: Vec<(String, String)>,
    pub max_tanks: u8,
    pub max_dds: u8,
    pub max_healers: u8,
    pub reserves: Vec<String>,
    pub absents: Vec<String>,
}

pub fn parse_trial_data(preview: &Message) -> Result<TrialData> {
    let trial_embed = preview.embeds.first().unwrap();
    let fields = &trial_embed.fields;
    let datetime = fields.get(0).unwrap().value.clone();
    let tanks = fields.get(7).unwrap();
    let dds = fields.get(8).unwrap();
    let healers = fields.get(9).unwrap();
    let reserves = fields.get(10).unwrap();
    let absents = fields.get(11).unwrap();

    Ok(TrialData {
        title: trial_embed.title.clone().unwrap(),
        description: trial_embed.description.clone(),
        datetime: if datetime.is_empty() {None} else {Some(datetime)},
        duration: fields.get(1).unwrap().value.parse::<DurationString>().map_err(anyhow::Error::msg)?,
        leader: fields.get(2).unwrap().value.clone(),
        guides: fields.get(3).unwrap().value.clone(),
        addons: fields.get(4).unwrap().value.clone(),
        tanks: tanks.value.clone().lines().map(|s| parse_player_class(s)).collect(),
        max_tanks: get_max(&tanks.name).parse::<u8>()?,
        dds: dds.value.clone().lines().map(|s| parse_player_class(s)).collect(),
        max_dds: get_max(&dds.name).parse::<u8>()?,
        healers: healers.value.clone().lines().map(|s| parse_player_class(s)).collect(),
        max_healers: get_max(&healers.name).parse::<u8>()?,
        reserves: reserves.value.clone().lines().map(|s| parse_player(s)).collect(),
        absents: absents.value.clone().lines().map(|s| parse_player(s)).collect()
    })
}

fn get_max(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".+\/(?P<max>\d+)").unwrap();
    }
    RE.captures(text).and_then(|cap| {
        cap.name("max").map(|max| max.as_str().to_string())
    }).unwrap()
}

fn parse_player(text: &str) -> String {
    text.replace("└", "").trim().to_string()
}

fn parse_player_class(text: &str) -> (String, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^└(?P<class><:.+>)\s(?P<player>.+)").unwrap();
    }
    RE.captures(text).and_then(|cap| Option::from({
        (cap.name("class").map(|max| max.as_str().to_string()).unwrap(),
         cap.name("player").map(|max| max.as_str().to_string()).unwrap())
    })).unwrap()
}

pub fn event_embed(
    data: &TrialData,
) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title(&data.title);
    if let Some(description) = &data.description {
        embed.description(description);
    }
    embed.field(":date: Fecha y Hora:", &data.datetime.clone().unwrap_or("".to_string()), true);
    embed.field(":hourglass_flowing_sand: Duración", &data.duration, true);
    embed.field(":crown: Lider", &data.leader, true);
    embed.field("Guias:", &data.addons, false);
    embed.field("AddOns recomendados:", &data.guides, false);
    embed.field("", "\u{200b}", false);
    embed.field("", "\u{200b}", false);
    embed.field(
        format!("<:tank:1154134006036713622> Tanks ({}/{})", &data.tanks.len(), &data.max_tanks),
        &data.tanks.iter().map(|(class, player)| format!("└{class} {player}")).collect::<Vec<String>>().join("\n"),
        false
    );
    embed.field(
        format!("<:dd:1154134731756150974> DD ({}/{})", &data.dds.len(), &data.max_dds),
        &data.dds.iter().map(|(class, player)| format!("└{class} {player}")).collect::<Vec<String>>().join("\n"),
        false
    );
    embed.field(
        format!("<:healer:1154134924153065544> Healers ({}/{})", &data.healers.len(), &data.max_healers),
        &data.healers.iter().map(|(class, player)| format!("└{class} {player}")).collect::<Vec<String>>().join("\n"),
        false);
    embed.field(
        format!(":wave: Reservas ({})", &data.reserves.len()),
        &data.reserves.iter().map(|player| format!("└ {player}")).collect::<Vec<String>>().join("\n"),
        false);
    embed.field(
        format!(":x: Ausencias ({})", &data.absents.len()),
        &data.absents.iter().map(|player| format!("└ {player}")).collect::<Vec<String>>().join("\n"),
        false);
    embed.field("", "\u{200b}", false);
    embed.field("", "[Calendario (.ics)](https://skiny.com)", false);
    embed.thumbnail("https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png");
    embed.timestamp(Timestamp::now());
    embed.footer(|f| f.text("Ultima modificacion:"));
    embed.color(Colour::from_rgb(0, 255, 0));
    embed
}

pub fn remove_from_all_roles(data: &mut TrialData, name: &str) {
    remove_from_role(&mut data.tanks, &name);
    remove_from_role(&mut data.dds, &name);
    remove_from_role(&mut data.healers, &name);
    remove_from_secondary(&mut data.reserves, &name);
    remove_from_secondary(&mut data.absents, &name);
}

fn remove_from_role(list: &mut Vec<(String, String)>, user_name: &str) {
    let index = list.iter().position(|(_, player)| player.as_str() == user_name);
    if let Some(index) = index {
        list.remove(index);
    }
}

fn remove_from_secondary(list: &mut Vec<String>, user_name: &str) {
    let index = list.iter().position(|player| player.as_str() == user_name);
    if let Some(index) = index {
        list.remove(index);
    }
}