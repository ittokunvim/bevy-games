use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashSet;

use crate::mainmenu::{mainmenu_setup, mainmenu_update};
use crate::ingame::{
    ingame_setup,
    move_player_from_input,
    translate_grid_coords_entities,
    cache_wall_locations,
    check_goal,
    check_pause
};
use crate::pause::{pause_setup, pause_update};

pub mod mainmenu;
pub mod ingame;
pub mod pause;

pub const GAME_TITLE: &str = "2D Setup";
pub const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 800.0);
pub const BACKGROUND_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Pause,
    // GameOver,
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Component)]
pub struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct GoalBundle {
    goal: Goal,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Default, Resource)]
pub struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelWalls {
    fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width 
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAME_TITLE.to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        // ldtk setup
        .add_plugins(LdtkPlugin)
        .init_resource::<LevelWalls>()
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .register_ldtk_int_cell::<WallBundle>(1)
        // mainmenu
        .add_systems(OnEnter(AppState::MainMenu), mainmenu_setup)
        .add_systems(Update, mainmenu_update.run_if(in_state(AppState::MainMenu)))
        // ingame
        .add_systems(OnEnter(AppState::InGame), ingame_setup)
        .add_systems(Update, move_player_from_input.run_if(in_state(AppState::InGame)))
        .add_systems(Update, translate_grid_coords_entities.run_if(in_state(AppState::InGame)))
        .add_systems(Update, cache_wall_locations.run_if(in_state(AppState::InGame)))
        .add_systems(Update, check_goal.run_if(in_state(AppState::InGame)))
        .add_systems(Update, check_pause.run_if(in_state(AppState::InGame)))
        // pause
        .add_systems(OnEnter(AppState::Pause), pause_setup)
        .add_systems(Update, pause_update.run_if(in_state(AppState::Pause)))
        // .add_systems(Update, gameover.run_if(in_state(AppState::GameOver)))
        .run();
}   
