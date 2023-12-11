use chrono::{Datelike, DateTime, Timelike, Utc};
use serenity::all::{ActionRow, ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateScheduledEvent, GetMessages, Message, ModalInteraction};
use serenity::all::ActionRowComponent::InputText;
use serenity::client::Context;
use serenity::model::channel::{MessageType, ReactionType};
use serenity::model::id::{EmojiId};
use serenity::model::Timestamp;
use serenity::model::prelude::ScheduledEventType;
use crate::events::trials::models::parse_trial_data;
use crate::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &ModalInteraction) -> Result<()> {
    let days_times = get_days_times(&interaction.data.components);

    let mut busy_days = String::from("");
    for (day, time) in days_times {
        let next_date = calculate_next_date(&day)
            // hack for spanish timezone
            .with_hour((&time[..2]).parse::<u32>()? - 1).unwrap()
            .with_minute((&time[3..]).parse::<u32>()?).unwrap();

        let guild = interaction.guild_id.unwrap();
        let guild_channels = guild.channels(&ctx.http).await?;

        let mut posted = false;
        for guild_channel in guild_channels.values() {
            let channel_no_accents = unidecode::unidecode(guild_channel.name());
            tracing::info!("Unidecoded: {}", channel_no_accents);
            if !channel_no_accents.contains(&day) {
                continue;
            }
            let messages = guild_channel.messages(&ctx.http, GetMessages::new().limit(10)).await?
                .into_iter()
                .filter(|msg| !msg.pinned && msg.kind != MessageType::PinsAdd)
                .collect::<Vec<Message>>();
            if messages.len() > 0 {
                continue;
            }

            if !posted {
                let mut data = parse_trial_data(&interaction.message.clone().unwrap())?;
                let duration: std::time::Duration = data.duration.into();
                let end_datetime = next_date + duration;
                data.datetime = Some(next_date.clone());
                let msg = guild_channel.send_message(&ctx.http, CreateMessage::new()
                    .embed(event_embed(&data))
                    .components(event_components())
                ).await?;
                guild.create_scheduled_event(&ctx.http, CreateScheduledEvent::new(ScheduledEventType::Voice, &data.title, Timestamp::from_unix_timestamp(next_date.timestamp()).unwrap())
                    .description(format!("https://discord.com/channels/{}/{}/{}\n{}", guild, guild_channel.id, msg.id, &data.description.unwrap_or("".to_string())))
                    .channel_id(guild_channels.values().find(|c| c.name.contains("evento-pve")).unwrap())
                    .end_time(Timestamp::from_unix_timestamp(end_datetime.timestamp()).unwrap())
                ).await?;
            }
            posted = true;
        }
        if !posted {
            busy_days = format!("{busy_days}\nEventos del {} ya ocupados", &day);
        }
    }

    interaction.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new().description(if busy_days.is_empty() {"Trial creada!"} else {busy_days.as_str()}))
            .components(vec![])
            .ephemeral(true)
    )).await?;

    Ok(())
}

fn get_days_times(components: &Vec<ActionRow>) -> Vec<(String, String)> {
    components.iter()
        .filter_map(|row| {
            if let InputText(input) = row.components.get(0).unwrap() {
                Some((input.custom_id.trim().to_string(), input.value.as_ref().unwrap_or(&"".to_string()).trim().to_string()))
            } else {
                None
            }
        }).collect()
}

fn calculate_next_date(day: &str) -> DateTime<Utc> {
    let now = Utc::now();
    let now_diff_monday = now.weekday().num_days_from_monday();

    let target_diff_monday = to_weekday(day).unwrap().num_days_from_monday();
    let next_target = if target_diff_monday > now_diff_monday {
        target_diff_monday - now_diff_monday
    } else if target_diff_monday == now_diff_monday {
        0
    } else {
        now_diff_monday + target_diff_monday + 1
    };
    now + chrono::Duration::days(next_target.into())
}

fn event_components() -> Vec<CreateActionRow> {
    let class_row = CreateActionRow::Buttons(vec![
        CreateButton::new("signup_tank")
            .label("Tank")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134006036713622), name: Some("tank".to_string())}),
        CreateButton::new("signup_dd")
            .label("DD")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134731756150974), name: Some("dd".to_string())}),
        CreateButton::new("signup_healer")
            .label("Healer")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId::new(1154134924153065544), name: Some("healer".to_string())})
    ]);

    let backup_row = CreateActionRow::Buttons(vec![
        CreateButton::new("signup_reserve")
            .label("Reserva")
            .style(ButtonStyle::Secondary)
            .emoji(ReactionType::Unicode("👋".to_string())),
        CreateButton::new("signup_absent")
            .label("Ausencia")
            .style(ButtonStyle::Secondary)
            .emoji(ReactionType::Unicode("❌".to_string()))
    ]);

    vec![class_row, backup_row]
}