use crate::MaterialsConfig;
use rocket::State;
use rocket_contrib::json::Json;
use serde::ser::{Serialize, Serializer, SerializeStruct};

use crate::{
    FILE_URL_COL,
    UNKNOWN_URL
};

pub struct MaterialDescriptor {
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

#[get("/keywords/<mat_id>")]
pub(crate) fn get_keywords(mat_id: i64, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let fxk = db.file_keyword_pairs();
    let keys = fxk.get(mat_id).unwrap();
    Json(keys.collect())
}

#[get("/<mat_id>")]
pub(crate) fn get_material(mat_id: i64, db: State<MaterialsConfig>) -> Json<MaterialDescriptor> {
    let f = db.files();
    let name = f.get(mat_id).unwrap();
    let url = f.get_nonkey(mat_id, FILE_URL_COL).unwrap_or(UNKNOWN_URL);
    Json(MaterialDescriptor{
        id: mat_id,
        name: name,
        url: url,
    })
}

#[get("/key/<mat_id>/<key_id>")]
pub(crate) fn key_material<'a>(mat_id: i64, key_id: i64, db: State<MaterialsConfig>) -> String {
    let fxk = db.file_keyword_pairs();
    match fxk.insert(mat_id, key_id) {
        Ok(_) => "Ok".to_string(),
        Err(e) => e.msg.to_string(),
    }
}

#[get("/search/<key_search>/<page_size>/<page_num>")]
pub(crate) fn search_materials<'a>(key_search: String, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let f = db.files();
    let i = f.search(key_search.as_str()).unwrap();
    Json(page_iter!(i, page_size, page_num))
}

#[get("/keyword/<key_id>/<page_size>/<page_num>")]
pub(crate) fn search_materials_by_keyword<'a>(key_id: i64, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let fxk = db.file_keyword_pairs();
    let i = fxk.invert(key_id).unwrap();
    Json(page_iter!(i, page_size, page_num))
}

#[get("/note/<note_id>/<page_size>/<page_num>")]
pub(crate) fn search_materials_by_note<'a>(note_id: i64, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let fxn = db.file_note_pairs();
    let i = fxn.invert(note_id).unwrap();
    Json(page_iter!(i, page_size, page_num))
}
