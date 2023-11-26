use rand::prelude::*;

use super::darth_tools::DarthTools;

pub trait RandomBytesTrait {
    fn new_random_bytes(
        uppercase_range: (u32, u32),
        lowercase_range: (u32, u32),
        number_range: (u32, u32),
        special_range: (u32, u32),
        emoji_range: (u32, u32),
    ) -> String;
}

impl RandomBytesTrait for DarthTools {
    fn new_random_bytes(
        uppercase_range: (u32, u32),
        lowercase_range: (u32, u32),
        number_range: (u32, u32),
        special_range: (u32, u32),
        emoji_range: (u32, u32),
    ) -> String {
        let (uppercase_min, uppercase_max) = uppercase_range;
        let (lowercase_min, lowercase_max) = lowercase_range;
        let (number_min,  number_max) =  number_range;
        let (special_min, special_max) = special_range;
        let (emoji_min, emoji_max) = emoji_range;
        let breaker = |min: u32, max: u32| thread_rng().gen_range(min + 1..=max + 1);
        let emojis = vec![
            '😀', '😄', '😊', '🙂', '😎', '😍', '🤩', '😂', '🤣', '😉', '😇', '🥰', '😋', '😜',
            '🤪', '😛', '🥳', '😺', '🐶', '🐱', '🐭', '🐰', '🦊', '🐻', '🐼', '🦁', '🐯', '🐮',
            '🐷', '🐸', '🐵', '🐔', '🐧', '🦆', '🦉', '🦄', '🐝', '🐞', '🦋', '🐢', '🐍', '🦎',
            '🦖', '🦕', '🐙', '🦑', '🦐', '🦞', '🦀', '🐳', '🐬', '🐟', '🐠', '🐡', '🦈', '🐋',
            '🐊', '🐆', '🐅', '🐃', '🐂', '🐄', '🦌', '🐪', '🐫', '🦙', '🦘', '🦥', '🦡', '🐘',
            '🦏', '🦛', '🐐', '🐏', '🐑', '🦒', '🐓', '🦃', '🦆', '🐕', '🐩', '🐈', '🐇', '🐁',
            '🐀', '🦔', '🐾', '🐉', '🐲', '🌵', '🌴', '🌷', '🌸', '🌹', '🌺', '🌻', '🌼', '🌽',
            '🌾', '🌿', '🍀', '🍁', '🍂', '🍃', '🍄', '🍅', '🍆', '🍇', '🍈', '🍉', '🍊', '🍋',
            '🍌', '🍍', '🍎', '🍏', '🍐', '🍑', '🍒', '🍓', '🍔', '🍕', '🍖', '🍗', '🍘', '🍙',
            '🍚', '🍛', '🍜', '🍝', '🍞', '🍟', '🍠', '🍡', '🍢', '🍣', '🍤', '🍥', '🍦', '🍧',
            '🍨', '🍩', '🍪', '🍫', '🍬', '🍭', '🍮', '🍯', '🍰', '🍱', '🍲', '🍳', '🍴', '🍵',
            '🍶', '🍷', '🍸', '🍹', '🍺', '🍻', '🍼', '🍾', '🍿', '🎀', '🎁', '🎂', '🎃', '🎄',
            '🎅', '🎆', '🎇', '🎈', '🎉', '🎊', '🎋', '🎌', '🎍', '🎎', '🎏', '🎐', '🎑', '🎒',
            '🎓', '🎠', '🎡', '🎢', '🎣', '🎤', '🎥', '🎦', '🎧', '🎨', '🎩', '🎪', '🎫', '🎬',
            '🎭', '🎮', '🎯', '🎰', '🎱', '🎲', '🎳', '🎴', '🎵', '🎶', '🎷', '🎸', '🎹', '🎺',
            '🎻', '🎼', '🎽', '🎾', '🎿', '🏀', '🏁', '🏂', '🏃', '🏄', '🏅', '🏆', '🏇', '🏈',
            '🏉', '🏊', '🏏', '🏐', '🏑', '🏒', '🏓', '🏠', '🏡', '🏢', '🏣', '🏤', '🏥', '🏦',
            '🏧', '🏨', '🏩', '🏪', '🏫', '🏬', '🏭', '🏮', '🏯', '💒', '🔥', '🔦', '🔧', '🔨',
            '🔩', '🔪', '🔫', '🔬', '🔭', '🔮', '🔯', '🔰', '🔱', '🔲',
        ];

        let special_characters = vec![
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '-', '+', '=', '[', ']', '{',
            '}', '|', '\\', '/', '<', '>', ',', '.', '?', ':', ';',
        ];
        let uppercase_breaker = breaker(uppercase_min, uppercase_max);
        let lowercase_breaker = breaker(lowercase_min, lowercase_max);
        let number_breaker = breaker(number_min, number_max);
        let special_characters_breaker = breaker(special_min, special_max);
        let emoji_breaker = breaker(emoji_min,  emoji_max);

        let mut input: String = "".to_string();
        let mut add_random_uppercase_char = 0;
        let mut add_random_lowercase_char = 0;
        let mut add_random_number_char = 0;
        let mut add_random_emoji_char = 0;
        let mut add_special_characters = 0;

        loop {
            add_random_uppercase_char += 1;
            match add_random_uppercase_char == uppercase_breaker {
                true => break,
                false => input.push(thread_rng().gen_range('A'..='Z')),
            }
        }
        loop {
            add_random_lowercase_char += 1;
            match add_random_lowercase_char == lowercase_breaker || lowercase_breaker == 0 {
                true => break,
                false => input.push(thread_rng().gen_range('a'..='z')),
            }
        }
        loop {
            add_random_number_char += 1;
            match add_random_number_char == number_breaker || number_breaker == 0 {
                true => break,
                false => input.push(thread_rng().gen_range('0'..='9')),
            }
        }
        loop {
            add_random_emoji_char += 1;
            match add_random_emoji_char == emoji_breaker || emoji_breaker == 0 {
                true => break,
                false => input.push(*emojis.choose(&mut thread_rng()).unwrap()),
            }
        }
        loop {
            add_special_characters += 1;
            match add_special_characters == special_characters_breaker
                || special_characters_breaker == 0
            {
                true => break,
                false => input.push(*special_characters.choose(&mut thread_rng()).unwrap()),
            }
        }
        let mut chars: Vec<char> = input.chars().collect();
        let mut rng = rand::thread_rng();
        chars.shuffle(&mut rng);
        let shuffled: String = chars.iter().collect();
        shuffled
    }
}
