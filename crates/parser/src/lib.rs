use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use story_gen_parser::*;
pub use story_gen_parser::{Action, Error};

type ParserResult<T> = Result<T, Error>;

// This component is used to run the Parser on another thread
#[derive(Component)]
struct ParserTask(Task<ParserResult<Vec<Action>>>);

// The plugin will listen for this event to trigger the Parser
#[derive(Event, Debug)]
pub struct InputTextEvent(pub String);

// This event is triggered when the parser is done
#[derive(Event, Debug)]
pub struct CreatedActionsEvent(pub ParserResult<Vec<Action>>);

pub struct ParserPlugin;

impl Plugin for ParserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputTextEvent>()
            .add_event::<CreatedActionsEvent>()
            .add_systems(Update, (handle_input_text, poll_parser_task));
    }
}

fn handle_input_text(mut commands: Commands, mut ev_input_text: EventReader<InputTextEvent>) {
    ev_input_text.iter().for_each(|ev| {
        let text = ev.0.clone();
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move { actions(&text) });

        commands.spawn(ParserTask(task));
    });
}

fn poll_parser_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ParserTask)>,
    mut ev_created_actions: EventWriter<CreatedActionsEvent>,
) {
    let Some((entity, mut task)) = tasks.iter_mut().next() else { return };

    // TODO: Maybe here use send_batch and introduce an IllegalAction
    if let Some(actions) = future::block_on(future::poll_once(&mut task.0)) {
        ev_created_actions.send(CreatedActionsEvent(actions));

        commands.entity(entity).despawn();
    }
}
