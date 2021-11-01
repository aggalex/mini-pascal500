use lazy_static::lazy_static;

use std::collections::HashMap;

use std::sync::{Arc, RwLock};

use crate::ast::types::Type;

pub struct Store<'a, Content> {
    content: Arc<RwLock<HashMap<String, Arc<Content>>>>,
    fallback: Option<&'a Store<'a, Content>>
}

impl<Content> Store<'_, Content> {
    pub fn new() -> Self {
        Self {
            content: Arc::new(RwLock::new(HashMap::new())),
            fallback: None
        }
    }

    pub fn get<'a>(&self,
                   reference: impl Into<&'a str>) -> Option<Arc<Content>> {
        let reference: &str = reference.into();
        self.content.read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(reference)
            .map(|c| Some(c.clone()))
            .unwrap_or_else(|| self.fallback
                .map(|store| store.get(reference))
                .unwrap_or(None)
            )
    }

    pub fn contains<'a>(&self, reference: impl Into<&'a str>) -> bool {
        let reference: &str = reference.into();
        self.content.read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(reference)
            .is_some()
    }

    pub fn set(&self,
               reference: impl ToString,
               content: Content) {
        let reference = reference.to_string();
        if let Some(fallback) = self.fallback {
            if fallback.contains(&reference[..]) {
                return fallback.set(reference, content)
            }
        }
        self.content.write()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(reference.to_string(), Arc::new(content));
    }

    pub fn insert(&self, key_values: Vec<(impl ToString, impl Into<Content>)>) {
        let mut map = self.content.write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        for (key, value) in key_values {
            map.insert(key.to_string(), Arc::new(value.into()));
        }
    }
}

pub struct Variable {
    pub name: String,
    pub r#type: Type
}

impl Variable {
    pub fn new (name: String, r#type: Type) -> Variable {
        Self {
            name, r#type
        }
    }
}

lazy_static! {

    pub static ref TYPES: Store<'static, Type> = {
        let store = Store::new();
        store.insert(vec![
            ("INTEGER", Type::Integer),
            ("CHAR", Type::Char),
            ("REAL", Type::Real),
            ("BOOLEAN", Type::Boolean)
        ]);
        store
    };

    pub static ref GLOBALS: Store<'static, Variable> = Store::new();
}