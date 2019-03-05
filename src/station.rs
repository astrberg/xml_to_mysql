use std::io;
use std::fs::File;
use quick_xml::Reader;
use quick_xml::events::Event;
use mysql::{Pool, Opts};
use mysql::OptsBuilder;

#[derive(Debug)]
pub struct StationData {
    id: String,
    name:  String,
    road_number: String,
    county_number: String,
    latitude: String,
    longitude: String,
}

// Get the XML file from datex using basic auth
pub fn fetch_xml(url: &str, user: &str, pass: &str) {
   
    let client = reqwest::Client::new();

    let mut response = client.get(url)
        .basic_auth(user, Some(pass))
        .send()
        .expect("Connection failed to Datex");
    assert!(response.status().is_success());


    let mut file = File::create("station_data.xml")
        .expect("Error creating file, station_data");
    io::copy(&mut response, &mut file)
        .expect("Failed to read response to file");

}


// Parse xml file and return station_data vector
pub fn read_file(xmlfile: &str) -> Vec<StationData> {

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
                                    // Utf8 to String
                                    station.id = String::from_utf8(attr.value.clone().into_owned()).unwrap()

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
    // Vec<StationData>
    station_data

} 

pub fn create_mysql_tables(opts: Opts) {

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
pub fn get_opts(user: &str, pass: &str, addr: &str, database: &str) -> Opts {
    // let user = "mysql";
    // let address = "127.0.0.1";
    let pass: String = ::std::env::var(pass).unwrap_or(pass.to_string());
    let port: u16 = ::std::env::var("3306").ok().map(|my_port| my_port.parse().ok().unwrap_or(3306)).unwrap_or(3306);

    let mut builder = OptsBuilder::default();
    
    builder.user(Some(user))
            .pass(Some(pass))
            .ip_or_hostname(Some(addr))
            .tcp_port(port)
            .db_name(Some(database));
    builder.into()
    
}

pub fn insert_station_data(opts: Opts, station_data: Vec<StationData>) {

    // Create new pool connection 
    let pool = Pool::new(opts).unwrap();

    for mut stmt in pool.prepare(r"INSERT INTO station_data (id, lat, lon, name, road_number, county_number) 
                                    VALUES (:id, :latitude, :longitude, :name, :road_number, :county_number);").into_iter() {
        
        for i in station_data.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
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
