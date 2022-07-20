use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NamesIdsDto {
    pub ids_to_names: HashMap<u32, String>,
    pub names_to_ids: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize)]
pub struct ExtraDataDto {
    pub companies: NamesIdsDto,
    pub categories: NamesIdsDto,
}

impl NamesIdsDto {
    pub fn name_from_id(&self, id: u32) -> String {
        match self.ids_to_names.get(&id) {
            Some(name) => name.clone(),
            None => "-Unknown name-".to_string(),
        }
    }
    
    pub fn id_from_name(&self, name: &String) -> u32 {
        match self.names_to_ids.get(name) {
            Some(id) => *id,
            None => 0,
        }
    }
}