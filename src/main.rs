extern crate reqwest;
extern crate quick_xml;


use std::io;
// use std::io::BufReader;
use std::fs::File;
// use quick_xml::Reader;
use quick_xml::events::Event;


fn fetch_xml() {
   
    let client = reqwest::Client::new();

    let mut response = client.get("https://datex.trafikverket.se/D2ClientPull/MetaDataBA/2_3/WeatherMetaData")
        .basic_auth("LTU", Some("DatexLTU2018#"))
        .send()
        .expect("Connection failed to Datex");
    assert!(response.status().is_success());


    let mut file = File::create("station_data.xml")
        .expect("Error creating file, station_data");
    io::copy(&mut response, &mut file)
        .expect("Failed to read response to file");

}



fn read_file() {

    // let file = File::open("station_data").unwrap();
    // let buffered = BufReader::new(file);
    // let mut reader = Reader::from_reader(buffered);
    let mut xml = quick_xml::Reader::from_file("station_data").expect("Failed to open file!");
    xml.trim_text(true);


    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        match xml.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"ns0:roadNumber" => {
                txt.push(
                    xml
                        .read_text(b"ns0:roadNumber", &mut Vec::new())
                        .expect("Could not decode text value"),
                );
                println!("{:?}", txt);
            }
            Ok(Event::Eof) => break, // END
            Err(e) => panic!("Error at position {}: {:?}", xml.buffer_position(), e),
            _ => (),       

        }
        buf.clear();

    }


}       

fn main() {

     read_file();
    // fetch_xml();
    
}