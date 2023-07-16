use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use rand::distributions::{Distribution, Uniform};

const WINDOW_WIDTH: f32 = 1080.0;
const WINDOW_HEIGHT: f32 = 720.0;

const BALL_COUNT: usize = 30;
const BALL_SIZE: Vec3 = Vec3::new(50.0, 50.0, 0.0);
const BALL_SPEED: f32 = 400.0;

const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BALL_COLOR: Color = Color::rgb(0.9, 0.3, 0.3);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .insert_resource(Scoreboard {
            ball_count: BALL_COUNT,
        })
        .add_startup_system(setup)
        .add_systems((apply_velocity, check_for_collisions, mouse_click))
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource)]
struct Scoreboard {
    ball_count: usize,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    let die_width = Uniform::from(LEFT_WALL + BALL_SIZE.x..RIGHT_WALL - BALL_SIZE.x);
    let die_height = Uniform::from(BOTTOM_WALL + BALL_SIZE.y..TOP_WALL - BALL_SIZE.y);
    let die_velocity = Uniform::from(-0.5..0.5);

    for _ in 0..BALL_COUNT {
        let ball_pos_x = die_width.sample(&mut rng);
        let ball_pos_y = die_height.sample(&mut rng);
        let ball_velocity_x = die_velocity.sample(&mut rng);
        let ball_velocity_y = die_velocity.sample(&mut rng);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(BALL_COLOR)),
                transform: Transform::from_translation(Vec3::new(ball_pos_x, ball_pos_y, 1.0))
                    .with_scale(BALL_SIZE),
                ..default()
            },
            Ball,
            Velocity(Vec2::new(ball_velocity_x, ball_velocity_y) * BALL_SPEED),
        ));
    }

    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Ball Count: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::new(
                BALL_COUNT.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCORE_COLOR,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        }),
    );
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn check_for_collisions(mut balls_query: Query<(&mut Velocity, &Transform), With<Ball>>) {
    for (mut ball_velocity, ball_transform) in balls_query.iter_mut() {
        let ball_size = ball_transform.scale.truncate();

        let left_window_collision =
            WINDOW_WIDTH / 2.0 < ball_transform.translation.x + ball_size.x / 2.0;
        let right_window_collision =
            -WINDOW_WIDTH / 2.0 > ball_transform.translation.x - ball_size.x / 2.0;
        let top_window_collision =
            WINDOW_HEIGHT / 2.0 < ball_transform.translation.y + ball_size.y / 2.0;
        let bottom_window_collision =
            -WINDOW_HEIGHT / 2.0 > ball_transform.translation.y - ball_size.y / 2.0;

        if left_window_collision || right_window_collision {
            ball_velocity.x = -ball_velocity.x;
        }

        if top_window_collision || bottom_window_collision {
            ball_velocity.y = -ball_velocity.y;
        }
    }
}

fn mouse_click(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_event: Res<Input<MouseButton>>,
    balls_query: Query<(Entity, &Transform), With<Ball>>,
) {
    let window = window_query.single();

    if mouse_event.just_pressed(MouseButton::Left) {
        let mut cursor_position = window.cursor_position().unwrap();
        let window_center = Vec2::new(window.width() / 2., window.height() / 2.);
        cursor_position = Vec2::new(
            cursor_position.x - window_center.x,
            cursor_position.y - window_center.y,
        );

        for (ball_entity, ball_transform) in balls_query.iter() {
            let ball_pos = ball_transform.translation.truncate();
            let distance = cursor_position.distance(ball_pos);
            if distance < BALL_SIZE.x - 10.0 {
                scoreboard.ball_count -= 1;
                commands.entity(ball_entity).despawn();
            }
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.ball_count.to_string();
}
