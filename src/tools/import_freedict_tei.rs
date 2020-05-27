extern crate diesel;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;
use diesel::PgConnection;
use dotenv;
use regex::Regex;

use langis::app::models::{NewWordEntry,NewWordEntryGroup};
use langis::app::database;
use langis::helpers::tool_helpers;

/// enum for tracking the state of which buffer to read body text into
#[derive(Copy, Clone)]
enum WhichTextBuf {
    OrthTxt,
    QuoteTxt,
    PosTxt,
    None
}

/// main
fn main() -> std::io::Result<()> {
    // get input file path from command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: `import-freedict-tei path/to/[lang]-[lang].tei`");
        println!("  tei file must be specified");
        return Ok(());
    }
    let filename = &args[1];

    // parse language identifier from filename
    let lang_re = Regex::new(r"(\w+)\-(\w+)\.tei").unwrap();
    let lang_re_caps = lang_re.captures(filename).unwrap();
    let orth_lang = lang_re_caps.get(1).unwrap().as_str();
    let quote_lang = lang_re_caps.get(2).unwrap().as_str();

    // connect to database
    dotenv::dotenv().ok();
    let db = database::get_database_pool();
    let conn: &PgConnection = &db.get().unwrap();

    // initialize file reader
    let file = File::open(filename)?;
    let file_reader = BufReader::new(file);

    // initialize xml reader
    let mut reader = Reader::from_reader(file_reader);
    reader.trim_text(true);

    // buffer for reader event data
    let mut buf = Vec::new();
    // tracks number of entry tags encountered
    let mut entry_count = 0;

    // text buffer for orth-tag reader text events
    let mut orth_txt = Vec::new();
    // text buffer for quote-tag reader text events
    let mut quote_txt = Vec::new();
    // text buffer for pos-tag reader text events
    let mut pos_txt = Vec::new();
    // tracks the sense offset if available
    let mut sense_idx = 0;
    // tracks which buffer should be expecting the next text event
    let mut txt_which = WhichTextBuf::None;

    // find or create sources record
    let source_name = format!("freedict-{}-{}.tei", orth_lang, quote_lang);
    let source = tool_helpers::find_or_create_source(&conn, source_name);

    // begin
    println!("Beginning import of tei with orth language: {:?}, quote language: {:?}", orth_lang, quote_lang);

    // store next group_id
    let mut group_id: i32 = 0;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    // entry-tag begin, count and reset buffers
                    b"entry" => {
                        entry_count += 1;
                        orth_txt.clear();
                        quote_txt.clear();
                        pos_txt.clear();
                        sense_idx = 0;

                        // insert word_entry_groups record
                        let new_group = NewWordEntryGroup {
                            source_id: source.id
                        };
                        group_id = tool_helpers::insert_word_entry_group(&conn, new_group);
                    },
                    // sense-tag begin
                    b"sense" => {
                        // parse n attribute from sense tag
                        let attr_txts = e.attributes().filter(|a| {
                            if let Ok(au) = a.as_ref() {
                                String::from_utf8(au.key.to_vec()).unwrap() == "n"
                            } else {
                                false
                            }
                        }).collect::<Vec<_>>();
                        if let Some(attr_result) = attr_txts.first() {
                            let attr_txt = attr_result.as_ref().unwrap().unescape_and_decode_value(&reader).unwrap();
                            // try parsing it to an interger
                            if let Ok(attr_sense_idx) = attr_txt.parse::<i32>() {
                                // note sense offset
                                sense_idx = attr_sense_idx;
                            } else {
                                // sense tag with no offset, assume 0
                                sense_idx = 0;
                            }
                        } else {
                            // sense tag with attributes, assume 0
                            sense_idx = 0;
                        }
                    },
                    // orth tag begin, prepare txt buffer
                    b"orth" => {
                        orth_txt.clear();
                        txt_which = WhichTextBuf::OrthTxt;
                    },
                    // pos tag begin, prepare txt buffer
                    b"pos" => {
                        pos_txt.clear();
                        txt_which = WhichTextBuf::PosTxt;
                    },
                    // quote tag begin, prepare txt buffer
                    b"quote" => {
                        quote_txt.clear();
                        txt_which = WhichTextBuf::QuoteTxt;
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"quote" => {
                        // check if pos data was read
                        let pos_str = pos_txt.join("");
                        let pos_value = if pos_str.len() > 0 {
                            Some(pos_str.trim().to_string())
                        } else { None };

                        // for each quote tag ended, store an entry in the dict_entries table
                        let new_entry = NewWordEntry {
                            orth: orth_txt.join("").trim().to_string(),
                            orth_lang: orth_lang.to_string(),
                            quote: quote_txt.join("").trim().to_string(),
                            quote_lang: quote_lang.to_string(),
                            sense: sense_idx,
                            group_id: group_id
                        };
                        let word_entry_id = tool_helpers::insert_word_entry(&conn, new_entry);

                        // insert part of speech tag as note, if exists
                        // TODO parse normalized pos values from freedict pos tags
                        if let Some(pos) = pos_value {
                            tool_helpers::insert_word_entry_note(&conn, word_entry_id, format!("pos: {}",pos));
                        }

                        quote_txt.clear();
                    },
                    // if orth or pos tag end, reset txt_which
                    b"orth" => txt_which = WhichTextBuf::None,
                    b"pos" => txt_which = WhichTextBuf::None,
                    // ignore other tag close events
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                let txt = e.unescape_and_decode(&reader).unwrap();
                match txt_which {
                    WhichTextBuf::OrthTxt => orth_txt.push(txt),
                    WhichTextBuf::PosTxt => pos_txt.push(txt),
                    WhichTextBuf::QuoteTxt => quote_txt.push(txt),
                    WhichTextBuf::None => (), // ignore if we are not expecting text
                }
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    // update sources table with last_updated_at
    tool_helpers::update_source(&conn, source.id);

    // done
    println!("Finished, processed {:?} entries", entry_count);
    Ok(())
}
