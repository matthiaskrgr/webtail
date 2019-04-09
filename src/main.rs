use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use reqwest::get;

fn main() {
    let cfg = App::new("webtail").arg(Arg::with_name("URL")).get_matches();
    let url = cfg.value_of("URL").unwrap_or("" /* default */);

    println!("getting: {}", url);

    loop {
        let mut req = reqwest::get(url).expect("Could not get url");
        let content = req.text().unwrap();

        for i in content.lines() {
            println!("{}", i);
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    //println!("{:?}", content);
}
