

use std::fmt;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::util::value::Value;

// mod value {
//     pub mod unknown {
//         pub const VALUE: &str = "UNKNOWN";
//     }
// }

#[derive(Clone, Default)]
pub struct Entity {
    id: String,
    name: String,
}

impl Entity {
    //REM: Constructor without parameters
    pub fn new() -> Self {
        Self::new_with_id(None)
    }

    //REM: Constructor with id parameter
    pub fn new_with_id(id: Option<&str>) -> Self {
        Self::new_with_id_and_name(id, None)
    }

    //REM: Constructor with id and name parameters
    pub fn new_with_id_and_name(id: Option<&str>, name: Option<&str>) -> Self {
        let id = match id {
            Some(id_str) if !id_str.trim().is_empty() => id_str,
            _ => Value::WAITING.value,
        };
        let mut entity = Self {
            id: id.to_string(),
            name: String::new(),
        };

        entity.init(name);
        entity
    }

    //REM: Copy constructor
    pub fn from_entity(entity: &Entity) -> Self {
        Self {
            id: entity.id.clone(),
            name: entity.name.clone(),
        }
    }

    fn init(&mut self, name: Option<&str>) {
        self.set_name(name);
    }

    pub fn set_name(&mut self, name: Option<&str>) {
        if let Some(name_str) = name {
            if !name_str.trim().is_empty() && name_str.to_lowercase() != Value::UNKNOWN.value.to_string() {
                let trimmed_name = name_str.trim();
                if self.name.is_empty() || self.name.to_lowercase() != trimmed_name.to_lowercase() {
                    self.name = trimmed_name.to_string();
                    return;
                }
            }
        }
        if self.name.is_empty() {
            self.name = Value::UNKNOWN.value.to_string();
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn hash_code_usize(&self) -> usize {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as usize
    }

    pub fn hash_code(&self) -> u32 {
        ((self.hash_code_usize() & 0xFFFF_FFFF) as u32)
    }

    pub fn hash_str(&self) -> String {
        format!("{:x}", self.hash_code() )
    }

    pub fn to_string(&self) -> String {
        return format!("{:?}", self );
    }
}

//REM: Implementing Display trait for Entity
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{:x}[id='{}', name='{}']", std::any::type_name::<Self>(), self.hash_code(), self.id, self.name)
    }
}

//REM: Implementing Debug trait for Entity
impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{:x}[id='{}', name='{}']", std::any::type_name::<Self>(), self.hash_code_usize(), self.id, self.name)
    }
}

//REM: Implementing PartialEq trait for Entity
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name.to_lowercase() == other.name.to_lowercase()
    }
}

//REM: Implementing Eq trait for Entity
impl Eq for Entity {}

//REM: Implementing Hash trait for Entity
impl Hash for Entity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.to_lowercase().hash(state);
    }
}
