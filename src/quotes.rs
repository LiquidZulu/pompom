use rand::seq::SliceRandom;

pub enum QuoteTypes {
    Work,
    Rest,
}

static WORK_QUOTES: [&str; 5] = [
    "Keep going!",
    "The secret to getting ahead is getting started.",
    "A journey of a thousand miles begins with a single step.",
    "The best time to plant a tree was 20 years ago. The second best time is now.",
    "Think it. Dream it. Do it.",
];

static REST_QUOTES: [&str; 3] = [
    "Take a break!",
    "Sitting quietly, doing nothing. Spring comes, and the grass grows, by itself.",
    "Take a moment to pause, and come back to the present.",
];

pub fn get_quote(quote_type: QuoteTypes) -> &'static str {
    match quote_type {
        QuoteTypes::Work => match WORK_QUOTES.choose(&mut rand::thread_rng()) {
            Some(thing) => thing,
            None => WORK_QUOTES[0],
        },
        QuoteTypes::Rest => match REST_QUOTES.choose(&mut rand::thread_rng()) {
            Some(thing) => thing,
            None => REST_QUOTES[0],
        },
    }
}
