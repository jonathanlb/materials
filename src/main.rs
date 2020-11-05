#![feature(proc_macro_hygiene, decl_macro)]

use normal::{IdPairs, Normal};
#[macro_use] extern crate rocket;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

const FILE_TAB: &str = "files";
const FILE_COL: &str = "name";
const FILE_URL_COL: &str = "url";

const KEY_TAB: &str = "keywords";
const KEY_COL: &str = "keyword";

const NOTE_TAB: &str = "notes";
const NOTE_COL: &str = "note";

const FILE_KEY_TAB: &str = "file_keyword";
const FILE_KEY_LEFT_COL: &str = "file";
const FILE_KEY_RIGHT_COL: &str = "keyword";

const FILE_NOTE_TAB: &str = "file_note";
const FILE_NOTE_LEFT_COL: &str = "file";
const FILE_NOTE_RIGHT_COL: &str = "note";

const NOTE_KEY_TAB: &str = "note_keyword";
const NOTE_KEY_LEFT_COL: &str = "note";
const NOTE_KEY_RIGHT_COL: &str = "keyword";

const UNKNOWN_URL: String = String::new();

struct MaterialsConfig<'a> {
    db_file: &'a str,
}

/// TODO: memoize and pool connections
impl<'a> MaterialsConfig<'a> {
    fn files(&self) -> Box<Normal> {
        Box::new(
            Normal::new_with_nonkeys(
                self.db_file, FILE_TAB, FILE_COL, vec!(FILE_URL_COL).iter()).
                unwrap())
    }

    fn file_keyword_pairs(&self) -> Box<IdPairs> {
        Box::new(
            IdPairs::new(
                self.db_file, FILE_KEY_TAB, FILE_KEY_LEFT_COL, FILE_KEY_RIGHT_COL).
                unwrap())
    }

    fn file_note_pairs(&self) -> Box<IdPairs> {
        Box::new(
            IdPairs::new(
                self.db_file, FILE_NOTE_TAB, FILE_NOTE_LEFT_COL, FILE_NOTE_RIGHT_COL).
                unwrap())
    }

    fn keywords(&self) -> Box<Normal> {
        Box::new(Normal::new(self.db_file, KEY_TAB, KEY_COL).unwrap())
    }

    fn notes(&self) -> Box<Normal> {
        Box::new(Normal::new(self.db_file, NOTE_TAB, NOTE_COL).unwrap())
    }

    fn note_keyword_pairs(&self) -> Box<IdPairs> {
        Box::new(
            IdPairs::new(self.db_file, NOTE_KEY_TAB, NOTE_KEY_LEFT_COL, NOTE_KEY_RIGHT_COL).
                unwrap())
    }
}

macro_rules! page_iter {
    ($i:expr, $page_size:expr, $page_num:expr) => {
        $i.skip($page_size*$page_num).take($page_size).collect()
    };
}

mod keywords;
use crate::keywords::{
    static_rocket_route_info_for_get_keyword, 
    static_rocket_route_info_for_search_keywords
};

mod materials;
use crate::materials::{
    static_rocket_route_info_for_get_keywords,
    static_rocket_route_info_for_get_material,
    static_rocket_route_info_for_key_material,
    static_rocket_route_info_for_search_materials,
    static_rocket_route_info_for_search_materials_by_keyword,
    static_rocket_route_info_for_search_materials_by_note
};

mod notes;
use crate::notes::{
    static_rocket_route_info_for_get_note,
    static_rocket_route_info_for_search_notes,
    static_rocket_route_info_for_search_notes_by_keyword
};

fn main() {
    let config = MaterialsConfig{
        db_file: "materials.sqlite"
    };

    // XXX attaches CORS always.... TODO: configure
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();

    rocket::ignite().
        manage(config).
        mount("/keyword", routes![get_keyword, search_keywords]).
        mount("/material", routes![get_keywords, get_material, key_material, search_materials, search_materials_by_keyword, search_materials_by_note]).
        mount("/note", routes![get_note, search_notes, search_notes_by_keyword]).
        attach(cors).
        launch();
}
