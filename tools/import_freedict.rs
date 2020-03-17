use std::io::BufReader;
use std::fs::File;
use quick_xml::Reader;
use quick_xml::events::Event;

fn main() -> std::io::Result<()> {
    // initialize file reader
    let file = File::open("../data/eng-jpn.tei")?;
    let file_reader = BufReader::new(file);

    // initialize xml reader
    let mut reader = Reader::from_reader(file_reader);
    reader.trim_text(true);

    // tracks number of entry tags encountered
    let mut entry_count = 0;
    // buffer for reader event data
    let mut buf = Vec::new();
    // text buffer for reader text events
    let mut txt = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"text" => println!("TEXT TAG!"),
                    b"sense" => {
                        println!("SENSE TAG! attributes values: {:?}",
                                        e.attributes().map(|a| String::from_utf8(a.unwrap().value.to_vec()).unwrap()).collect::<Vec<_>>());
                    },
                    b"entry" => entry_count += 1,
                    // quote tag begin, prepare txt buffer
                    b"quote" => txt.clear(),
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"quote" => {
                        println!("QUOTE TAG! content: {:?}",txt);
                        txt.clear();
                    },  
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();

        if entry_count > 10 {
            break;
        }
    }

    // done
    Ok(())
}