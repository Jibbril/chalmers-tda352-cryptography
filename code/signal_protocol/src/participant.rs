use crate::{mod_group::ModGroup, utils::mod_exp};

#[allow(dead_code)] 
pub struct Participant {
    pub name: String,
    pub pk: u64,
    pub mtpk:u64,
    pub ephpks: Vec<u64>,
    sk: u64,
    mtsk: u64,
    ephsks: Vec<u64>
}

impl Participant {
    pub fn new(name: &str, group: &ModGroup) -> Self{
        let mut secret_keys = group.sample_n(10);
        let mut public_keys: Vec<u64> = secret_keys.iter()
            .map(|k| mod_exp(group.generator, *k, group.modulo))
            .collect();
        
        Self {
            name: name.to_string(),
            pk: public_keys.pop().unwrap(),
            sk: secret_keys.pop().unwrap(),
            mtpk: public_keys.pop().unwrap(),
            mtsk: secret_keys.pop().unwrap(),
            ephpks: public_keys,
            ephsks: secret_keys,
        }
    }
}
