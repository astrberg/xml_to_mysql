extern crate reqwest;
extern crate quick_xml;


use std::io;
use std::fs::File;
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

    let mut xml = quick_xml::Reader::from_file("station_data.xml").expect("Failed to open file!");
    xml.trim_text(true);


    // let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        match xml.read_event(&mut buf) {

            Ok(Event::Start(ref e)) => {

                match e.name() {
                    b"ns0:measurementSiteRecord" => println!("id: {}", &xml.read_text(e.attributes(), &mut Vec::new().unwrap()), 
                    b"ns0:roadNumber" => println!("{}", &xml.read_text(e.name(), &mut Vec::new()).unwrap()),

                    _ => (),    
                }

            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error {}: {:?}", xml.buffer_position(), e),
            _ => (),
            
        }
        buf.clear();
    }


}       

fn main() {

     read_file();
    // fetch_xml();
    
}