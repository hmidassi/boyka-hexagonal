use chrono::NaiveDate;
use crate::persistence::martial_arts::martial_arts::MartialArt;

pub mod experience_module {
    pub enum Experience{
        CurrentExperience{experience: GeneralExperience},
        PastExperience{experience: GeneralExperience, end_date: chrono::NaiveDate}
    }

    pub struct GeneralExperience {
        id: i16,
        club_name: String,
        starting_date: chrono::NaiveDate,
        //one club may offer several martial arts at once
        martial_arts: Vec<crate::persistence::martial_arts::martial_arts::MartialArt>,
        //find a smarter way to associate each martial art and its average duration per
        average_practice_per_week_in_min: Vec<i16>
    }
}
