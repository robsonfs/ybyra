use rocket::{get, launch, routes};
use git2::Repository;

#[get("/")]
fn hello() -> String {
    let mut txt = String::from(
        "Ybyrá: Strong, Open, and Decentralized - The Future of Code Sharing!\n"
    );

    txt.push_str(
        r#"
                &&& &&  & &&
            && &\/&\|& ()|/ @, &&
            &\/(/&/&||/& /_/)_&/
        &() &\/&|()|/&\/ '%" & ()
        &_\/_&&_/\/&||/& /_/)_&/
        &&   && & &|& |&&/&--& &&
                |||/  \||/  % &/
                ||||    ||||   &&
                ||||    ||||
        /\  /|||\  /\  / ||\
        /  \/ ||| \/  \/  |||
        / /\  /|||/\  /\  /\||
        / /  \/ ||| \/  \/  \ ||
            |/           \|
    "#
    );

    txt
}

#[get("/")]
fn repositories() -> String {
    println!("I'm here");
    match Repository::open("/home/robson/Projects/open_source/ybyra") {
        Ok(repo) => format!("Repo Owner: {}",
            repo
                .config()
                .unwrap()
                .get_path("user.name")
                .unwrap()
                .to_str()
                .unwrap()
        ),
        Err(e) => format!("Error: {}", e)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/repositories", routes![repositories])
}

