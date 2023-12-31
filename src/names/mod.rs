use rand::Rng;


pub fn give_word() -> String {
    let words: Vec<&str> = vec![
        "Apple",
        "Beach",
        "Clown",
        "Dance",
        "Early",
        "Flood",
        "Ghost",
        "House",
        "Igloo",
        "Jolly",
        "Karma",
        "Lemon",
        "Music",
        "Ninja",
        "Olive",
        "Pizza",
        "Quiet",
        "River",
        "Sunny",
        "Tacos",
        "Unity",
        "Vital",
        "Witty",
        "Xerox",
        "Yacht",
        "Zooms",
        "Angel",
        "Blend",
        "Coral",
        "Dingo",
        "Easter",
        "Flame",
        "Gummy",
        "Happy",
        "Inkle",
        "Jelly",
        "Karma",
        "Loyal",
        "Magic",
        "Nymph",
        "Opera",
        "Peace",
        "Quirk",
        "Rebel",
        "Silky",
        "Tasty",
        "Urban",
        "Vixen",
        "Wacky",
        "Yummy",
        "Zeal",
        "Amber",
        "Brave",
        "Caves",
        "Diver",
        "Elbow",
        "Fairy",
        "Glove",
        "Hello",
        "Indie",
        "Jokes",
        "Karma",
        "Lying",
        "Mango",
        "Noble",
        "Onion",
        "Peace",
        "Queen",
        "Radii",
        "Scout",
        "Tally",
        "Urban",
        "Vexed",
        "Whisk",
        "Yummy",
        "Zebra",
        "Anime",
        "Black",
        "Candy",
        "Dazed",
        "Eager",
        "Fable",
        "Giddy",
        "Happy",
        "Inset",
        "Jelly",
        "Karma",
        "Lemon",
        "Magic",
        "Noble",
        "Onion",
        "Peace",
        "Quirk",
        "Rebel",
        "Silky",
        "Tasty",
        "Unity",
        "Vixen",
        "Wacky",
        "Yummy"
            ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    words[index].to_string()

}

