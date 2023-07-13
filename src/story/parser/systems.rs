use super::{components::StoryParserLoader, InputActionEvent};
use crate::story::chatgpt::InputStoryEvent;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use futures_lite::future;
use story_gen_parser::actions;

pub fn handle_input_story(
    mut commands: Commands,
    mut ev_input_story: EventReader<InputStoryEvent>,
) {
    let Some(story) = ev_input_story.iter().next().map(|ev| ev.0.clone()) else { return };

    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool.spawn(async move { actions(&story) });

    commands.spawn(StoryParserLoader(task));
}

pub fn poll_story_loader_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut StoryParserLoader)>,
    mut ev_input_action: EventWriter<InputActionEvent>,
) {
    let Some((entity, mut task)) = tasks.iter_mut().next() else { return };

    if let Some(actions) = future::block_on(future::poll_once(&mut task.0)) {
        match actions {
            Ok(actions) => {
                println!("\n\nActions: {:?}", actions);
                ev_input_action.send_batch(actions.into_iter().map(InputActionEvent));
            }
            Err(s) => {
                // TODO: Handle error
                panic!("Error: {:?}", s);
            }
        }

        commands.entity(entity).despawn();
    }
}
