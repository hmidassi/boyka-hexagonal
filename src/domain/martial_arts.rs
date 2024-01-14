pub mod martial_arts {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct MartialArt {
        pub(crate) martialart_name: String,
        pub(crate) punches: i8,
        pub(crate) kicks: i8,
        pub(crate) knees: i8,
        pub(crate) elbows: i8,
        pub(crate) standup_grappling: i8,
        pub(crate) ground_grappling: i8,
        pub(crate) ground_n_pound: i8,
        pub(crate) trapping: i8,
        pub(crate) weapon_manipulation: i8,
        pub(crate) weapon_defense: i8,
    }

    impl MartialArt {

        pub fn new(martial_art_name: String, proportions_map: HashMap<&str, i8>) -> Result<Self, String> {

            if(martial_art_name.is_empty()){
                return Err("Any martial art should have a name".to_string());
            }
            //find a way to use reflection at some point
            let martial_art: MartialArt = Self {
                martialart_name: martial_art_name,
                punches: *proportions_map.get("punches").unwrap(),
                kicks: *proportions_map.get("kicks").unwrap(),
                knees: *proportions_map.get("knees").unwrap(),
                elbows: *proportions_map.get("elbows").unwrap(),
                standup_grappling: *proportions_map.get("standup_grappling").unwrap(),
                ground_grappling: *proportions_map.get("ground_grappling").unwrap(),
                ground_n_pound: *proportions_map.get("ground_n_pound").unwrap(),
                trapping: *proportions_map.get("trapping").unwrap(),
                weapon_manipulation: *proportions_map.get("weapon_manipulation").unwrap(),
                weapon_defense: *proportions_map.get("weapon_defense").unwrap(),
            };
            if(!sum_equal_to_100(&martial_art)){
                return Err("The proportion of themes' sum should equal 100".to_string());
            }
            Ok(martial_art)
        }


    }

    fn sum_equal_to_100(martial_art: &MartialArt) -> bool {
        martial_art.punches
            + martial_art.kicks
            + martial_art.standup_grappling
            + martial_art.ground_grappling
            + martial_art.trapping
            + martial_art.weapon_manipulation
            + martial_art.weapon_defense
            + martial_art.knees
            + martial_art.elbows
            + martial_art.ground_n_pound
            == 100
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::domain::martial_arts::martial_arts::MartialArt;
    use super::*;

    const MARTIAL_ART_NAME: &str = "Jeet Kune Do";

    #[test]
    fn should_create_jkd_martial_art() {
        let mut proportions_map = HashMap::new();
        proportions_map.insert("punches", 20);
        proportions_map.insert("kicks", 10);
        proportions_map.insert("standup_grappling", 10);
        proportions_map.insert("ground_grappling", 5);
        proportions_map.insert("trapping", 10);
        proportions_map.insert("weapon_manipulation", 10);
        proportions_map.insert("weapon_defense", 10);
        proportions_map.insert("knees", 10);
        proportions_map.insert("elbows", 10);
        proportions_map.insert("ground_n_pound", 5);

        let martial_art = MartialArt::new(MARTIAL_ART_NAME.to_string(), proportions_map);

        assert_eq!(martial_art.clone().unwrap().martialart_name, MARTIAL_ART_NAME);
        assert_eq!(martial_art.unwrap().punches, 20);
    }

    #[test]
    fn proportion_sum_should_be_equal_to_100() {
        let mut proportions_map = HashMap::new();
        //sum equals 90
        proportions_map.insert("punches", 10);
        proportions_map.insert("kicks", 10);
        proportions_map.insert("standup_grappling", 10);
        proportions_map.insert("ground_grappling", 5);
        proportions_map.insert("trapping", 10);
        proportions_map.insert("weapon_manipulation", 10);
        proportions_map.insert("weapon_defense", 10);
        proportions_map.insert("knees", 10);
        proportions_map.insert("elbows", 10);
        proportions_map.insert("ground_n_pound", 5);

        let martial_art = MartialArt::new(MARTIAL_ART_NAME.to_string(), proportions_map);
        assert_eq!(martial_art.is_err(), true);
        assert_eq!(martial_art.unwrap_err(), "The proportion of themes' sum should equal 100");
    }

    #[test]
    fn martial_art_should_have_a_name() {
        let mut proportions_map = HashMap::new();
        proportions_map.insert("punches", 20);
        proportions_map.insert("kicks", 10);
        proportions_map.insert("standup_grappling", 10);
        proportions_map.insert("ground_grappling", 5);
        proportions_map.insert("trapping", 10);
        proportions_map.insert("weapon_manipulation", 10);
        proportions_map.insert("weapon_defense", 10);
        proportions_map.insert("knees", 10);
        proportions_map.insert("elbows", 10);
        proportions_map.insert("ground_n_pound", 5);

        let martial_art = MartialArt::new("".to_string(), proportions_map);
        assert_eq!(martial_art.is_err(), true);
        assert_eq!(martial_art.unwrap_err(), "Any martial art should have a name");
    }
}