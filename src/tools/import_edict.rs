extern crate diesel;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::path::Path;

use langis::database;
use langis::models::{NewSource, NewWordTranslation, Source};
use langis::schema;
use langis::tool_helpers;

/// main
fn main() -> std::io::Result<()> {
    // get input file path from command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: `import-edict path/to/[converted-edict2-or-cedict-file]`");
        println!("  file must be specified");
        return Ok(());
    }
    let filename = &args[1];

    // try to guess language from filename
    let lang_id = if filename.contains("edict2") {
        "jpn".to_string()
    } else if filename.contains("cedict") {
        "zho".to_string()
    } else {
        panic!("Could not guess language from filename, sorry")
    };

    // connect to database
    let conn = database::establish_connection();

    // initialize file reader
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);

    // find or create sources record
    let source_name = Path::new(filename).file_stem().unwrap().to_str().unwrap();
    let source = tool_helpers::find_or_create_source(&conn, source_name.to_string());

    // tracks number of entries encountered
    let mut entry_count = 0;

    // begin
    println!("Beginning import of edict file with orth language: {:?}, quote language: \"eng\"", lang_id);

    // Consumes a lazy iterator, reading the file line-by-line
    for line in file_reader.lines() {
        if let Ok(ip) = line {
            if ip.starts_with("#") {
                // skip lines that begin with #
            } else if ip.starts_with("　？？？") {
                // skip the edict2 header line
            } else {
                println!("{}", &ip);
                // TODO

                // incr
                entry_count += 1;
            }
        } else if let Err(er) = line {
            if er.kind() == ErrorKind::InvalidData {
                println!("Invalid data encountered, perhaps this edict file has not been converted to utf8?");
            } else {
                println!("Unknown Error\n{:?}", er);
            }
            return Err(er);
        }

        if entry_count > 10 {
            break;
        }
    }

    // update sources table with last_updated_at
    tool_helpers::update_source(&conn, source.id);

    // done
    println!("Finished, processed {:?} entries", entry_count);
    Ok(())
}