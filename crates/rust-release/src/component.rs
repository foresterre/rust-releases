use crate::date::Date;
use crate::version::Version;

pub struct Component {
    id: String,
    version: Version,
    date: Option<Date>,
    // aliases: Option<Vec<String>>,
}
