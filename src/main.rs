use rocket::{get, launch, routes};

#[get("/")]
fn hello() -> String {
    let mut txt = String::from(
        "Ybyrá: Strong, Open, and Decentralized—The Future of Code Sharing!\n"
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}

