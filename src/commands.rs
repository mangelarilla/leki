use serenity::all::{CommandType, Context, CreateCommand, GuildId};
use tracing::{error, info, info_span, Instrument};

pub async fn register_commands(ctx: &Context, guild: GuildId) {
    let span = info_span!("register_commands");
    async move {
        register_command(ctx, guild, CreateCommand::new("events")
            .description("Event management")
            .description_localized("es-ES", "Gestión de eventos")
        ).await;
        register_command(ctx, guild, CreateCommand::new("Edit event")
            .name_localized("es-ES","Editar evento")
            .kind(CommandType::Message)
        ).await;
        register_command(ctx, guild, CreateCommand::new("Delete event")
            .name_localized("es-ES","Eliminar evento")
            .kind(CommandType::Message)
        ).await;
    }.instrument(span).await;
}

async fn register_command(ctx: &Context, guild: GuildId, builder: CreateCommand) {
    let command = guild.create_command(&ctx.http, builder).await;
    match command {
        Ok(command) => {
            info!("Command '{}' registered", &command.name)
        },
        Err(error) => error!("Error registering command: {}",  error)
    }
}