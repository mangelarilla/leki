pub mod delete_event;
pub mod edit;
pub mod create;
pub mod signup;

use rand::prelude::SliceRandom;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};

pub use delete_event::delete_event;
pub use edit::edit_event;
pub use create::create_event;
pub use signup::signup_event;

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
