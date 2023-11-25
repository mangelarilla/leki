use chrono::{Datelike, DateTime, Timelike, Utc};
use serenity::builder::{CreateComponents};
use serenity::client::Context;
use serenity::model::application::component::{ActionRow, ButtonStyle};
use serenity::model::application::component::ActionRowComponent::InputText;
use serenity::model::channel::{MessageType, ReactionType};
use serenity::model::id::{EmojiId};
use serenity::model::prelude::{InteractionResponseType, Message};
use serenity::model::prelude::modal::ModalSubmitInteraction;
use serenity::model::Timestamp;
use serenity::model::prelude::ScheduledEventType;
use crate::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &ModalSubmitInteraction) -> Result<()> {
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
            if !guild_channel.name.contains(&day) {
                continue;
            }
            let messages = guild_channel.messages(&ctx.http, |b| b.limit(10)).await?
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
                let msg = guild_channel.send_message(&ctx.http, |m| m
                    .set_embed(event_embed(&data))
                    .set_components(event_components())
                ).await?;
                guild.create_scheduled_event(&ctx.http, |e| e
                    .kind(ScheduledEventType::Voice)
                    .name(&data.title)
                    .description(format!("https://discord.com/channels/{}/{}/{}\n{}", guild, guild_channel.id, msg.id, &data.description.unwrap_or("".to_string())))
                    .channel_id(guild_channels.values().find(|c| c.name.contains("evento-pve")).unwrap())
                    .start_time(Timestamp::from_unix_timestamp(next_date.timestamp()).unwrap())
                    .end_time(Timestamp::from_unix_timestamp(end_datetime.timestamp()).unwrap())
                ).await?;
            }
            posted = true;
        }
        if !posted {
            busy_days = format!("{busy_days}\nEventos del {} ya ocupados", &day);
        }
    }

    interaction.create_interaction_response(&ctx.http, |r| r
        .kind(InteractionResponseType::UpdateMessage)
        .interaction_response_data(|d| {
            if busy_days.is_empty() {
                d.embed(|e| e.description("Trial creada!"));
            } else {
                d.embed(|e| e.description(busy_days));
            }
            d.set_components(CreateComponents::default());
            d.ephemeral(true);
            d
        })
    ).await?;

    Ok(())
}

fn get_days_times(components: &Vec<ActionRow>) -> Vec<(String, String)> {
    components.iter()
        .filter_map(|row| {
            if let InputText(input) = row.components.get(0).unwrap() {
                Some((input.custom_id.trim().to_string(), input.value.trim().to_string()))
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

fn event_components() -> CreateComponents {
    let mut components = CreateComponents::default();
    components.create_action_row(|ar| ar
        .create_button(|b| b
            .custom_id("signup_tank")
            .label("Tank")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId(1154134006036713622), name: Some("tank".to_string())})
        )
        .create_button(|b| b
            .custom_id("signup_dd")
            .label("DD")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId(1154134731756150974), name: Some("dd".to_string())})
        )
        .create_button(|b| b
            .custom_id("signup_healer")
            .label("Healer")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::Custom { animated: false, id: EmojiId(1154134924153065544), name: Some("healer".to_string())})
        )
    );
    components.create_action_row(|ar| ar
        .create_button(|b| b
            .custom_id("signup_reserve")
            .label("Reserva")
            .style(ButtonStyle::Secondary)
            .emoji(ReactionType::Unicode("👋".to_string()))
        )
        .create_button(|b| b
            .custom_id("signup_absent")
            .label("Ausencia")
            .style(ButtonStyle::Secondary)
            .emoji(ReactionType::Unicode("❌".to_string()))
        )
    );
    components
}