use alloc::string::{String, ToString};

pub fn get_unit_prefix(magnitude: i8) -> String {
    if magnitude.eq(&0) {
        "".to_string()
    } else if magnitude.eq(&-3) {
        "m".to_string()
    } else if magnitude.eq(&-6) {
        "my".to_string()
    } else if magnitude.eq(&1) {
        "10".to_string()
    } else if magnitude.eq(&2) {
        "100".to_string()
    } else if magnitude.eq(&3) {
        "k".to_string()
    } else if magnitude.eq(&4) {
        "10k".to_string()
    } else if magnitude.eq(&5) {
        "100k".to_string()
    } else if magnitude.eq(&6) {
        "M".to_string()
    } else if magnitude.eq(&9) {
        "T".to_string()
    } else {
        let mut base = "1e".to_string();
        base += &magnitude.to_string();
        base
    }
}
