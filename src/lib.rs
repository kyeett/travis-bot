#![crate_name = "travis_bot"]
#![crate_type = "lib"]

#![doc(html_root_url = "https://kyeett.github.io/travis-bot/")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/kyeett/travis-bot/master/demo.png")]


//! This is a comment.

/// This function always returns true. It's very useful!
pub fn always_true() -> bool { true }

#[test]
fn it_works() {
    assert_eq!(always_true(), true)
}
