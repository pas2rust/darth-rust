use crate::tools::random_bytes::random_bytes;

pub struct DarthTools;

pub trait DarthToolsTrait {
    fn new_uuid() -> String;
    fn new_random_bytes(
        gen_uppercase: Option<u32>,
        gen_lowercase: Option<u32>,
        gen_number: Option<u32>,
        gen_special_characters: Option<u32>,
        gen_emoji: Option<u32>,
    ) -> String;
}

impl DarthToolsTrait for DarthTools {
    fn new_uuid() -> String {
        "".to_string()
    }
    fn new_random_bytes(
            gen_uppercase: Option<u32>,
            gen_lowercase: Option<u32>,
            gen_number: Option<u32>,
            gen_special_characters: Option<u32>,
            gen_emoji: Option<u32>,
        ) -> String {
        random_bytes(
            gen_uppercase,
            gen_lowercase,
            gen_number,
            gen_special_characters,
            gen_emoji,
        )
    }
}