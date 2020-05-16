use dbus::blocking::Connection;
use std::collections::HashMap;
use dbus::arg;
use std::string::String;
use std::time::Duration;

fn _list_names() {

    // First open up a connection to the session bus.
    let conn = Connection::new_session().expect("session not recvd");

    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    // Now make the method call. The ListNames method call takes zero input parameters and
    // one output parameter which is an array of strings.
    // Therefore the input is a zero tuple "()", and the output is a single tuple "(names,)".
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ()).expect("something");
    // Let's print all the names to stdout.
    for name in names { println!("{}", name); }
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn refarg(val: &dyn arg::RefArg) -> String {
    if let Some(data) = val.as_str() {
        return data.to_string();
    } else if let Some(data) = val.as_iter() {
        for i in data {
            return refarg(&i);
        }
        return "not found in iterable".to_string();
    } else {
        return "not found".to_string();
    }
}

#[derive(Debug)]
pub enum PlayStatus {
    _PAUSED,
    PLAYING,
    STOPPED
}

pub struct Song {
    pub title: String,
    pub artist: String,
    pub status: PlayStatus,
}

pub fn cur_song() -> Song {
    let conn = Connection::new_session().expect("expected");
    let proxy = conn.with_proxy("org.mpris.MediaPlayer2.spotify", "/org/mpris/MediaPlayer2", Duration::from_millis(5000));
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
    let metadata: Result<HashMap<String, arg::Variant<Box<dyn arg::RefArg>>>, _> = proxy.get("org.mpris.MediaPlayer2.Player", "Metadata");
    match metadata {
        Ok(val) => {
            // println!("metadata: {:?}", val);

            let title = val.get("xesam:title").expect("title not found");
            let title = refarg(&title);

            let artist = val.get("xesam:artist").expect("artist not found");
            let artist = refarg(&artist);
            return Song { title, artist, status: PlayStatus::PLAYING };
        },
        Err(err) => {
            println!("printing err: {:?}", err);
            return Song { title: "not found".to_string(), artist: "not found".to_string() , status:PlayStatus::STOPPED};
        }
    }
}
