use rocket;


pub fn mount(attach: impl Fn(rocket::Rocket) -> rocket::Rocket) {
    use rocket_codegen::routes;
    use crate::controllers::*;

    attach(rocket::ignite())
        .mount("/",
               routes![hello_world::greet],
        ).launch();
}
