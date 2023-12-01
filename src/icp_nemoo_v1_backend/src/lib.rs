use candid::CandidType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Fisher {
    id: String,
    name: String,
    city: String,
    age: i32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Fish {
    id: String,
    tag_id: String,
    fisher: String,
    location: String,
    date_catch: String,
    date_process: String,
    latitute: String,
    longitude: String,
    gross_weight: i32,
    sekami_1: i32,
    sekami_2: i32,
    haranaka_1: i32,
    haranaka_2: i32,
    toro: i32,
    head: i32,
    tail: i32,
    bone: i32,
    skin: i32,
}

lazy_static! {
    static ref FISH: Mutex<HashMap<String, Fish>> = Mutex::new(HashMap::new());
    static ref FISHER: Mutex<HashMap<String, Fisher>> = Mutex::new(HashMap::new());
}

#[ic_cdk::query]
fn get_command() -> String {
    format!("ic_nemoo_v1")
}

#[ic_cdk::query]
fn get_fish(id: String) -> Fish {
    match FISH.lock().unwrap().get(&id) {
        Some(fish) => (*fish).clone(),
        None => Fish {
            id: String::from("N/A"),
            tag_id: String::from("N/A"),
            fisher: String::from("N/A"),
            location: String::from("N/A"),
            date_catch: String::from("N/A"),
            date_process: String::from("N/A"),
            latitute: String::from("N/A"),
            longitude: String::from("N/A"),
            gross_weight: -1,
            sekami_1: -1,
            sekami_2: -1,
            haranaka_1: -1,
            haranaka_2: -1,
            toro: -1,
            head: -1,
            tail: -1,
            bone: -1,
            skin: -1,
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
    map.insert(
        id.clone(),
        Fish {
            id: id.clone(),
            tag_id: fish.tag_id.clone(),
            fisher: fish.fisher.clone(),
            location: fish.location.clone(),
            date_catch: fish.date_catch.clone(),
            date_process: fish.date_process.clone(),
            latitute: fish.latitute.clone(),
            longitude: fish.longitude.clone(),
            gross_weight: fish.gross_weight,
            sekami_1: fish.sekami_1,
            sekami_2: fish.sekami_2,
            haranaka_1: fish.haranaka_1,
            haranaka_2: fish.haranaka_2,
            toro: fish.toro,
            head: fish.head,
            tail: fish.tail,
            bone: fish.bone,
            skin: fish.skin,
        },
    );

    format!("Save Success: {}", id)
}

#[ic_cdk::update]
fn save_fisher(fisher: Fisher) -> String {
    let id = fisher.id;

    let mut map = FISHER.lock().unwrap();
    map.insert(
        id.clone(),
        Fisher {
            id: id.clone(),
            name: fisher.name.clone(),
            city: fisher.city.clone(),
            age: fisher.age,
        },
    );

    format!("Save Success: {}", id)
}
