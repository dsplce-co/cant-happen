use regex::Regex;

use super::*;

#[test]
fn the_cause_is_included_works() {
    let message = "Regex::new should not fail";

    let result = std::panic::catch_unwind(|| {
        Regex::new(r"^\d{+$").cant_happen(message);
    });

    unsafe {
        let error = result.unwrap_err().downcast_unchecked::<String>();
        assert!(error.contains(message));
    }
}
