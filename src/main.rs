#![feature(proc_macro_hygiene, decl_macro)]

use normal::{IdPairs, Normal};
#[macro_use] extern crate rocket;
use rocket::State;
use rocket_contrib::json::Json;
use serde::ser::{Serialize, Serializer, SerializeStruct};

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

trait MaterialsDb {
    fn files(&self) -> Box<Normal>;
    fn keywords(&self) -> Box<Normal>;
}

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

#[get("/<key_id>")]
fn get_keyword(key_id: i64, db: State<MaterialsConfig>) -> String {
    db.keywords().get(key_id).unwrap()
}

#[get("/search/<key_search>/<page_size>/<page_num>")]
fn search_keywords<'a>(key_search: String, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let norm = db.keywords();
    let i = norm.search(key_search.as_str()).unwrap();
    Json(page_iter!(i, page_size, page_num))
}

struct MaterialDescriptor {
    id: i64,
    name: String,
    url: String,
}

impl Serialize for MaterialDescriptor {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        let mut state = s.serialize_struct("MaterialDescriptor", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("url", &self.url)?;
        state.end()
    }
}

#[get("/<key_id>")]
fn get_material(key_id: i64, db: State<MaterialsConfig>) -> Json<MaterialDescriptor> {
    let f = db.files();
    let name = f.get(key_id).unwrap();
    let url = f.get_nonkey(key_id, FILE_URL_COL).unwrap_or(UNKNOWN_URL);
    Json(MaterialDescriptor{
        id: key_id,
        name: name,
        url: url,
    })
}

#[get("/search/<key_search>/<page_size>/<page_num>")]
fn search_materials<'a>(key_search: String, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let f = db.files();
    let i = f.search(key_search.as_str()).unwrap();
    Json(page_iter!(i, page_size, page_num))
}

#[get("/keyword/<key_id>/<page_size>/<page_num>")]
fn search_materials_by_keyword<'a>(key_id: i64, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let fxk = db.file_keyword_pairs();
    let i = fxk.invert(key_id).unwrap();
    Json(page_iter!(i, page_size, page_num))
}

#[get("/note/<note_id>/<page_size>/<page_num>")]
fn search_materials_by_note<'a>(note_id: i64, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let fxn = db.file_note_pairs();
    let i = fxn.invert(note_id).unwrap();
    Json(page_iter!(i, page_size, page_num))
}

#[get("/<note_id>")]
fn get_note(note_id: i64, db: State<MaterialsConfig>) -> String {
    db.notes().get(note_id).unwrap()
}

#[get("/search/<note_search>/<page_size>/<page_num>")]
fn search_notes<'a>(note_search: String, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let norm = db.notes();
    let i = norm.search(note_search.as_str()).unwrap();
    Json(page_iter!(i, page_size, page_num))
}

#[get("/keyword/<key_id>/<page_size>/<page_num>")]
fn search_notes_by_keyword<'a>(key_id: i64, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let nxk = db.note_keyword_pairs();
    let i = nxk.invert(key_id).unwrap();
    Json(page_iter!(i, page_size, page_num))
}

fn main() {
    let config = MaterialsConfig{
        db_file: "materials.sqlite"
    };

    rocket::ignite().
        manage(config).
        mount("/keyword", routes![get_keyword, search_keywords]).
        mount("/material", routes![get_material, search_materials, search_materials_by_keyword, search_materials_by_note]).
        mount("/note", routes![get_note, search_notes, search_notes_by_keyword]).
        launch();
}
