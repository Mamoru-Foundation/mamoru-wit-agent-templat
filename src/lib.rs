wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "mamoru-core",

    path: "wit", //"wit",
});
struct Agent {}

impl Guest for Agent {
    fn run() {
        println!("-------------------------> My dummy sui");
    }
}

export!(Agent);


