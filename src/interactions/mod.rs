pub mod pipelines;

use std::path::{PathBuf};
use chrono::{DateTime, Utc};
use rand::prelude::IteratorRandom;
use serenity::all::{ChannelId, CreateAttachment, GuildId, Message, MessageId, MessageType, ScheduledEventType};
use serenity::builder::{CreateScheduledEvent, GetMessages};
use serenity::client::Context;
use serenity::model::Timestamp;
use crate::events::generic::models::EventGenericData;
use crate::events::{EventData, EventRole};
use crate::events::pvp::models::PvPData;
use crate::events::trials::models::TrialData;
use crate::interactions::pipelines::InteractionPipeline;
use crate::messages::events::{CreateEvent, DeleteEvent, EditEventRole, EventInfo, EventScope, NewEvent, PreviewComp, SelectDay, SelectRoster, SelectTime, SignupEvent, UpdateComp};
use crate::messages::events::edit_event::EditEvent;
use crate::prelude::*;

const NEW_TRIAL: &'static str = "new_trial";
const NEW_PVP: &'static str = "new_pvp";
const NEW_GENERIC: &'static str = "new_generic";
pub(crate) fn define_pipeline() -> InteractionPipeline {
    let mut pipeline = InteractionPipeline::new();

    pipeline.interaction("events", NewEvent::new(NEW_TRIAL, NEW_PVP, NEW_GENERIC));

    define_event_pipeline::<TrialData>(&mut pipeline, NEW_TRIAL);
    define_event_pipeline::<PvPData>(&mut pipeline, NEW_PVP);
    define_event_pipeline::<EventGenericData>(&mut pipeline, NEW_GENERIC);

    pipeline.command_async("Delete event", DeleteEvent::new());
    pipeline.command("Edit event", EditEvent::new());

    pipeline
}

fn define_event_pipeline<T>(pipeline: &mut InteractionPipeline, trigger: &str)
    where T: EventData + Send + Sync + 'static,
          Error: From<<T as TryFrom<Message>>::Error>
{
    let event_info = T::prefix_id("event_info");
    let event_comp_confirm = T::prefix_id("event_comp_confirm");
    let event_comp_modify = T::prefix_id("event_comp_modify");
    let event_comp_info = T::prefix_id("event_comp_info");
    let event_public = T::prefix_id("event_public");
    let event_semi_public = T::prefix_id("event_semi_public");
    let event_private = T::prefix_id("event_private");
    let event_semi_public_confirm = T::prefix_id("event_semi_public_confirm");
    let event_private_confirm = T::prefix_id("event_private_confirm");
    let event_day = T::prefix_id("event_day");
    let event_time = T::prefix_id("event_time");

    let private_roster = SelectRoster::<T>::new(&event_private_confirm, "private", T::roles());
    for (id, handler) in private_roster.registries() {
        pipeline.message(id, handler);
    }

    let semi_public_roster = SelectRoster::<T>::new(&event_semi_public_confirm, "semi_public", T::roles());
    for (id, handler) in semi_public_roster.registries() {
        pipeline.message(id, handler);
    }

    pipeline
        .interaction(trigger, EventInfo::new(&event_info))
        .modal(event_info, PreviewComp::<T>::new(&event_comp_confirm, &event_comp_modify))
        .interaction(event_comp_modify, UpdateComp::<T>::new(&event_comp_info))
        .message(event_comp_confirm, EventScope::<T>::new(&event_public, &event_semi_public, &event_private))
        .modal(event_comp_info, EventScope::<T>::new(&event_public, &event_semi_public, &event_private))
        .interaction(event_semi_public, semi_public_roster)
        .interaction(event_private, private_roster)
        .message(event_public, SelectDay::<T>::new(&event_day, false))
        .message(event_semi_public_confirm, SelectDay::<T>::new(&event_day, false))
        .message(event_private_confirm, SelectDay::<T>::new(&event_day, true))
        .message_async(&event_day, SelectTime::new(&event_time))
        .message_async(event_time, CreateEvent::<T>::new())
    ;

    for role in T::roles() {
        let handler = SignupEvent::<T>::new(EventRole::Signed(role));
        let (flex_id, flex_handler) = handler.flex_registry();
        pipeline.message(flex_id, flex_handler);
        let (class_id, class_handler) = handler.class_registry();
        pipeline.message_async(class_id, class_handler);
        pipeline.message(T::prefix_id(role.to_id()), handler);

        pipeline.message(T::prefix_id(format!("edit_{}", role.to_id())), EditEventRole::<T>::new(EventRole::Signed(role)));
        pipeline.message_async(T::prefix_id(format!("edit_role_{}", role.to_id())), EditEventRole::<T>::new(EventRole::Signed(role)));
    }

    let handler = SignupEvent::<T>::new(EventRole::Reserve);
    let (flex_id, flex_handler) = handler.flex_registry();
    pipeline.message(flex_id, flex_handler);
    let (class_id, class_handler) = handler.class_registry();
    pipeline.message_async(class_id, class_handler);
    pipeline.message(T::prefix_id(EventRole::Reserve.to_id()), handler);
    pipeline.message(T::prefix_id(format!("edit_{}", EventRole::Reserve.to_id())), EditEventRole::<T>::new(EventRole::Reserve));
    pipeline.message_async(T::prefix_id(format!("edit_{}", EventRole::Reserve.to_id())), EditEventRole::<T>::new(EventRole::Reserve));

    pipeline.message(T::prefix_id(EventRole::Absent.to_id()), SignupEvent::<T>::new(EventRole::Absent));
}

pub(crate) async fn create_discord_event(guild: GuildId, ctx: &Context, data: &impl EventData, date: DateTime<Utc>, channel: ChannelId, msg: MessageId, is_pvp: bool) -> Result<()> {
    let duration: std::time::Duration = data.duration().into();
    let end_datetime = date + duration;
    guild.create_scheduled_event(&ctx.http, CreateScheduledEvent::new(ScheduledEventType::Voice, data.title(), Timestamp::from_unix_timestamp(date.timestamp()).unwrap())
        .description(format!("https://discord.com/channels/{}/{}/{}\n{}", guild, channel, msg, data.description()))
        .channel_id(if guild == 1134046249293717514 {1157232748604444683} else {
            if is_pvp {1144350647848812564} else {1144350408769286274}
        })
        .end_time(Timestamp::from_unix_timestamp(end_datetime.timestamp()).unwrap())
        .image(&get_image(is_pvp, ctx, data.title()).await?)
    ).await?;
    Ok(())
}

async fn get_image(is_pvp: bool, ctx: &Context, title: String) -> Result<CreateAttachment> {
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

fn guess_image(title: String) -> String {
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

pub async fn get_event_channel_messages(channel: ChannelId, ctx: &Context) -> Result<Vec<Message>> {
    let messages = channel.messages(&ctx.http, GetMessages::new()).await?
        .into_iter()
        .filter(|msg| !msg.pinned && msg.kind != MessageType::PinsAdd)
        .collect::<Vec<Message>>();
    Ok(messages)
}



