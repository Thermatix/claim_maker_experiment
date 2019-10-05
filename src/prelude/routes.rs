use rocket;


pub fn mount() {
    use rocket_codegen::routes;
    use crate::controllers::*;
    rocket::ignite()
        .mount("/",
               routes![hello_world::greet],
        ).launch();
}
