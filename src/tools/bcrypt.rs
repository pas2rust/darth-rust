use super::darth_tools::DarthTools;

pub trait BcryptTrait {
    fn new_bcrypt_hash(value: String, cost: u32) -> Result<String, bcrypt::BcryptError>;
    fn new_bcrypt_verify(value: String, hash: &String) -> Result<bool, bcrypt::BcryptError>;
}

impl BcryptTrait for DarthTools {
    fn new_bcrypt_hash(value: String, cost: u32) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(value, cost)
    }
    fn new_bcrypt_verify(value: String, hash: &String) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(value, hash)
    }
}
