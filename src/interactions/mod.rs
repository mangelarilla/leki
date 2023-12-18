
mod new;
mod signup;

use chrono::{DateTime, Utc};
use serenity::all::{ChannelId, Colour, CommandInteraction, ComponentInteraction, CreateAttachment, CreateEmbedAuthor, GuildId, Message, MessageId, MessageType, ModalInteraction, ScheduledEventType};
use serenity::builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateScheduledEvent, GetMessages};
use serenity::client::Context;
use serenity::model::Timestamp;
use tracing::{error};
use crate::events::models::{EventBasicData, EventKind};
use crate::prelude::*;

pub(crate) async fn handle_commands(ctx: &Context, interaction: CommandInteraction) {
    let result = match interaction.data.name.as_str() {
        "events" => new::new_event_response(&interaction, ctx).await,
        // "Edit event" => todo!(),
        "Delete event" => {
            let message = interaction.data.resolved.messages.values().next().unwrap();
            if message.author.id != 1148032756899643412 {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                    .content("Eso no es un evento atontao!")
                    .ephemeral(true)
                )).await.unwrap();
            }
            else if let Ok(_) = EventKind::try_from(message.clone()) {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new())).await.unwrap();
                let channel_messages = get_event_channel_messages(message.channel_id, ctx).await.unwrap();
                message.channel_id.delete_messages(&ctx.http, channel_messages).await.unwrap();

                let guild = interaction.guild_id.unwrap();
                let events = guild.scheduled_events(&ctx.http, false).await.unwrap();
                for event in events {
                    if event.creator_id.unwrap() == 1148032756899643412 {
                        let (_, _, event_msg) = parse_event_link(&event.description.unwrap());
                        if MessageId::new(event_msg) == message.id {
                            guild.delete_scheduled_event(&ctx.http, event.id).await.unwrap();
                            crate::tasks::unset_reminder(&message.channel_id);
                        }
                    }
                }
            }
            Ok(())
        },
        "help" => {
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(CreateEmbed::new()
                        .image("https://eso-hub.com/storage/headers/sets-overview-page-banner-image-header-g-pcsz-x.jpg")
                        .thumbnail("https://static.wikia.nocookie.net/elder-scrolls-fanon/images/6/61/Leki.jpg")
                        .colour(Colour::new(980808))
                        .author(CreateEmbedAuthor::new("Poleyecto")
                            .url("https://github.com/mangelarilla/leki")
                            .icon_url("https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png"))
                        .title("Como usar Leki")
                        .description(r#"
**Para crear un evento**, se hace mediante el comando `/events`
**Para editar o borrar un evento**, usa el *menu contextual*, *haciendo clic derecho en el __mensaje del evento__ o en los tres puntos horizontales que se resaltan en el mensaje*, en el menu ve a la opcion de `Apps`, y alli apareceran las dos opciones para borrar o editar.
                        "#)
                        .field("Features", r#"
- Creacion de eventos PvP, Trials o generales
- Posibilidad de seleccionar multiples dias y horas
- La seleccion de dias se basa en el nombre del canal, (ej: contiene "lunes" en el nombre), por tanto no funciona en otros canales, y seleccionaria el siguiente "lunes" del calendario.
- Creacion de los eventos asociados en Discord automatica
- Seleccion automatica de la imagen del evento de Discord basada en el titulo
- Creacion de eventos (PvP o Trial) con rosters abiertos, semi-abiertos o cerrados
- Borrado de eventos con purga del canal incluida, excepto chinchetas
- Habilidad para apuntarse por roles, reserva o marcar ausencias en el evento
- Recordatorio en el canal del evento 30 minutos para los apuntados
- Generacion de un script de invitaciones in-game para el RL en el recordatorio
- Al finalizar o borrar el evento, manda un DM al RL para confirmar la purga del canal
                        "#, false)
                    )
            )).await.unwrap();
            Ok(())
        },
        _ => {
            error!("Command interaction '{}' not handled", &interaction.data.name);
            interaction.create_response(&ctx.http, not_implemented_response()).await.unwrap();
            Ok(())
        }
    };

    if let Err(why) = result {
        error!("Error at '{}': {why:#?}", &interaction.data.name);
        interaction.create_response(&ctx.http, error_response(error_msg(why))).await.unwrap();
    }
}

pub(crate) async fn handle_component(ctx: &Context, interaction: ComponentInteraction) {
    let result = match interaction.data.custom_id.as_str() {
        "delete_event" => {
            let embed = interaction.message.embeds.first().unwrap();
            let channel_id = embed.fields.get(0).unwrap().value.clone().parse::<u64>().unwrap();
            let channel = ChannelId::new(channel_id);
            let messages = get_event_channel_messages(channel, ctx).await.unwrap();
            channel.delete_messages(&ctx.http, messages).await.unwrap();
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                .content("Borrado!"))).await.unwrap();
            Ok(())
        },
        _ => {
            if interaction.data.custom_id.starts_with("new_") {
                new::handle_component(&interaction, ctx).await.unwrap();
                Ok(())
            } else if interaction.data.custom_id.starts_with("signup_") {
                signup::handle_component(&interaction, ctx).await.unwrap();
                Ok(())
            } else {
                error!("Component interaction '{}' not handled", &interaction.data.custom_id);
                interaction.create_response(&ctx.http, not_implemented_response()).await.unwrap();
                Ok(())
            }
        }
    };

    if let Err(why) = result {
        error!("Error at '{}': {why:#?}", &interaction.data.custom_id);
        interaction.create_response(&ctx.http, error_response(error_msg(why))).await.unwrap();
    }
}

pub(crate) async fn handle_modal(ctx: &Context, interaction: ModalInteraction) {
    if interaction.data.custom_id.starts_with("new_") {
        let result = new::handle_modal(&interaction, ctx).await;

        if let Err(why) = result {
            error!("Error at '{}': {why:?}", &interaction.data.custom_id);
        }
    }
}

async fn create_discord_event(guild: GuildId, ctx: &Context, data: &impl EventBasicData, date: DateTime<Utc>, channel: ChannelId, msg: MessageId, is_pvp: bool) -> Result<()> {
    let duration: std::time::Duration = data.duration().into();
    let end_datetime = date + duration;
    guild.create_scheduled_event(&ctx.http, CreateScheduledEvent::new(ScheduledEventType::Voice, data.title(), Timestamp::from_unix_timestamp(date.timestamp()).unwrap())
        .description(format!("https://discord.com/channels/{}/{}/{}\n{}", guild, channel, msg, data.description()))
        .channel_id(if guild == 1134046249293717514 {1157232748604444683} else {
            if is_pvp {1144350647848812564} else {1144350408769286274}
        })
        .end_time(Timestamp::from_unix_timestamp(end_datetime.timestamp()).unwrap())
        .image(&CreateAttachment::url(&ctx.http, &guess_image(data.title(), is_pvp)).await.unwrap())
    ).await?;
    Ok(())
}

fn guess_image(title: String, is_pvp: bool) -> String {
    let title = unidecode::unidecode(&title.to_lowercase());
    if is_pvp {
        "https://dottzgaming.com/wp-content/uploads/2019/03/pvp-header-eso-1.jpg".to_string()
    } else if title.contains("aa") || title.contains("aetherian") || title.contains("aeterico") {
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

async fn get_event_channel_messages(channel: ChannelId, ctx: &Context) -> Result<Vec<Message>> {
    let messages = channel.messages(&ctx.http, GetMessages::new()).await?
        .into_iter()
        .filter(|msg| !msg.pinned && msg.kind != MessageType::PinsAdd)
        .collect::<Vec<Message>>();
    Ok(messages)
}

fn error_msg(why: Error) -> &'static str {
    match why {
        Error::Timestamp(_) => "Te has inventado la fecha bro",
        Error::ParseInt(_) => "Te has inventado la hora bro",
        Error::DurationParse(_) => "Te has inventado la duracion, ejemplos validos: 1h, 2h30m",
        _ => "Wooops"
    }
}

fn not_implemented_response() -> CreateInteractionResponse {
    error_response("Estamos trabajando en ello :D")
}

fn error_response(msg: &str) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(msg)
            .ephemeral(true)
    )
}