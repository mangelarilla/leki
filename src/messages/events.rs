pub(crate) mod new_event;
pub(crate) mod event_composition;
pub(crate) mod event_scope;
pub(crate) mod select_date;
pub(crate) mod create_event;
pub(crate) mod event_info;
pub(crate) mod signup_event;
pub(crate) mod delete_event;
pub(crate) mod edit;

use rand::prelude::SliceRandom;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
pub(crate) use new_event::NewEvent;
pub(crate) use event_composition::EventComposition;
pub(crate) use event_scope::EventScope;
pub(crate) use select_date::SelectDate;
pub(crate) use create_event::CreateEvent;
pub(crate) use event_info::EventInfo;
pub(crate) use signup_event::SignupEvent;
pub(crate) use delete_event::DeleteEvent;
pub(crate) use edit::edit_event;

fn not_an_event_response() -> CreateInteractionResponse {
    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
        .content(vec![
            "Eso no es un evento atontao!",
            "Ponte las gafas que esto no es un evento",
            "Madre mia estas cuajao",
            "Si si, ahora lo *borro*, espabilao",
            "Ya te gustaria a ti",
            "Le hemos dado fuerte al vinate eh?",
            "Vas mas perdido que mi creador en cyro",
            "A la proxima, me chivo y te mandan a portales",
            "Pues sabes que te digo? Lo vas a borrar tú -_-",
            "estas bien? quieres hablar?",
            "Que qué ocurre??? tú sabrás..."
        ].choose(&mut rand::thread_rng()).unwrap().to_string())
        .ephemeral(true)
    )
}
