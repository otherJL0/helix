mod alphabetic;
mod boolean;
mod date_time;
mod integer;

pub fn integer(selected_text: &str, amount: i64) -> Option<String> {
    integer::increment(selected_text, amount)
}

pub fn date_time(selected_text: &str, amount: i64) -> Option<String> {
    date_time::increment(selected_text, amount)
}
<<<<<<< Updated upstream
=======

pub fn boolean(selected_text: &str, amount: i64) -> Option<String> {
    boolean::increment(selected_text, amount)
}

pub fn alphabetic(selected_text: &str, amount: i64) -> Option<String> {
    alphabetic::increment(selected_text, amount)
}
>>>>>>> Stashed changes
