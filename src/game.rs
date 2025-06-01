use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use rand::Rng;

use crate::{PausableSystems, screens::Screen};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup)
        .add_systems(
            Update,
            advance_turn
                .run_if(input_just_pressed(KeyCode::Space))
                .in_set(PausableSystems),
        );
}

#[derive(Resource)]
struct Turn(u32);

#[derive(Component)]
struct TurnText;

#[derive(Component)]
struct Person {
    symptoms_at_turn: Option<u32>,
}

#[derive(Resource)]
struct SickMaterial(Handle<ColorMaterial>);

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Turn(0));

    commands.spawn((
        TurnText,
        Text::new("Day 0"),
        TextFont {
            font_size: 35.0,
            ..default()
        },
        Node {
            justify_self: JustifySelf::Center,
            ..default()
        },
        StateScoped(Screen::Gameplay),
    ));

    let mesh = meshes.add(Circle::new(20.0));
    let healthy_material = materials.add(Color::srgb(0.9, 0.9, 1.0));
    let sick_material = materials.add(Color::srgb(1.0, 0.3, 0.3));
    commands.insert_resource(SickMaterial(sick_material));

    let mut rng = rand::thread_rng();
    let infected = IVec2::new(rng.gen_range(-5..5), rng.gen_range(-5..5));

    for y in -5..5 {
        for x in -5..5 {
            let infected = infected.x == x && infected.y == y;
            commands.spawn((
                Person {
                    symptoms_at_turn: if infected {
                        Some(rng.gen_range(1..4))
                    } else {
                        None
                    },
                },
                Mesh2d(mesh.clone()),
                MeshMaterial2d(healthy_material.clone()),
                Transform::from_xyz(x as f32 * 60.0, y as f32 * 60.0, 0.0),
                StateScoped(Screen::Gameplay),
            ));
        }
    }
}

fn advance_turn(
    mut turn: ResMut<Turn>,
    mut turn_text: Single<&mut Text, With<TurnText>>,
    mut person_q: Query<(&mut Person, &mut MeshMaterial2d<ColorMaterial>)>,
    sick_material: Res<SickMaterial>,
) {
    turn.0 += 1;
    turn_text.0 = format!("Day {}", turn.0);
    for (person, mut mesh_material) in &mut person_q {
        let Some(symptoms_turn) = person.symptoms_at_turn else {
            continue;
        };
        if turn.0 == symptoms_turn {
            mesh_material.0 = sick_material.0.clone();
        }
    }
}
