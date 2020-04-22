extern crate diesel;
extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::Path;

use langis::database;
use langis::edict_helpers;
use langis::app::models::{NewWordEntry};
use langis::tool_helpers;

lazy_static::lazy_static! {
    // parse language identifier from filename
    pub static ref ORTH_RGX: Regex = Regex::new(r"([^\[]+)(?:\[(.+)\])?").unwrap();
    // matches EntL ids from edict2 file
    pub static ref ENTL_RGX: Regex = Regex::new(r"^EntL(?:\d+)X?$").unwrap();
    // matches {bracket} tags
    pub static ref BRACKET_TAGS_RGX: Regex = Regex::new(r"([^{]*)(\{.+\})(.*)").unwrap();
    // list of edict grammar part-of-speech tags
    pub static ref EDICT_POS: Vec<&'static str> = vec!["adj-f","adj-i","adj-ix","adj-na",
        "adj-nari","adj-no","adj-pn","adj-t","adv","adv-to","aux","aux-adj","aux-v","conj",
        "cop","ctr","exp","int","n","n-adv","n-pref","n-suf","n-t","num","pn","pref","prt",
        "suf","v1","v1-s","v2a-s","v2b-k","v2d-s","v2g-k","v2g-s","v2h-k","v2h-s","v2k-k",
        "v2k-s","v2m-s","v2n-s","v2r-k","v2r-s","v2s-s","v2t-k","v2t-s","v2w-s","v2y-k",
        "v2y-s","v2z-s","v4b","v4g","v4h","v4k","v4m","v4r","v4s","v4t","v5aru","v5b","v5g",
        "v5k","v5k-s","v5m","v5n","v5r","v5r-i","v5s","v5t","v5u","v5u-s","vi","vk","vn","vr",
        "vs","vs-c","vs-i","vs-s","vt","vz"];
    // list of edict orth tags
    pub static ref EDICT_ORTH_TAGS: Vec<&'static str> = vec!["P","ik","iK","io","ateji","ok","oK","oik"];
    // list of edict bracket tags
    pub static ref EDICT_BRACKET_TAGS: Vec<&'static str> = vec!["anat","archit","astron","baseb","biol",
        "bot","Buddh","bus","chem","Christn","comp","econ","engr","finc","food","geol","geom","law",
        "ling","MA","mahj","math","med","mil","music","physics","Shinto","shogi","sports","sumo","zool"];
    // special tags from cedict file
    pub static ref CEDICT_TAGS: Vec<&'static str> = vec!["anatomy","archaic","behavior","brand","Buddhism","botany",
        "Cantonese","character","chemistry","coll.","colloquial","computing","dialect","derog.","fig.","finance","geology",
        "grammar","honorific","idiom","Internet slang","law","literary","linguistics","loanword","math.","meaning unclear",
        "medicine","military","music","name","old","onom.","physics","polite","proverb","slang","sports","Tw"];
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
        if let Ok(line_raw) = line {
            if line_raw.starts_with("#") {
                // skip lines that begin with #
            } else if line_raw.starts_with("　？？？") {
                // skip the edict2 header line
            } else {
                // fix specific lines in the edict2 and cedict files
                let line_text = if line_raw.starts_with("倍速 [ばいそく] /(adj-pn) (1) {comp} double-speed (drive, etc.}/") {
                    // bugged parenthesis
                    line_raw.replace("(drive, etc.}","(drive, etc.)")
                } else if line_raw.starts_with("如是 [にょぜ] /(n) (1) {Buddh} (See 如是我聞) (\"like this\"; often the opening word of a sutra)") {
                    // my parser cant handle quotes with only paren-text
                    line_raw.replace("(\"like this\"; often","\"like this\" (often")
                } else if line_raw.starts_with("唐棕櫚;唐棕梠 [とうじゅろ;トウジュロ] /(n) miniature Chusan palm (Trachycarpus wagnerianus)/(poss. Trachycarpus fortunei)") {
                    // combine these into single quote text
                    line_raw.replace("(Trachycarpus wagnerianus)/(poss. Trachycarpus fortunei)","(Trachycarpus wagnerianus) (poss. Trachycarpus fortunei)")
                } else if line_raw.starts_with("大牌檔 大牌档 [da4 pai2 dang4] /food stall/open-air restaurant (originally Hong Kong usage, now usually written as 大排檔|大排档[da4 pai2 dang4]") {
                    // unbalanced parentheses
                    line_raw.replace("大排檔|大排档[da4 pai2 dang4]","大排檔|大排档[da4 pai2 dang4])")
                } else if line_raw.starts_with("掖庭 掖庭 [ye4 ting2] /Lateral Courts in the imperial palace (housing concubines and administrative offices") {
                    // unbalanced parentheses
                    line_raw.replace("administrative offices","administrative offices)")
                } else if line_raw.starts_with("歹勢 歹势 [dai3 shi4] /(Tw) excuse me/to be sorry/(Taiwanese, Tai-lo pr. [pháinn-sè]") {
                    // unbalanced parentheses
                    line_raw.replace("Tai-lo pr. [pháinn-sè]","Tai-lo pr. [pháinn-sè])")
                } else if line_raw.starts_with("知人者智，自知者明 知人者智，自知者明 [zhi1 ren2 zhe3 zhi4 , zi4 zhi1 zhe3 ming2] /those who understand others are clever, but those who know themselves are truly wise (idiom, from Laozi's 道德經|道德经[Dao4 de2 jing1]") {
                    // unbalanced parentheses, also split the notes
                    line_raw.replace("(idiom, from Laozi's 道德經|道德经[Dao4 de2 jing1]","(idiom) (from Laozi's 道德經|道德经[Dao4 de2 jing1])")
                } else if line_raw.starts_with("能願動詞 能愿动词 [neng2 yuan4 dong4 ci2] /modal verb (e.g. 肯[ken3], 能[neng2], 會|会[hui4], 要[yao4], 該|该[gai1], 得[dei3], 願意|愿意[yuan4 yi4], 可以[ke3 yi3], 可能[ke3 neng2], 敢[gan3], 應該|应该[ying1 gai1]") {
                    // unbalanced parentheses
                    line_raw.replace("應該|应该[ying1 gai1]/","應該|应该[ying1 gai1])")
                } else { line_raw };

                // split the line by '/' to separate orth and quote parts
                let line_parts = edict_helpers::split_by_outer_slashes(&line_text);

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
                let quote_parts: Vec<String> = quote_parts_raw.iter().filter({|qp|
                    !(qp.is_empty() || ENTL_RGX.is_match(qp))
                }).map(|qp| qp.clone()).collect();


                // insert rows for each variation
                for op in orth_parts {
                    // for edict, parse out extra tags in the orth field
                    let mut collected_orth_tags = Vec::<String>::new();
                    let processed_orth = if lang_id == "jpn" {
                        let orth_tag_parts: Vec<&str> = op.split('(').collect();
                        for orth_tag_raw in orth_tag_parts[1..].into_iter() {
                            let orth_tag = orth_tag_raw.trim_end_matches(')');
                            if EDICT_ORTH_TAGS.contains(&orth_tag) {
                                collected_orth_tags.push(orth_tag.to_string());
                            } else {
                                println!("unknown orth tag: {:?}", orth_tag);
                            }
                        }

                        // return first part from split as processed_orth
                        orth_tag_parts[0]
                    } else {
                        // no processing for cedict
                        op
                    };

                    for (sense_idx, qp) in quote_parts.clone().into_iter().enumerate() {
                        // extract known notes from edict2 quote strings
                        // TODO extract See notes
                        let mut collected_notes = Vec::<String>::new();
                        let mut collected_tags = Vec::<String>::new();
                        let (qp_rem, qp_notes) = edict_helpers::extract_outer_paren_groups(&qp);
                        for qp_note in qp_notes {
                            // trim single leading and trailing ( )
                            let mut trimmed_note = qp_note.as_str();
                            if trimmed_note.chars().next() == Some('(') {
                                trimmed_note = &trimmed_note[1..];
                            }
                            if trimmed_note.chars().last() == Some(')') {
                                trimmed_note = &trimmed_note[..trimmed_note.len()-1];
                            }
                            // check if note is known tag
                            if (lang_id == "zho" && CEDICT_TAGS.contains(&trimmed_note)) ||
                               EDICT_POS.contains(&trimmed_note)
                            {
                                // known tag
                                collected_tags.push(trimmed_note.to_string());
                            } else {
                                // not known tag, store as note instead
                                collected_notes.push(trimmed_note.to_string());
                            }
                        }

                        // remainder str for { } extraction
                        let mut rqp = qp_rem.clone();

                        loop {
                            // ignore cedict line that describes curly brackets
                            if rqp.starts_with("curly brackets { }") {
                                break;
                            }

                            // parse out known {bracket} tags
                            let rqpc = rqp.clone();
                            let tag_match_opt = BRACKET_TAGS_RGX.captures(rqpc.as_str());
                            if let Some(tag_caps) = tag_match_opt {
                                rqp = (tag_caps.get(1).map_or("", |m| m.as_str()).trim().to_string() +
                                    " " + tag_caps.get(3).map_or("", |m| m.as_str()).trim()).trim().to_string();
                                if let Some(matched_tag) = tag_caps.get(2) {
                                    // sometimes bracket tags have multiple tags separated by ;
                                    let split_tags: Vec<&str> = matched_tag.as_str().split(';').collect();
                                    for split_tag in split_tags {
                                        let trimmed_tag = split_tag.trim_start_matches('{').trim_end_matches('}');
                                        if EDICT_BRACKET_TAGS.contains(&trimmed_tag) {
                                            collected_tags.push(trimmed_tag.to_string());
                                        } else {
                                            println!("WARNING! unknown bracket tag: {:?}", trimmed_tag);
                                            println!("  {:?}", rqpc);
                                            println!();
                                        }
                                    }
                                }
                            } else {
                                break;
                            }
                        }

                        // trim whitespace from remainder text
                        rqp = rqp.trim().to_string();

                        // if we ended up with empty quote text
                        if rqp == "" {
                            if collected_notes.len() == 1 && collected_notes[0] == "P" {
                                // ignore /(P)/ because it seems to be duplicated in the orth
                                // and i dont know how to handle this case
                                continue;
                            } else if collected_notes.len() == 1 && collected_notes[0] == "sometimes called \"negative electricity\"" {
                                // skip this quote/note because its hard to handle and hopefully not useful
                                continue;
                            } else if collected_notes.len() == 1 && collected_notes[0] == "sometimes called \"positive electricity\"" {
                                // also skip this one (there are actually two)
                                continue;
                            } else if collected_notes.len() == 1 && collected_notes[0] == "powerful Turkic confederation from medieval Inner Asia" {
                                // easiest to just turn this one into quote text
                                rqp = "powerful Turkic confederation from medieval Inner Asia".to_string();
                            } else {
                                // cedict has several entries with note text but no quote text
                                // edict should have none, so print warnings
                                if lang_id == "jpn" {
                                    println!("WARNING! unexpected empty quote: {:?}, notes: {:?}, tags: {:?}", rqp, collected_notes, collected_tags);
                                    println!("  {:?}", line_text);
                                    println!();
                                }
                            }
                        }

                        // show warning if nothing useful was parsed
                        if rqp == "" && collected_notes.len() == 0 && collected_tags.len() == 0 {
                            println!("WARNING! unexpected empty; quote: {:?}, notes: {:?}, tags: {:?}", rqp, collected_notes, collected_tags);
                            println!("  {:?}", line_text);
                            println!();
                        }


                        // insert word_entries record
                        let new_entry = NewWordEntry {
                            orth: processed_orth.to_string(),
                            orth_lang: lang_id.to_string(),
                            quote: rqp.to_string(),
                            quote_lang: "eng".to_string(),
                            sense: sense_idx as i32,
                            source_id: source.id
                        };
                        let word_entry_id = tool_helpers::insert_word_entry(&conn, new_entry);

                        // process notes for additional tags
                        let processed_notes = collected_notes.into_iter().map({|note|
                            // only for edict, not cedict
                            if lang_id == "jpn" {
                                // comp tags are just special notes
                                if note == "{comp}" {
                                    "comp".to_string()
                                // idk what unc stands for, but it refers to special grammar markings
                                } else if note == "unc" {
                                    "unc".to_string()
                                } else {
                                    // split by ,
                                    let split_note: Vec<&str> = note.split(',').collect();
                                    let split_note_len = (&split_note).len();
                                    // check if all parts are known edict pos tags
                                    let pos_notes: Vec<&str> = split_note.into_iter().filter({|n|
                                        EDICT_POS.contains(n)
                                    }).collect();
                                    if pos_notes.len() == 0 {
                                        //  no pos tags
                                        note.to_string()
                                    } else if pos_notes.len() == split_note_len {
                                        // all pos tags, store each as seperate tag
                                        for pos_note in pos_notes {
                                            collected_tags.push(pos_note.to_string());
                                        }
                                        // return empty note
                                        "".to_string()
                                    } else {
                                        // note with mixed pos and non-pos tags is an error case
                                        println!("note with unknown pos tags: {:?}", note);
                                        "".to_string()
                                    }
                                }
                            } else {
                                // no processing for cedict
                                note.to_string()
                            }
                        }).filter({|pn| !pn.is_empty()});

                        // insert notes
                        for note in processed_notes {
                            tool_helpers::insert_word_entry_note(&conn, word_entry_id, note);
                        }

                        // insert orth tags
                        for orth_tag in collected_orth_tags.clone() {
                            tool_helpers::insert_word_entry_tag(&conn, word_entry_id, orth_tag);
                        }

                        // insert other tags
                        for tag in collected_tags.clone() {
                            tool_helpers::insert_word_entry_tag(&conn, word_entry_id, tag);
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