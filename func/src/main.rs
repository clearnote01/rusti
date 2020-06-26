mod fractal;
use clap::clap_app;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;

macro_rules! yo {
    (
        $( $name:expr ),*
    ) => {
        {
            $(
                println!("Yo {}!", $name);
            )*
        }
    };
}

macro_rules! map {
    (
        $(
            $k:tt :  $v:expr
        ),*
    ) => {
        {
            let mut m = HashMap::new();
            $(
                println!("Dict key {} val {}", $k, $v);
                m.insert($k, $v);
            )*
            m
        }
    }
}

//
// dict!({ "a": "b", "c": "d"})
//
//
//
//
//
fn main() -> anyhow::Result<()> {
    let f = File::open("fire.txt");
    println!("Hello, world!");
    println!("Fire");
    yo!("Saika", "Madokka");
    map! {
        "a_value":"b",
        "c": "c_value"
    };
    map! {
        1: "some",
        2: "day"
    };
    // fractal::run();
    let data = "http://www.foobar.com/my/route";
    let re = Regex::new(
        r"(?x)

        (?P<protocol>http|ftp|https)  # some useful comment here
        ://
        (?P<site>[a-zA-Z0-9\.]+) # can't think of anything useful 
        /
        (?P<route>.*) # get the route 
    ",
    )?;
    let matches = re.captures(&data).unwrap();
    assert_eq!(matches["protocol"].parse::<String>()?, "http");
    assert_eq!(matches["site"].parse::<String>()?, "www.foobar.com");
    assert_eq!(matches["route"].parse::<String>()?, "my/route");
    Ok(())
}

fn summer(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn t() {
    assert_eq!(summer(5, 10), 15);
    assert_eq!(summer(5, 10), 15);
}
