pub mod people {
    use bevy::{ecs::system::Commands, prelude::{Component, Query}, ecs::query::With};

    #[derive(Component)]
    pub struct Person;

    #[derive(Component)]
    pub struct Name(String);

    pub fn add_people(mut commands: Commands) {
        commands
            .spawn()
            .insert(Person)
            .insert(Name("Elaina Proctor".to_string()));
        commands
            .spawn()
            .insert(Person)
            .insert(Name("Renzo Hume".to_string()));
        commands
            .spawn()
            .insert(Person)
            .insert(Name("Zayna Nieves".to_string()));
    }

    pub fn greet_people(query: Query<&Name, With<Person>>) {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}
