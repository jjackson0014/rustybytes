use bevy::prelude::*;

fn main() {

    // Run the app.
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

// Bevy ECS:

// Entities: A simple Type containing a unique integer
#[derive(Component)]
struct Person;

// Components: Structs that implement the Component trait
#[derive(Component)]
struct Name(String);

// Resources: Globally unique data
#[derive(Resource)]
struct GreetTimer(Timer);

// Systems: normal Rust functions
fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    // Use Commands to spawn entities
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

// Plugins
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(
            Timer::from_seconds(2.0, TimerMode::Repeating)
        ));

        app.add_systems(Startup,
            add_people
        );

        app.add_systems(Update,
            (update_people, greet_people).chain()
        );
    }
}


