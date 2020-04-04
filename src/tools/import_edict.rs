extern crate diesel;
extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::Path;

use langis::database;
use langis::models::{NewWordTranslation};
use langis::tool_helpers;

lazy_static::lazy_static! {
    // parse language identifier from filename
    pub static ref ORTH_RGX: Regex = Regex::new(r"([^\[]+)(?:\[(.+)\])?").unwrap();
    // matches EntL ids from edict2 file
    pub static ref ENTL_RGX: Regex = Regex::new(r"^EntL(?:\d+)X?$").unwrap();
    // matches pos tags and See tags
    pub static ref TAGS_RGX: Regex = Regex::new(r"([^(]*)(\([^)]+\))(.*)").unwrap();
    // matches {comp} tags
    pub static ref COMP_RGX: Regex = Regex::new(r"([^(]*)(\{comp\})(.*)").unwrap();
}

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
                // split the line by '/' to separate orth and quote parts
                let line_parts: Vec<&str> = ip.split("/").collect();

                // match orth part with optional reading group
                let orth_caps = ORTH_RGX.captures(line_parts.first().unwrap()).unwrap();

                let orth = orth_caps.get(1).map_or("", |m| m.as_str()).trim();
                let readings = orth_caps.get(2).map_or("", |m| m.as_str()).trim();

                // edict2 readings are split by ;, cedict readings are split by a space
                let orth_splitter = if lang_id == "zho" { " " } else { ";" };
                let orth_parts: Vec<&str> = orth.split(orth_splitter).collect();

                // edict2 can have multiple readings split by ;, cedict does not have multiple readings
                let reading_parts: Vec<&str> = if lang_id == "zho" { 
                    vec![readings] 
                } else {
                    readings.split(";").collect()
                };

                // split quote parts (definitions) by /
                let quote_parts_raw = &line_parts[1..];
                // filter out EntL ids and empty strings
                let quote_parts: Vec<&str> = quote_parts_raw.iter().filter({|qp|
                    !(qp.is_empty() || ENTL_RGX.is_match(qp))
                }).map(|qp| qp.clone()).collect();
                

                // insert rows for each variation
                for op in orth_parts {
                    for (sense_idx, qp) in quote_parts.clone().into_iter().enumerate() {
                        // extract pos and See tags from edict2 quote strings
                        let mut rqp = qp.clone().to_string();
                        let mut collected_tags = Vec::<String>::new();
                        loop {
                            // parse out pos tags and See tags
                            // TODO restrict parsing to only known tags, ignore other stuff between parens
                            let rqpc = rqp.clone();
                            let tag_match_opt = TAGS_RGX.captures(rqpc.as_str());
                            if let Some(tag_caps) = tag_match_opt {
                                rqp = (tag_caps.get(1).map_or("", |m| m.as_str()).trim().to_string() + 
                                    " " + tag_caps.get(3).map_or("", |m| m.as_str()).trim()).trim().to_string();
                                if let Some(matched_tag) = tag_caps.get(2) {
                                    collected_tags.push(matched_tag.as_str().to_string());
                                }
                            } else {
                                break;
                            }
                        }

                        loop {
                            // parse out {comp} tags
                            let rqpc = rqp.clone();
                            let comp_match_opt = COMP_RGX.captures(rqpc.as_str());
                            if let Some(comp_caps) = comp_match_opt {
                                rqp = (comp_caps.get(1).map_or("", |m| m.as_str()).trim().to_string() + 
                                    " " + comp_caps.get(3).map_or("", |m| m.as_str()).trim()).trim().to_string();
                                if let Some(matched_tag) = comp_caps.get(2) {
                                    collected_tags.push(matched_tag.as_str().to_string());
                                }
                            } else {
                                break;
                            }
                        }

                        // insert word_translations record
                        let new_entry = NewWordTranslation {
                            orth: op.to_string(),
                            orth_lang: lang_id.to_string(),
                            quote: rqp.to_string(),
                            quote_lang: "eng".to_string(),
                            sense: sense_idx as i32,
                            source_id: source.id
                        };
                        let word_translation_id = tool_helpers::insert_word_translation(&conn, new_entry);

                        // insert collected tags
                        for note in collected_tags {
                            tool_helpers::insert_notes_and_tags(&conn, word_translation_id, note);
                        }
                    }
                }

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
    }

    // update sources table with last_updated_at
    tool_helpers::update_source(&conn, source.id);

    // done
    println!("Finished, processed {:?} entries", entry_count);
    Ok(())
}