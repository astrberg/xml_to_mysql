/*
Parse given file, tags are static and need to modified if new unknown XML file is given.
If DATEX II XML file structure is changed parsing will most likely stop working.

TODO: weather_data XML file has precipitationType ex. snow/rain and precipitationIntensity that are not parsed
because of the original MYSQL table structure!

*/

use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Debug)]
pub struct StationData {
    pub id: String,
    pub name:  String,
    pub road_number: String,
    pub county_number: String,
    pub latitude: String,
    pub longitude: String,
    _secret: (), // Disliked the use of pub, will prevent from use of struct elsewere then in this module
}
#[derive(Debug)]
pub struct WeatherData {
    pub station_id: String,
    pub timestamp: String,
    pub road_temperature: String,
    pub air_temperature: String,
    pub air_humidity: String,
    pub wind_speed: String,
    pub wind_direction: String,
    _secret: (),

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
                            _secret: (),

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
    // Used for nested tags
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
                        _secret: (),

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

