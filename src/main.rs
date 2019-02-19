extern crate reqwest;
extern crate quick_xml;


use std::io;
use std::fs::File;
use std::string::FromUtf8Error;
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

    let mut id_arr = Vec::new();
    // let mut road_number = Vec::new();

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
                        // let mut attr = e.attributes().map(|attr| attr.unwrap().value).collect::<Vec<_>>();
                        // id_arr.push(get_attribute_value(&attr);
                        // println!("{:?}", id_arr);
                        println!("{:?}", id_arr);


                    // let attr = e.attributes().collect::<Vec<_>>();
                         // id_arr.push(format!("{}", from_utf8(e.attributes().key.unwrap())));
                    } 
                    // b"ns0:roadNumber" => {
                    //     road_number.push(xml.read_text(e.name(), &mut Vec::new()).unwrap());
                    //     println!("{:?}", road_number);
                    // }

                    _ => (),

            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error {}: {:?}", xml.buffer_position(), e),

            _ => (),
        }
        buf.clear();

    }


}       

fn get_attribute_value(attr: &Attribute) -> Result<String, FromUtf8Error> {
    let value = (&attr.value).clone().into_owned();
    String::from_utf8(value)
}

fn main() {
     read_file();
    // fetch_xml();
    
}