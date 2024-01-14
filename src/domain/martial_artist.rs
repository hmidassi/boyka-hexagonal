use crate::persistence::experience_module::experience_module::Experience;



pub mod martial_artist{

    pub struct MartialArtist{
        id: i16,
        first_name: String,
        last_name: String,
        experiences: Vec<crate::persistence::experience_module::experience_module::Experience>
    }

    impl MartialArtist{
        pub fn list(conn: &diesel::SqliteConnection) -> Vec<Self>{
           return martial_artist_dsl.load::<MartialArtist>(conn).expect("Failed to load martial artists");
        }
    }
}