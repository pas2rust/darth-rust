use uuid::Uuid;

use super::darth_tools::DarthTools;

pub trait UuidTrait {
    fn new_uuid() -> String;
}

impl UuidTrait for DarthTools {
    fn new_uuid() -> String {
        Uuid::new_v4().to_string()
    }
}