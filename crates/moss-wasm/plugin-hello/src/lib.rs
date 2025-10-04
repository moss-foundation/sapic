wit_bindgen::generate!({
    world: "plugin",
    path: "../wit/plugin-hello.wit"
});

struct Component;

impl Guest for Component {
    fn run(name: String) -> String {
        format!("Hello, {name} from plugin!")
    }
}

export!(Component);
