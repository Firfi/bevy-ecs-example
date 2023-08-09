use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn move_system(mut query: Query<&mut Position>) {
    for mut position in query.iter_mut() {
        position.x += 0.1;
        position.y += 0.1;
        println!("Updated position to: ({}, {})", position.x, position.y);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Position { x: 0.0, y: 0.0 });
    commands.spawn(Position { x: 0.0, y: 0.0 });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // game loop, window, etc
        .add_systems(Startup, setup)
        .add_systems(Update, move_system)
        .run();
}

// TODO i.e. Attack System, AOE System - based on Position, query the ones who have HP

// // This system will handle player attacks.
// fn player_attack(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&Player, &Position, &Attack)>,
//     mut query_targets: Query<(&Enemy, &mut Health, &Position)>,
// ) {
//     for (_player, position, attack) in query.iter_mut() {
//         if keyboard_input.pressed(KeyCode::Space) {
//             for (_enemy, mut health, target_position) in query_targets.iter_mut() {
//                 if distance(position, target_position) <= attack.range {
//                     health.current = health.current.saturating_sub(attack.damage);
//                 }
//             }
//         }
//     }
// }
//
// // This system handles AoE attacks.
// fn aoe_attack(
//     mut query: Query<(&AoE, &Position)>,
//     mut query_targets: Query<(&Enemy, &mut Health, &Position)>,
// ) {
//     for (aoe, position) in query.iter_mut() {
//         for (_enemy, mut health, target_position) in query_targets.iter_mut() {
//             if distance(position, target_position) <= aoe.radius {
//                 health.current = health.current.saturating_sub(aoe.damage);
//             }
//         }
//     }
// }