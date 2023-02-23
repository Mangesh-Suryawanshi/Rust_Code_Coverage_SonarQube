#[macro_use] extern  crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to MS todo app using rust"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/",routes![index])
}