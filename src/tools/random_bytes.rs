use rand::prelude::*;

use super::darth_tools::DarthTools;

pub trait RandomBytesTrait {
    fn new_random_bytes(
        gen_uppercase: Option<u32>,
        gen_lowercase: Option<u32>,
        gen_number: Option<u32>,
        gen_special_characters: Option<u32>,
        gen_emoji: Option<u32>,
    ) -> String;
}

impl RandomBytesTrait for DarthTools {
    fn new_random_bytes(
        gen_uppercase: Option<u32>,
        gen_lowercase: Option<u32>,
        gen_number: Option<u32>,
        gen_special_characters: Option<u32>,
        gen_emoji: Option<u32>,
    ) -> String {
        let breaker = |value: Option<u32>| value.unwrap_or(thread_rng().gen_range(5..=20));
        let emojis = vec![
            '😀', '😄', '😊', '🙂', '😎', '😍', '🤩', '😂', '🤣', '😉', '😇', '🥰', '😋', '😜', '🤪',
            '😛', '🥳', '😺', '🐶', '🐱', '🐭', '🐰', '🦊', '🐻', '🐼', '🦁', '🐯', '🐮', '🐷', '🐸',
            '🐵', '🐔', '🐧', '🦆', '🦉', '🦄', '🐝', '🐞', '🦋', '🐢', '🐍', '🦎', '🦖', '🦕', '🐙',
            '🦑', '🦐', '🦞', '🦀', '🐳', '🐬', '🐟', '🐠', '🐡', '🦈', '🐋', '🐊', '🐆', '🐅', '🐃',
            '🐂', '🐄', '🦌', '🐪', '🐫', '🦙', '🦘', '🦥', '🦡', '🐘', '🦏', '🦛', '🐐', '🐏', '🐑',
            '🦒', '🐓', '🦃', '🦆', '🐕', '🐩', '🐈', '🐇', '🐁', '🐀', '🦔', '🐾', '🐉', '🐲', '🌵',
            '🌴', '🌷', '🌸', '🌹', '🌺', '🌻', '🌼', '🌽', '🌾', '🌿', '🍀', '🍁', '🍂', '🍃', '🍄',
            '🍅', '🍆', '🍇', '🍈', '🍉', '🍊', '🍋', '🍌', '🍍', '🍎', '🍏', '🍐', '🍑', '🍒', '🍓',
            '🍔', '🍕', '🍖', '🍗', '🍘', '🍙', '🍚', '🍛', '🍜', '🍝', '🍞', '🍟', '🍠', '🍡', '🍢',
            '🍣', '🍤', '🍥', '🍦', '🍧', '🍨', '🍩', '🍪', '🍫', '🍬', '🍭', '🍮', '🍯', '🍰', '🍱',
            '🍲', '🍳', '🍴', '🍵', '🍶', '🍷', '🍸', '🍹', '🍺', '🍻', '🍼', '🍾', '🍿', '🎀', '🎁',
            '🎂', '🎃', '🎄', '🎅', '🎆', '🎇', '🎈', '🎉', '🎊', '🎋', '🎌', '🎍', '🎎', '🎏', '🎐',
            '🎑', '🎒', '🎓', '🎠', '🎡', '🎢', '🎣', '🎤', '🎥', '🎦', '🎧', '🎨', '🎩', '🎪', '🎫',
            '🎬', '🎭', '🎮', '🎯', '🎰', '🎱', '🎲', '🎳', '🎴', '🎵', '🎶', '🎷', '🎸', '🎹', '🎺',
            '🎻', '🎼', '🎽', '🎾', '🎿', '🏀', '🏁', '🏂', '🏃', '🏄', '🏅', '🏆', '🏇', '🏈', '🏉',
            '🏊', '🏏', '🏐', '🏑', '🏒', '🏓', '🏠', '🏡', '🏢', '🏣', '🏤', '🏥', '🏦', '🏧', '🏨',
            '🏩', '🏪', '🏫', '🏬', '🏭', '🏮', '🏯', '💒', '🔥', '🔦', '🔧', '🔨', '🔩', '🔪', '🔫',
            '🔬', '🔭', '🔮', '🔯', '🔰', '🔱', '🔲',
        ];
    
        let special_characters = vec![
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '-', '+', '=', '[', ']', '{', '}',
            '|', '\\', '/', '<', '>', ',', '.', '?', ':', ';',
        ];
        let uppercase_breaker = breaker(gen_uppercase);
        let lowercase_breaker = breaker(gen_lowercase);
        let number_breaker = breaker(gen_number);
        let special_characters_breaker = breaker(gen_special_characters);
        let emoji_breaker = breaker(gen_emoji);
    
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