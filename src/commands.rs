use serenity::all::{CacheHttp, CommandType, CreateCommand, GuildId};
use tracing::{error, info, info_span, Instrument};

pub async fn register_commands(http: impl CacheHttp, guild: GuildId) {
    let span = info_span!("register_commands");
    async move {
        register_command(&http, guild, CreateCommand::new("events")
            .description("Event management")
            .description_localized("es-ES", "Gestión de eventos")
        ).await;
        register_command(&http, guild, CreateCommand::new("Edit event")
            .name_localized("es-ES","Editar evento")
            .kind(CommandType::Message)
        ).await;
        register_command(&http, guild, CreateCommand::new("Delete event")
            .name_localized("es-ES","Eliminar evento")
            .kind(CommandType::Message)
        ).await;
        register_command(&http, guild, CreateCommand::new("help")
            .description("Como se usa Leki")
        ).await;
    }.instrument(span).await;
}

async fn register_command(http: impl CacheHttp, guild: GuildId, builder: CreateCommand) {
    let command = guild.create_command(http, builder).await;

    match command {
        Ok(command) => info!("Command '{}' registered", &command.name),
        Err(error) => error!("Error registering command: {}",  error)
    }
}