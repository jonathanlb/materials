use crate::MaterialsConfig;
use rocket::State;
use rocket_contrib::json::Json;
    
#[get("/<key_id>")]
pub(crate) fn get_keyword(key_id: i64, db: State<MaterialsConfig>) -> String {
    db.keywords().get(key_id).unwrap()
}

#[get("/search/<key_search>/<page_size>/<page_num>")]
pub(crate) fn search_keywords<'a>(
    key_search: String, page_size: usize, page_num: usize, db: State<MaterialsConfig>
) -> Json<Vec<i64>> {
    let norm = db.keywords();
    let i = norm.search(key_search.as_str()).unwrap();
    Json(page_iter!(i, page_size, page_num))
}
