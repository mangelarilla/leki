pub mod pipelines;

use crate::events::{EventKind};
use crate::interactions::pipelines::InteractionPipeline;
use crate::messages::events::{DeleteEvent, EventInfo, NewEvent};

const NEW_TRIAL: &'static str = "new_trial";
const NEW_PVP: &'static str = "new_pvp";
pub(crate) fn define_pipeline() -> InteractionPipeline {
    let mut pipeline = InteractionPipeline::new();

    pipeline.add("events", NewEvent::new(NEW_TRIAL, NEW_PVP));

    let new_trial = EventInfo::new(EventKind::Trial, &mut pipeline);
    pipeline.add(NEW_TRIAL, new_trial);
    let new_pvp = EventInfo::new(EventKind::PvP, &mut pipeline);
    pipeline.add(NEW_PVP, new_pvp);

    pipeline.add("Delete event", DeleteEvent::new());

    pipeline
}



