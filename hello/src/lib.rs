struct Component;

use bindings::exports::component::hello::say_hi::SayHi;
use bindings::exports::component::hello::say_bye::SayBye;

impl SayHi for Component {
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}

impl SayBye for Component {
    fn bye() -> String {
        "See ya, next time!".to_string()
    }
}

bindings::export!(Component);
