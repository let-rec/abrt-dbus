mod dbus_server;
use crate::dbus_server::dbus_server;

mod match_signal;
use crate::match_signal::match_signal;

// mod dbus_tokio_server;
// use crate::dbus_tokio_server::tokio_server;
fn main() {
//  let c = Connection::new_session()?;
// c.request_name("com.example.dbustest", false, true, false)?;
// let mut cr = Crossroads::new();
// let token = cr.register("com.example.dbustest", |b| {
//     b.method("Hello", ("name",), ("reply",), |_, _, (name,): (String,)| {
//         Ok((format!("Hello {}!", name),))
//     });
// });
// cr.insert("/hello", &[token], ());
// cr.serve(&c)
    // let _ = dbus_server();
    match_signal();
    // tokio_server();
}







// use dbus::blocking::Connection;
// use std::time::Duration;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // First open up a connection to the session bus.
//     let conn = Connection::new_session()?;

//     // Second, create a wrapper struct around the connection that makes it easy
//     // to send method calls to a specific destination and path.
//     let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

//     // Now make the method call. The ListNames method call takes zero input parameters and
//     // one output parameter which is an array of strings.
//     // Therefore the input is a zero tuple "()", and the output is a single tuple "(names,)".
//     let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

//     // Let's print all the names to stdout.
//     for name in names { println!("{}", name); }

//     Ok(())
// }
