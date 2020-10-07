use crate::MaterialsConfig;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/<note_id>")]
pub(crate) fn get_note(note_id: i64, db: State<MaterialsConfig>) -> String {
    db.notes().get(note_id).unwrap()
}

#[get("/search/<note_search>/<page_size>/<page_num>")]
pub(crate) fn search_notes<'a>(note_search: String, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let norm = db.notes();
    let i = norm.search(note_search.as_str()).unwrap();
    Json(page_iter!(i, page_size, page_num))
}

#[get("/keyword/<key_id>/<page_size>/<page_num>")]
pub(crate) fn search_notes_by_keyword<'a>(key_id: i64, page_size: usize, page_num: usize, db: State<MaterialsConfig>) -> Json<Vec<i64>> {
    let nxk = db.note_keyword_pairs();
    let i = nxk.invert(key_id).unwrap();
    Json(page_iter!(i, page_size, page_num))
}
