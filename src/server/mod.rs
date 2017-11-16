pub mod api;
pub mod asset;
pub mod home;

use filesystem::walkdir::SortMethod;

/*
* SortQueries struct
*
* Query parameters needed to know what kind
* of sorting method use.
*
*/
#[derive(FromForm)]
pub struct SortQueries {
    by: String,
    ascending: Option<bool>,
}

fn parse_queries(queries: SortQueries) -> (SortMethod, bool) {
    let mut method = SortMethod::Name;
    let mut ascending = true;

    if queries.by == "time" {
        method = SortMethod::Time;
    }
    else
    if queries.by == "size" {
        method = SortMethod::Size;
    }

    if let Some(mode) = queries.ascending {
        ascending = mode;
    }

    return (method, ascending);
}
