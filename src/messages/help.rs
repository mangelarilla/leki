use serenity::all::{CommandInteraction, Context, CreateEmbed, CreateEmbedAuthor, CreateInteractionResponse, CreateInteractionResponseMessage};
use shuttle_persist::PersistInstance;
use crate::messages::BotInteractionMessage;

pub struct Help;

#[shuttle_runtime::async_trait]
impl BotInteractionMessage for Help {
    async fn command(&self, _interaction: &CommandInteraction, _ctx: &Context, _store: &PersistInstance) -> crate::prelude::Result<CreateInteractionResponse> {
        Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .embed(CreateEmbed::new()
                    .image("https://eso-hub.com/storage/headers/sets-overview-page-banner-image-header-g-pcsz-x.jpg")
                    .thumbnail("https://static.wikia.nocookie.net/elder-scrolls-fanon/images/6/61/Leki.jpg")
                    .colour((152, 8, 8))
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
        ))
    }
}