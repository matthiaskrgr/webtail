use clap::{App, Arg};

fn main() {
    let cfg = App::new("webtail").arg(Arg::with_name("URL")).get_matches();
    let url = cfg.value_of("URL").unwrap_or("" /* default */);

    let refresh_rate = std::time::Duration::from_secs(10);

    println!("getting: {}", url);

    let mut content_old = String::new();

    loop {
        let mut req = reqwest::get(url).expect("Could not get url");
        let content = req.text().unwrap();

        let line_diff = content.lines().count() - content_old.lines().count();
        let old_lines = content.lines().count() - line_diff;
        // only print new lines (skip number of previous lines)
        content
            .lines()
            .skip(old_lines)
            .for_each(|line| println!("{}", line));

        std::thread::sleep(refresh_rate);
        content_old = content;
    }
}
