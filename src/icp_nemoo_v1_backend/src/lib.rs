use candid::CandidType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::marker::Copy;
use std::sync::Mutex;

use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize,  Clone, Debug, Hash, PartialEq)]
pub struct Fisher {
    id: String,
    name: String,
    city: String,
    age: i32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Fish {
    id: String,
    fisher: String,
    weight: i32,
    height: i32,
}

lazy_static! {
    static ref FISH: Mutex<HashMap<String, Fish>> = Mutex::new(HashMap::new());
    static ref FISHER: Mutex<HashMap<String, Fisher>> = Mutex::new(HashMap::new());
}

#[ic_cdk::query]
fn get_fish(id: String) -> Fish {
    match FISH.lock().unwrap().get(&id) {
        Some(fish) => (*fish).clone(),
        None => Fish {
            id: String::from("N/A"),
            fisher: String::from("N/A"),
            weight: -1,
            height: -1,
        },
    }
}

#[ic_cdk::query]
fn get_fisher(id: String) -> Fisher {
    match FISHER.lock().unwrap().get(&id) {
        Some(fisher) => (*fisher).clone(),
        None => Fisher {
            id: String::from("N/A"),
            name: String::from("N/A"),
            city: String::from("N/A"),
            age: -1,
        },
    }
}

#[ic_cdk::update]
fn save_fish(fish: Fish) -> String {
    let id = fish.id;

    let mut map = FISH.lock().unwrap();
    map.insert(id.clone(), Fish {
        id: id.clone(),
        fisher: fish.fisher,
        height: fish.height,
        weight: fish.weight,
    });

    format!("Save Success: {}", id)
}

#[ic_cdk::update]
fn save_fisher(fisher: Fisher) -> String {
    let id = fisher.id;

    let mut map = FISHER.lock().unwrap();
    map.insert(id.clone(), Fisher {
        id: id.clone(),
        name: fisher.name.clone(),
        city: fisher.city.clone(),
        age: fisher.age,
    });

    format!("Save Success: {}", id)
}
