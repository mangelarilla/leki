use chrono::{Datelike, DateTime, Duration, FixedOffset, Timelike, Utc, Weekday};
use duration_string::DurationString;
use serenity::builder::{CreateComponents, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::component::{ActionRow, ButtonStyle};
use serenity::model::application::component::ActionRowComponent::InputText;
use serenity::model::channel::{GuildChannel, ReactionType};
use serenity::model::id::{ChannelId, EmojiId, UserId};
use serenity::model::mention::Mention;
use serenity::model::prelude::modal::ModalSubmitInteraction;
use serenity::utils::Colour;
use crate::prelude::*;

pub(crate) async fn handle(ctx: &Context, interaction: &ModalSubmitInteraction) -> Result<()> {
    let days_times = get_days_times(&interaction.data.components);

    for (day, time) in days_times {
        let next_date = calculate_next_date(&day)
            .with_hour((&time[..2]).parse::<u32>()?).unwrap()
            .with_minute((&time[3..]).parse::<u32>()?).unwrap();

        let guild = interaction.guild_id.unwrap();
        let guild_channels = guild.channels(&ctx.http).await?;

        for guild_channel in guild_channels.values() {
            if !guild_channel.name.contains(&day) {
                continue;
            }
            let messages = guild_channel.messages(&ctx.http, |b| b.limit(2)).await.unwrap();
            if messages.len() > 1 {
                continue;
            }

            let data = &interaction.message;
            tracing::info!("{data:#?}");
            // guild_channel.send_message(&ctx.http, |m| m
            //     .set_embed(event_embed(&title, &description, &next_date, duration, &interaction.user.id, &addons, &guides))
            //     .set_components(event_components())
            // )
        }
    }

    Ok(())
}

pub fn get_days_times(components: &Vec<ActionRow>) -> Vec<(String, String)> {
    components.iter()
        .filter_map(|row| {
            if let InputText(input) = row.components.get(0).unwrap() {
                Some((input.custom_id.trim().to_string(), input.value.trim().to_string()))
            } else {
                None
            }
        }).collect()
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

fn calculate_next_date(day: &str) -> DateTime<FixedOffset> {
    let now = Utc::now().with_timezone(&FixedOffset::east_opt(2 * 3600).unwrap());
    let now_diff_monday = now.weekday().num_days_from_monday();

    let target_diff_monday = to_weekday(day).unwrap().num_days_from_monday();
    let next_target = if target_diff_monday > now_diff_monday {
        target_diff_monday - now_diff_monday
    } else {
        now_diff_monday + target_diff_monday + 1
    };
    now + Duration::days(next_target.into())
}

fn event_embed(
    title: &str,
    description: &str,
    timestamp: &DateTime<FixedOffset>,
    duration: DurationString,
    leader: &UserId,
    addons: &str,
    guides: &str
) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title(title);
    embed.description(description);
    embed.field(":date: Fecha y Hora:", format!("<t:{}:f>", timestamp.timestamp()), true);
    embed.field(":hourglass_flowing_sand: Duración", duration, true);
    embed.field(":crown: Lider", Mention::User(*leader), true);
    embed.field("Guias:", addons, false);
    embed.field("AddOns recomendados:", guides, false);
    embed.field("", "\u{200b}", false);
    embed.field("", "\u{200b}", false);
    embed.field("<:tank:1154134006036713622> Tanks (1/2)", "└<:necro:1154088177796137030> polerokfi", false);
    embed.field("<:dd:1154134731756150974> DD (3/8)", "└<:arcanist:1154134563392606218> polemacho\n└<:arcanist:1154134563392606218> superpole\n└<:arcanist:1154134563392606218> polejon", false);
    embed.field("<:healer:1154134924153065544> Healers (2/2)", "└<:warden:1154134387546398720> poleheal\n└<:warden:1154134387546398720> ultrapole", false);
    embed.field(":wave: Reservas (0)", "", false);
    embed.field(":x: Ausencias (0)", "", false);
    embed.field("", "\u{200b}", false);
    embed.field("", "[Calendario (.ics)](https://skiny.com)", false);
    embed.thumbnail("https://images.uesp.net/2/26/ON-mapicon-SoloTrial.png");
    embed.timestamp("2017-01-03T23:00:00Z");
    embed.color(Colour::from_rgb(0, 255, 0));
    embed.footer(|f| f.text("Creado por polerokfi"));
    embed
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