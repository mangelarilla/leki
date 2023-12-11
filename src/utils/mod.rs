use chrono::{DateTime, Utc, Weekday};
use duration_string::DurationString;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::all::ActionRowComponent::InputText;
use serenity::all::{ActionRow, Colour, CreateEmbedFooter};
use serenity::builder::CreateEmbed;
use serenity::model::id::UserId;
use serenity::model::prelude::{Message};
use serenity::model::Timestamp;
use crate::prelude::*;

pub fn get_text(components: &Vec<ActionRow>, idx: usize) -> String {
    let input_text = components.get(idx)
        .map(|row| row.components.get(0))
        .flatten().unwrap();

    if let InputText(input) = input_text {
        input.value.as_ref().unwrap_or(&"".to_string()).to_string()
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

pub fn to_weekday_localized(day: &Weekday) -> String {
    match day {
        Weekday::Mon => "lunes",
        Weekday::Tue => "martes",
        Weekday::Wed => "miercoles",
        Weekday::Thu => "jueves",
        Weekday::Fri => "viernes",
        Weekday::Sat => "sabado",
        Weekday::Sun => "domingo",
    }.to_string()
}

#[derive(Debug)]
pub struct TrialData {
    pub title: String,
    pub description: Option<String>,
    pub datetime: Option<DateTime<Utc>>,
    pub duration: DurationString,
    pub leader: String,
    pub guides: String,
    pub addons: String,
    pub tanks: Vec<(String, UserId)>,
    pub dds: Vec<(String, UserId)>,
    pub healers: Vec<(String, UserId)>,
    pub max_tanks: u8,
    pub max_dds: u8,
    pub max_healers: u8,
    pub reserves: Vec<UserId>,
    pub absents: Vec<UserId>,
}

pub fn parse_trial_data(preview: &Message) -> Result<TrialData> {
    let trial_embed = preview.embeds.first().unwrap();
    let fields = &trial_embed.fields;
    let datetime = fields.get(0).unwrap().value.clone()
        .replace("<t:", "")
        .replace(":f>", "")
        .parse::<i64>().ok();
    let tanks = fields.get(7).unwrap();
    let dds = fields.get(8).unwrap();
    let healers = fields.get(9).unwrap();
    let reserves = fields.get(10).unwrap();
    let absents = fields.get(11).unwrap();

    Ok(TrialData {
        title: trial_embed.title.clone().unwrap(),
        description: trial_embed.description.clone(),
        datetime: datetime.map(|dt| DateTime::from_timestamp(dt, 0).unwrap()),
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

fn parse_player(text: &str) -> UserId {
    let id = text.replace("└", "")
        .replace("<@", "")
        .replace(">", "")
        .trim()
        .parse::<u64>();

    UserId::new(id.unwrap())
}

fn parse_player_class(text: &str) -> (String, UserId) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^└(?P<class><:.+>)\s<@(?P<player>\d+)>").unwrap();
    }
    RE.captures(text).and_then(|cap| Option::from({
        (cap.name("class").map(|max| max.as_str().to_string()).unwrap(),
         cap.name("player").map(|max| UserId::new(max.as_str().parse::<u64>().unwrap())).unwrap())
    })).unwrap()
}

pub fn parse_event_link(text: &str) -> (u64, u64, u64) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^https:\/\/discord\.com\/channels\/(?P<guild>\d+)\/(?P<channel>\d+)\/(?P<msg>\d+)$").unwrap();
    }
    tracing::info!("Link: {}", text);
    RE.captures(text.lines().next().unwrap()).and_then(|cap| Option::from({
        (cap.name("guild").map(|max| max.as_str().parse::<u64>().unwrap()).unwrap(),
         cap.name("channel").map(|max| max.as_str().parse::<u64>().unwrap()).unwrap(),
         cap.name("msg").map(|max| max.as_str().parse::<u64>().unwrap()).unwrap()
        )
    })).unwrap()
}

pub fn event_embed(
    data: &TrialData,
) -> CreateEmbed {
    CreateEmbed::new()
        .title(&data.title)
        .description(if let Some(description) = &data.description {description.as_str()} else {""})
        .field(":date: Fecha y Hora:", if let Some(datetime) = data.datetime.clone() {
            format!("<t:{}:f>", datetime.timestamp())
        } else {"".to_string()}, true)
        .field(":hourglass_flowing_sand: Duración", &data.duration.to_string(), true)
        .field(":crown: Lider", &data.leader, true)
        .field("Guias:", &data.addons, false)
        .field("AddOns recomendados:", &data.guides, false)
        .field("", "\u{200b}", false)
        .field("", "\u{200b}", false)
        .field(
            format!("<:tank:1154134006036713622> Tanks ({}/{})", &data.tanks.len(), &data.max_tanks),
            &data.tanks.iter().map(|(class, player)| format!("└{class} <@{player}>")).collect::<Vec<String>>().join("\n"),
            false)
        .field(
            format!("<:dd:1154134731756150974> DD ({}/{})", &data.dds.len(), &data.max_dds),
            &data.dds.iter().map(|(class, player)| format!("└{class} <@{player}>")).collect::<Vec<String>>().join("\n"),
            false)
        .field(
            format!("<:healer:1154134924153065544> Healers ({}/{})", &data.healers.len(), &data.max_healers),
            &data.healers.iter().map(|(class, player)| format!("└{class} <@{player}>")).collect::<Vec<String>>().join("\n"),
            false)
        .field(
            format!(":wave: Reservas ({})", &data.reserves.len()),
            &data.reserves.iter().map(|player| format!("└ <@{player}>")).collect::<Vec<String>>().join("\n"),
            false)
        .field(
            format!(":x: Ausencias ({})", &data.absents.len()),
            &data.absents.iter().map(|player| format!("└ <@{player}>")).collect::<Vec<String>>().join("\n"),
            false)
        .field("", "\u{200b}", false)
        .field("", "[Calendario (.ics)](https://skiny.com)", false)
        .thumbnail("https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png")
        .timestamp(Timestamp::now())
        .footer(CreateEmbedFooter::new("Ultima modificacion:"))
        .color(Colour::from_rgb(0, 255, 0))
}

pub fn remove_from_all_roles(data: &mut TrialData, user: UserId) {
    remove_from_role(&mut data.tanks, user);
    remove_from_role(&mut data.dds, user);
    remove_from_role(&mut data.healers, user);
    remove_from_secondary(&mut data.reserves, user);
    remove_from_secondary(&mut data.absents, user);
}

fn remove_from_role(list: &mut Vec<(String, UserId)>, user: UserId) {
    let index = list.iter().position(|(_, player)| *player == user);
    if let Some(index) = index {
        list.remove(index);
    }
}

fn remove_from_secondary(list: &mut Vec<UserId>, user: UserId) {
    let index = list.iter().position(|player| *player == user);
    if let Some(index) = index {
        list.remove(index);
    }
}