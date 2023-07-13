use crate::story::parser::InputActionEvent;
use bevy::prelude::*;
use story_gen_parser::Action;

#[derive(Debug)]
pub enum StoryAction {
    Say(String, String, Option<Vec<u8>>),
    Comment(String),
}

#[derive(Event, Debug)]
pub struct InputStoryActionEvent(pub StoryAction);

pub fn handle_input_action(mut ev_input_action: EventReader<InputActionEvent>) {
    // Use a queue to store the actions and then process them in a separate system Maybe I will
    // have a 'processor' plugin that will handle this and it will call to each plugin to handle
    // it's action type for example, the say action will be handled by the fakeyou plugin the
    // comment action will be handled by the comment plugin, etc. each plugin will read from an
    // event the action type it handles and it will write to an event a struct with the data it
    // computes or maybe the plugin has an event reader and an event writer. That way I can send
    // directly the arguments from inside the action.
    //
    // Oh, the best idea is: Each plugin will have an event reader that gets as input the action
    // arguments, and the main plugin will have an event writer that gets as input the parsed
    // action, from the plugin that handled it.
    //
    // Maybe will need an index for them, so I can know which plugin handles which action. For the
    // order or the actions.
    ev_input_action.into_iter().for_each(|ev| {
        let action = &ev.0;
        match &action {
            Action::Comment(comment) => println!("Sending comment to comment plugin {:?}", comment),
            Action::Say(name, text) => {
                println!("Sending say to fakeyou plugin {:?}: {:?}", name, text)
            }
        }
    });
}

pub fn hanlde_story_action(mut ev_input_story_action: EventReader<InputStoryActionEvent>) {
    // The collector. This one should store the actions and send them to the story player
    ev_input_story_action.into_iter().for_each(|ev| {
        let action = &ev.0;
        println!("Received back the fully parsed action: {:?}", action);
    });
}
