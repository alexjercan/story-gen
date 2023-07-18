mod assets;
mod components;
mod debug;
mod input;
mod main_menu;
mod options;
mod pipeline;
mod selection_menu;
mod styles;
mod systems;

use bevy::prelude::*;
use clickable_button::ClickableButtonPlugin;
use systems::*;

pub const SYSTEM_TEXT: &str = r###"You are given the JSON specification of a language used to describe a story.
The description of each instruction is as follows:

- `comment`: {"comment": {"text": "description"}} Insert a comment in the story.
- `say`: {"say": {"name": "character", "text": "dialogue"}} Make a character speak.

Do NOT use any other instruction than the ones listed above.

See the following script as an example of how your script MUST to be formatted:

```
[
  { "comment": {"text": "The story takes place inside Rick's garage. Rick is trying to fix the portal gun."}},
  {"say": {"name": "Rick", "text": "I need to fix this portal gun."}},
  {"say": {"name": "Morty", "text": "Oh jeez, Rick! There is an alien in the living room."}},
  {"say": {"name": "Rick", "text": "Frick, Morty! I have to fix the portal gun so I can get rid of the alien."}}
]
```

"###;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins((
            assets::AssetsLoaderPlugin,
            ClickableButtonPlugin,
            main_menu::MainMenuPlugin,
            options::OptionsMenuPlugin,
            selection_menu::SelectionMenuPlugin,
            input::StoryInputPlugin,
            pipeline::PipelinePlugin,
        ))
        // .add_plugins(debug::DebugPlugin)
        .add_systems(OnEnter(AppState::Story), handle_started)
        .add_systems(
            Update,
            handle_created_text.run_if(in_state(AppState::Story)),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    AssetLoading,
    MainMenu,
    OptionsMenu,
    SelectionMenu,
    Story,
}
