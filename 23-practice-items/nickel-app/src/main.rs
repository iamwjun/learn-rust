#[macro_use] extern crate nickel;

use std::io::Write;
use nickel::status::StatusCode::NotFound;
use nickel::{Nickel, NickelError, HttpRouter, Action, Continue, Halt, Request};

fn main() {
    let mut server = Nickel::new();

    fn custom_404<'a>(err: &mut NickelError, _req: &mut Request) -> Action {
        if let Some(ref mut res) = err.stream {
            if res.status() == NotFound {
                let _ = res.write_all(b"page not found");
                return Halt(())
            }
        }
        Continue(())
    }

    let custom_handler: fn(&mut NickelError, &mut Request) -> Action = custom_404;

    server.get("/bar", middleware!("This is the /bar handler"));

    server.get("/user/:userid", middleware! {|request|
        format!("this user's id is {:?}", request.param("userid"))
    });

    server.get("/a/*/d", middleware! {"matchs /a/b/d not /a/b/c/d"});

    server.get("/a/**/d", middleware! {"This matches /a/b/d and also /a/b/c/d"});

    server.handle_error(custom_handler);

    server.listen("127.0.0.1:6767").unwrap();
}
