extern crate reqwest;
extern crate quick_xml;


use std::io;
use std::fs::File;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::attributes::Attribute;


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

    let mut xml = Reader::from_file("station_data.xml").expect("Failed to open file!");
    xml.trim_text(true);

    let mut id_arr = vec![];
    let mut station_name = vec![];
    let mut road_number = vec![];
    let mut county_number = vec![];
    let mut latitude = vec![];
    let mut longitude = vec![];
    let mut i = 0;


    let mut buf = Vec::new();

    loop {
        match xml.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => 
                
                match e.name() {
                    b"ns0:measurementSiteRecord" => {

                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"id" => {
                                    id_arr.push(get_attribute_value(attr));
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    }
                    b"ns0:value" => {
                        station_name.push(xml.read_text(e.name(), &mut Vec::new()).unwrap());
                    }
                    b"ns0:roadNumber" => {
                        road_number.push(xml.read_text(e.name(), &mut Vec::new()).unwrap());

                    }
                    b"ns0:countyNumber" => {
                        county_number.push(xml.read_text(e.name(), &mut Vec::new()).unwrap());

                    }
                    b"ns0:latitude" => {
                        latitude.push(xml.read_text(e.name(), &mut Vec::new()).unwrap());

                    }
                    b"ns0:longitude" => {
                        longitude.push(xml.read_text(e.name(), &mut Vec::new()).unwrap());

                    }

                    _ => (),

            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error {}: {:?}", xml.buffer_position(), e),

            _ => (),
        }
        buf.clear();

    }
    println!("{:?}", id_arr);
    println!("{:?}", station_name);
    println!("{:?}", road_number);
    println!("{:?}", county_number);
    println!("{:?}", latitude);
    println!("{:?}", longitude);
    println!{"{}", i};




}       

fn get_attribute_value(attr: &Attribute) -> String {
    let value = (&attr.value).clone().into_owned();
    String::from_utf8(value).unwrap()
}

fn main() {
     read_file();
    // fetch_xml();
    
}