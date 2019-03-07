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
#[derive(Debug)]
pub struct WeatherData {
    station_id: String,
    timestamp: String,
    road_temperature: String,
    air_temperature: String,
    air_humidity: String,
    wind_speed: String,
    wind_direction: String,

}

// Parse xml file and return station_data vector
pub fn parse_station(xmlfile: &str) -> Vec<StationData> {

    let mut xml = Reader::from_file(xmlfile).expect("Failed to open file!");
    xml.trim_text(true); //remove whitespaces
    
    let mut lat_stored = false;
    let mut long_stored = false;

    let mut station_data = Vec::new();
    let mut buf = Vec::new();

    loop {
        
        match xml.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
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
                        let station = station_data.last_mut().unwrap();
                        // Get station id
                        station.id = e.attributes()
                                    .filter_map(|a| a.ok())
                                    .find(|a| a.key == b"id")
                                    .expect("Failed to find id!")
                                    .unescape_and_decode_value(&xml)
                                    .expect("Failed to decode id!");

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

            }
            Ok(Event::Eof) => break,  
            Err(e) => panic!("Error at pos {}: {:?}", xml.buffer_position(), e),

            _ => (),
        }
        buf.clear();
    }
    // Vec<StationData>
    station_data

}

pub fn parse_weather(xmlfile: &str) -> Vec<WeatherData> {
     #[derive(Clone, Copy)]
    enum State {
        Root,
        Air,
        Road,
        Humidity,
        Wind,
    };

    let mut xml = Reader::from_file(xmlfile).expect("Failed to open file!");
    xml.trim_text(true); //remove whitespaces
    
    let mut weather_data = Vec::new();
    let mut buf = Vec::new();
    // let mut ns_buffer = Vec::new();
    let mut state = State::Root;
    loop {
        
        match xml.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name() {
                b"measurementSiteReference" => {
                    let weather = WeatherData {

                        station_id: String::new(),
                        timestamp: String::new(),
                        road_temperature: String::new(),
                        air_temperature: String::new(),
                        air_humidity: String::new(),
                        wind_speed: String::new(),
                        wind_direction: String::new(),

                    };
                    weather_data.push(weather);
                    let weather = weather_data.last_mut().unwrap();
                    // Get station id
                    weather.station_id = e.attributes()
                                .filter_map(|a| a.ok())
                                .find(|a| a.key == b"id")
                                .expect("Failed to find id!")
                                .unescape_and_decode_value(&xml)
                                .expect("Failed to decode id!");
                    }
                _ => {}
            }
            Ok(Event::Start(ref e)) => {
                match (state, e.name()) {
                    (State::Root, b"airTemperature") => state = State::Air,
                    (State::Air, b"temperature") => {
                            let weather = weather_data.last_mut().expect("Failed to get pointer, airTemperature");
                            weather.air_temperature = xml.read_text(e.name(), &mut Vec::new()).expect("Failed to read text at airTemperature");
                    }
                    (State::Root, b"measurementTimeDefault") => {
                        let weather = weather_data.last_mut().expect("Failed to get pointer, measurementTimeDefault");
                        weather.timestamp = xml.read_text(e.name(), &mut Vec::new()).expect("Failed to read text at measurementTimeDefault");
                    }
                    (State::Root, b"roadSurfaceTemperature") => state = State::Road,
                    (State::Road, b"temperature") => {
                        let weather = weather_data.last_mut().expect("Failed to get pointer, roadSurfaceTemperature");
                        weather.road_temperature = xml.read_text(e.name(), &mut Vec::new()).expect("Failed to read text at roadSurfaceTemperature");
                        
                    }            
                    (State::Root, b"relativeHumidity") => state = State::Humidity,
                    (State::Humidity, b"percentage") => {
                        let weather = weather_data.last_mut().expect("Failt to get pointer, relativeHumidity");
                        weather.air_humidity = xml.read_text(e.name(), &mut Vec::new()).expect("Failed to read text at relativeHumidity");
                        
                    }                                     
                    (State::Root, b"windSpeed") => state = State::Wind,
                    (State::Wind, b"speed") => { 
                        let weather = weather_data.last_mut().expect("Failed to get pointer, windSpeed");
                        weather.wind_speed = xml.read_text(e.name(), &mut Vec::new()).expect("Failed to read text at windSpeed");
                        
                    }
                    (State::Root, b"directionCompass") => {
                        let weather = weather_data.last_mut().expect("Failed to get pointer, directionCompass");
                        weather.wind_direction = xml.read_text(e.name(), &mut Vec::new()).expect("Failed to read text at directionCompass");
                            
                    }
                    _ => {} // There are several other `Event`s we do not consider here
                }
            }
            
            Ok(Event::End(ref e)) => {
                match (state, e.name()) {
                    (State::Air, b"airTemperature") => state = State::Root,
                    (State::Road, b"roadSurfaceTemperature") => state = State::Root,
                    (State::Humidity, b"relativeHumidity") => state = State::Root,
                    (State::Wind, b"windSpeed") => state = State::Root,


                    _ => {}
                }
            }
            Ok(Event::Eof) => break,  
            Err(e) => panic!("Error at pos {}: {:?}", xml.buffer_position(), e),

            _ => (),
        }
        buf.clear();
    }

    // Vec<WeatherData>
    weather_data
    




}



pub fn insert_station_data(opts: Opts, station_data: Vec<StationData>) {

    // Create new pool connection 
    let pool = Pool::new(opts).expect("Pool failed to get opts in fn insert_station_data");

    let insert_stmt = r"INSERT INTO station_data (id, lat, lon, name, road_number, county_number) 
                                    VALUES (:id, :latitude, :longitude, :name, :road_number, :county_number)
                                    ON DUPLICATE KEY UPDATE lat=:latitude, lon=:longitude, name=:name, road_number=:road_number,
                                    county_number=:county_number;";

    for mut stmt in pool.prepare(insert_stmt).into_iter() { 
        
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

pub fn insert_weather_data(opts: Opts, weather_data: Vec<WeatherData>) {
    let insert_stmt = "INSERT INTO weather_data 
                        (station_id, timestamp, air_temperature, road_temperature, air_humidity, wind_speed, wind_direction) 
                        VALUES (:id, :timestamp, :air_temperature, :road_temperature, :air_humidity, :wind_speed, :wind_direction)";

}
    
// Setup connection to mysql
pub fn get_opts(user: &str, pass: &str, addr: &str, database: &str) -> Opts {
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

pub fn create_mysql_tables(opts: Opts) {

    let pool = Pool::new(opts).expect("Pool failed to get opts in fn create_mysql_tables");

    pool.prep_exec(r"CREATE TABLE `station_data` (
                        `id` char(20) NOT NULL,
                        `lat` float DEFAULT NULL,
                        `lon` float DEFAULT NULL,
                        `name` varchar(30) DEFAULT NULL,
                        `road_number` int(10) DEFAULT NULL,
                        `county_number` int(10) DEFAULT NULL,
                        PRIMARY KEY (`id`)
                    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=COMPACT;", ()).expect("Failed to create table: station_data");
    pool.prep_exec(r"CREATE TABLE `weather_data` (
                    `id` int(11) NOT NULL AUTO_INCREMENT,
                    `station_id` char(20) DEFAULT NULL,
                    `timestamp` timestamp NULL DEFAULT NULL,
                    `road_temperature` float DEFAULT NULL,
                    `air_temperature` float DEFAULT NULL,
                    `air_humidity` float DEFAULT NULL,
                    `wind_speed` float DEFAULT NULL,
                    `wind_direction` varchar(10) DEFAULT NULL,
                    PRIMARY KEY (`id`),
                    KEY `station_id` (`station_id`),
                    CONSTRAINT `weather_data_ibfk_1` FOREIGN KEY (`station_id`) REFERENCES `station_data` (`id`)
                    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=COMPACT;", ()).expect("Failed to create table: weather_Data");
}

