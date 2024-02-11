use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use chrono::{Datelike, DateTime, Timelike, Utc, Weekday};
use rand::prelude::IteratorRandom;
use serenity::all::{ChannelId, ComponentInteraction, ComponentInteractionData, ComponentInteractionDataKind, Context, CreateActionRow, CreateAttachment, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateScheduledEvent, GuildId, Mention, MessageId, ScheduledEventId, ScheduledEventType, Timestamp};
use crate::{tasks};
use crate::events::{Event, EventKind, EventRole, EventScopes};
use crate::interactions::pipelines::InteractionPipeline;
use crate::messages::BotInteractionMessage;
use crate::messages::events::SignupEvent;
use crate::prelude::*;

pub(crate) struct CreateEvent {
    kind: EventKind
}

impl CreateEvent {
    pub(crate) fn new(kind: EventKind, pipeline: &mut InteractionPipeline) -> Self {
        for role in kind.roles() {
            let handler = SignupEvent::new(role);
            pipeline.add(format!("signup_{}", role.to_id()), handler.clone());
            pipeline.add(handler.class_id(), handler.clone());
            pipeline.add(handler.flex_id(), handler.clone());
        }

        CreateEvent { kind }
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for CreateEvent {
    async fn component(&self, interaction: &ComponentInteraction, ctx: &Context, store: &Store) -> Result<CreateInteractionResponse> {
        let (channel, next_date) = get_date_time(&interaction.data).unwrap();
        let guild = interaction.guild_id.unwrap();

        store.update_datetime(interaction.message.id, next_date.clone()).await?;
        let event = store.get_event(interaction.message.id).await?;

        let mut components = vec![];
        if event.scope != EventScopes::Private {
            components.push(CreateActionRow::Buttons(self.kind.roles()
                .into_iter()
                .filter_map(|r| match r {
                    EventRole::Reserve | EventRole::Absent => None,
                    _ => Some(r.to_button(format!("signup_{}", r.to_id()), r.to_string()))
                }).collect()));
        };

        components.push(CreateActionRow::Buttons(vec![
            EventRole::Reserve.to_button(format!("signup_{}", EventRole::Reserve.to_id()), EventRole::Reserve.to_string()),
            EventRole::Absent.to_button(format!("signup_{}", EventRole::Absent.to_id()), EventRole::Absent.to_string())
        ]));

        let msg = channel.send_message(&ctx.http, CreateMessage::new()
            .embed(event.embed())
            .components(components)
        ).await?;

        store.update_id(interaction.message.id, msg.id).await?;
        store.update_discord_event(msg.id, create_discord_event(guild, ctx, &event, next_date.clone(), channel, msg.id).await?).await?;
        tasks::set_reminder(next_date.clone(), Arc::new(ctx.clone()), channel, msg.id, Arc::new(store.clone()));

        Ok(CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .embed(CreateEmbed::new().title("Nuevo evento!").description(format!("Evento creado en {}", Mention::Channel(channel).to_string())))
                .components(vec![])
        ))
    }
}

async fn create_discord_event(guild: GuildId, ctx: &Context, data: &Event, date: DateTime<Utc>, channel: ChannelId, msg: MessageId) -> Result<ScheduledEventId> {
    let duration: std::time::Duration = data.duration.into();
    let end_datetime = date + duration;
    let event = guild.create_scheduled_event(&ctx.http, CreateScheduledEvent::new(ScheduledEventType::Voice, &data.title, Timestamp::from_unix_timestamp(date.timestamp()).unwrap())
        .description(format!("https://discord.com/channels/{}/{}/{}\n{}", guild, channel, msg, data.description))
        .channel_id(if guild == 1134046249293717514 {1157232748604444683} else {
            if data.kind == EventKind::PvP {1144350647848812564} else {1144350408769286274}
        })
        .end_time(Timestamp::from_unix_timestamp(end_datetime.timestamp()).unwrap())
        .image(&get_image(data.kind == EventKind::PvP, ctx, &data.title).await?)
    ).await?;
    Ok(event.id)
}

async fn get_image(is_pvp: bool, ctx: &Context, title: &str) -> Result<CreateAttachment> {
    let attachment = if is_pvp {
        CreateAttachment::path(random_pvp_image()?).await?
    } else {
        CreateAttachment::url(&ctx.http, &guess_image(title)).await?
    };

    Ok(attachment)
}

fn random_pvp_image() -> Result<PathBuf> {
    let mut path = PathBuf::from("assets");
    path.push("pvp");

    let image = path.read_dir()?
        .filter_map(|f| f.ok())
        .choose(&mut rand::thread_rng())
        .unwrap();

    Ok(image.path())
}

fn guess_image(title: &str) -> String {
    let title = unidecode::unidecode(&title.to_lowercase());
    if title.contains("aa") || title.contains("aetherian") || title.contains("aeterico") {
        "https://images.uesp.net/thumb/f/fc/ON-load-Aetherian_Archive.jpg/1200px-ON-load-Aetherian_Archive.jpg".to_string()
    } else if title.contains("as") || title.contains("asylum") || title.contains("amparo") {
        "https://eso-hub.com/storage/headers/asylum-sanctorium-trial-e-s-o-header-yyye8-n.jpg".to_string()
    } else if title.contains("hrc") || title.contains("hel ra") || title.contains("helra") {
        "https://eso-hub.com/storage/headers/hel-ra-citadel-trial-e-s-o-header--f-kt-c3e.jpg".to_string()
    } else if title.contains("so") || title.contains("ophidia") || title.contains("sanctum") {
        "https://i.redd.it/nh0o94messq71.png".to_string()
    } else if title.contains("dsr") || title.contains("dreadsail") || title.contains("arrecife") {
        "https://eso-hub.com/storage/headers/dreadsail-reef-header-e-s-o-header--v1-u-s-t5.jpg".to_string()
    } else if title.contains("ss") || title.contains("sunspire") || title.contains("sol") {
        "https://www.universoeso.com.br/wp-content/uploads/2021/03/vssssssssssss.jpg".to_string()
    } else if title.contains("mol") || title.contains("maw") || title.contains("lorkhaj") {
        "https://esosslfiles-a.akamaihd.net/cms/2016/03/a2295f32b46ac88aed5edb06c1f94fc1.jpg".to_string()
    } else if title.contains("cr") || title.contains("cloudrest") || title.contains("nubelia") {
        "https://esosslfiles-a.akamaihd.net/cms/2018/05/85480dd6e0cdf59a1326c3fa188ec3fc.jpg".to_string()
    } else if title.contains("se") || title.contains("sanity") || title.contains("locura") {
        "https://esosslfiles-a.akamaihd.net/ape/uploads/2023/05/5ece21494783d382a25baf809807957d.jpg".to_string()
    } else if title.contains("hof") || title.contains("fabrication") || title.contains("fabricacion") {
        "https://images.uesp.net/thumb/5/51/ON-load-Halls_of_Fabrication.jpg/1200px-ON-load-Halls_of_Fabrication.jpg".to_string()
    } else if title.contains("ka") || title.contains("kyne") || title.contains("egida") {
        "https://esosslfiles-a.akamaihd.net/cms/2020/05/2c2bc79be7a47609fa7b594935f9df6d.jpg".to_string()
    } else {
        "https://esosslfiles-a.akamaihd.net/ape/uploads/2022/09/f96a76373bd8e0521609bf24e88acb03.jpg".to_string()
    }
}

fn get_date_time(data: &ComponentInteractionData) -> Option<(ChannelId, DateTime<Utc>)> {
    if let ComponentInteractionDataKind::StringSelect { values} = &data.kind {
        let time = values.first().unwrap();
        let (_, channel_day) = data.custom_id.split_once("__").unwrap();
        let (channel, day) = channel_day.split_once("_").unwrap();
        let hour = (&time[..2]).parse::<u32>().unwrap() - 1; // hack for spanish timezone
        let minute = (&time[3..]).parse::<u32>().unwrap();
        let dt = calculate_next_date(&day, hour, minute)
            .with_hour(hour).unwrap()
            .with_minute(minute).unwrap();
        let id = ChannelId::from_str(&channel).unwrap();
        Some((id, dt))
    } else { None }
}

fn calculate_next_date(day: &str, hour: u32, minute: u32) -> DateTime<Utc> {
    let now = Utc::now();

    let now_diff_monday = now.weekday().num_days_from_monday();
    let target_diff_monday = to_weekday(day).unwrap().num_days_from_monday();
    let next_target = if target_diff_monday > now_diff_monday {
        target_diff_monday - now_diff_monday
    } else if target_diff_monday == now_diff_monday {
        if now.hour() > hour || (now.hour() == hour && now.minute() > minute) { 7 } else { 0 }
    } else {
        target_diff_monday + (7 - now_diff_monday)
    };
    now + chrono::Duration::days(next_target.into())
}

fn to_weekday(day: &str) -> Option<Weekday> {
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