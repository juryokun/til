#[derive(Debug)]
struct St {
    id: i32,
    cd: String,
}

use std::borrow::Borrow;

use super::Rc;
fn rc_run() {
    let a = Rc::new(St {
        id: 1,
        cd: "aaa".to_string(),
    });
}

#[test]
fn test_rc() {
    rc_run();
}
