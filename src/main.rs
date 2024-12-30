use std::fs::File;
use std::io::Read;

use anyhow::Result;
use idyllic::app::App;
use idyllic::parse::parse_lines;

fn get_log() -> Result<String> {
    let mut log_file = File::open("data/sample-nginx.log")?;
    let mut buf = String::new();

    log_file.read_to_string(&mut buf)?;

    Ok(buf)
}

fn main() -> Result<()> {
    let log = get_log()?;
    // let mut terminal = ratatui::init();
    // let mut app = App::new(&log);

    // app.run(&mut terminal)?;

    let parsed = parse_lines(&log)?;

    dbg!(parsed);

    Ok(())
}
