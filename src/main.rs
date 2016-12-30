/*
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
extern crate clap;
extern crate clipboard;

use std::io;
use std::io::prelude::*;
use std::process::exit;
use clap::{App, Arg, ArgGroup};
use clipboard::ClipboardContext;


fn copy() -> Result<(), Box<std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(input)?;
    Ok(())
}

fn paste() -> Result<(), Box<std::error::Error>> {
    let ctx = ClipboardContext::new()?;
    let contents = ctx.get_contents()?;
    print!("{}", contents);
    Ok(())
}
fn main() {
    // Parse the command line arguments
    let matches = App::new("clip")
                      .version("1.0.0")
                      .author("Matthew S. <stanleybookowl@gmail.com")
                      .about("Copies and pastes text to and from the clipboard")
                      .group(ArgGroup::with_name("action"))
                      .arg(Arg::with_name("copy")
                               .help("Copies text from stdin to the clipboard")
                               .short("c")
                               .long("copy")
                               .group("action"))
                       .arg(Arg::with_name("paste")
                                .help("Pastes text from the clipboard to stdout. If no option is given, this is the default.")
                                .short("p")
                                .long("paste")
                                .group("action"))
                        .get_matches();

    if matches.is_present("copy") {
        // copy the text
        match copy() {
            Ok(_) => (),
            Err(e) => {
                io::stderr().write(&format!("Error: {:?}", e).into_bytes()).unwrap();
                exit(1);
            }
        };
    } else {
        // paste the text
        match paste() {
            Ok(_) => (),
            Err(e) => {
                io::stderr().write(&format!("Error: {:?}", e).into_bytes()).unwrap();
                exit(1);
            }
        };
    }

}
