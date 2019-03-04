extern crate reqwest;
extern crate quick_xml;
#[macro_use]
extern crate mysql;


use std::io;
use std::fs::File;

use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::attributes::Attribute;

use mysql::{Pool, Opts};
use mysql::OptsBuilder;

#[derive(Debug)]
struct StationData {
    id: String,
    name:  String,
    road_number: String,
    county_number: String,
    latitude: String,
    longitude: String,
}

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



fn read_file(xmlfile: &str) -> Vec<StationData> {

    let mut xml = Reader::from_file(xmlfile).expect("Failed to open file!");
    xml.trim_text(true);
    
    let mut lat_stored = false;
    let mut long_stored = false;

    let mut station_data = Vec::new();

    let mut buf = Vec::new();
    
    

    loop {
        
        match xml.read_event(&mut buf) {
            Ok(Event::Start(e)) => match e.name() {
                    b"ns0:measurementSiteRecord" => {
                        let station = StationData {

                            id: String::new(),
                            name: String::new(),
                            road_number: String::new(),
                            county_number: String::new(),
                            latitude: String::new(),
                            longitude: String::new(),
                        };
                        station_data.push(station);
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"id" => {
                                    let station = station_data.last_mut().unwrap();
                                    station.id = get_attr_value(attr);
                                }
                                    
                                Ok(_) => (),
                                Err(_) => (),
                            }
                        }
                    }
                    b"ns0:value" => {
                        let station = station_data.last_mut().unwrap();
                        station.name = xml.read_text(e.name(), &mut Vec::new()).unwrap();
                    }                                     
                    b"ns0:roadNumber" => {
                        let station = station_data.last_mut().unwrap();
                        station.road_number = xml.read_text(e.name(), &mut Vec::new()).unwrap();
                    }
                    b"ns0:countyNumber" => {
                        let station = station_data.last_mut().unwrap();
                        station.county_number = xml.read_text(e.name(), &mut Vec::new()).unwrap();
                    }
                    // For some reason latitude and longitude coordinates are stored twice in the XML file
                    b"ns0:latitude" => {
                        if lat_stored {
                            lat_stored = false;
                        } else {
                            let station = station_data.last_mut().unwrap();
                            station.latitude = xml.read_text(e.name(), &mut Vec::new()).unwrap();
                            lat_stored = true;
                        }

                    }
                    b"ns0:longitude" => {
                        if long_stored {
                            long_stored = false;
                        } else {
                            let station = station_data.last_mut().unwrap();
                            station.longitude = xml.read_text(e.name(), &mut Vec::new()).unwrap();
                            long_stored = true;
                        }

                    }
                           
                    _ => (), // There are several other `Event`s we do not consider here

            },
            Ok(Event::Eof) => break,  
            Err(e) => panic!("Error at pos {}: {:?}", xml.buffer_position(), e),

            _ => (),
        }
        buf.clear();
    }
    
    station_data

} 
     

fn get_attr_value(attr: &Attribute) -> String {
    let value = (&attr.value).clone().into_owned();
    String::from_utf8(value).unwrap()
}

fn create_mysql_tables() {

    let opts = get_opts();
    let pool = Pool::new(opts).unwrap();

    pool.prep_exec(r"CREATE TABLE `station_data` (
                        `id` char(20) NOT NULL,
                        `lat` float DEFAULT NULL,
                        `lon` float DEFAULT NULL,
                        `name` varchar(30) DEFAULT NULL,
                        `road_number` int(10) DEFAULT NULL,
                        `county_number` int(10) DEFAULT NULL,
                        PRIMARY KEY (`id`)
                    )", ()).unwrap();


}
// Setup connection to mysql
fn get_opts() -> Opts {
    let user = "mysql";
    let address = "127.0.0.1";
    let password: String = ::std::env::var("password").unwrap_or("password".to_string());
    let port: u16 = ::std::env::var("3306").ok().map(|my_port| my_port.parse().ok().unwrap_or(3306)).unwrap_or(3306);

    let mut builder = OptsBuilder::default();
    builder.user(Some(user))
            .pass(Some(password))
            .ip_or_hostname(Some(address))
            .tcp_port(port)
            .db_name(Some("db"));
    builder.into()
}

fn insert_station_data(station_data: Vec<StationData>) {
    
    let opts = get_opts();
    let pool = Pool::new(opts).unwrap();

    for mut stmt in pool.prepare(r"INSERT INTO station_data (id, lat, lon, name, road_number, county_number) 
                                    VALUES (:id, :latitude, :longitude, :name, :road_number, :county_number);").into_iter() {
        
        for i in station_data.iter() {
            stmt.execute(params!{
                "id" => i.id.clone(),
                "latitude" => i.latitude.clone(),
                "longitude" => i.longitude.clone(),
                "name" => i.name.clone(),
                "road_number" => i.road_number.clone(),
                "county_number" => i.county_number.clone(),
            }).unwrap();
        }
    }
    
}

fn main() {
    // insert_station_data();
    let stations = read_file("station_data.xml");
    // for i in stations.iter() {
    //     println!("{:?}", i.id);

    // }
    insert_station_data(stations);
    // fetch_xml();
    // create_mysql_tables();
}