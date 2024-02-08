use std::path::PathBuf;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use rand::prelude::IteratorRandom;
use serenity::all::{ChannelId, ComponentInteraction, Context, CreateActionRow, CreateAttachment, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateScheduledEvent, GuildId, Mention, MessageId, ScheduledEventId, ScheduledEventType, Timestamp};
use shuttle_persist::PersistInstance;
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
            let handler = SignupEvent::new(role, kind);
            pipeline.add(role.to_id(), handler.clone());
            pipeline.add(handler.class_id(), handler.clone());
            pipeline.add(handler.flex_id(), handler.clone());
        }

        CreateEvent { kind }
    }
}

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for CreateEvent {
    async fn component(&self, interaction: &ComponentInteraction, ctx: &Context, store: &PersistInstance) -> Result<CreateInteractionResponse> {
        let mut event = store.load::<Event>(interaction.message.id.to_string().as_str())?;
        let (channel, next_date) = get_date_time(&interaction.data).unwrap();
        let guild = interaction.guild_id.unwrap();

        event.datetime = Some(next_date.clone());

        let mut components = vec![];
        if event.scope != EventScopes::Private {
            components.push(CreateActionRow::Buttons(self.kind.roles()
                .into_iter()
                .filter_map(|r| match r {
                    EventRole::Reserve | EventRole::Absent => None,
                    _ => Some(r.to_button(r.to_id(), r.to_string()))
                }).collect()));
        };

        components.push(CreateActionRow::Buttons(vec![
            EventRole::Reserve.to_button(EventRole::Reserve.to_id(), EventRole::Reserve.to_string()),
            EventRole::Absent.to_button(EventRole::Absent.to_id(), EventRole::Absent.to_string())
        ]));

        let msg = channel.send_message(&ctx.http, CreateMessage::new()
            .embed(event.embed())
            .components(components)
        ).await?;

        event.scheduled_event = Some(create_discord_event(guild, ctx, &event, next_date.clone(), channel, msg.id).await?);
        tasks::set_reminder(next_date.clone(), Arc::new(ctx.clone()), channel, msg.id, Arc::new(store.clone()));

        store.save(msg.id.to_string().as_str(), event)?;
        store.remove(interaction.message.id.to_string().as_str())?;

        Ok(CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .add_embed(CreateEmbed::new().title("Nuevo evento!").description(format!("Evento creado en {}", Mention::Channel(channel).to_string())))
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