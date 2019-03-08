extern crate reqwest;
extern crate quick_xml;
#[macro_use] // params! 
extern crate mysql;

use std::{thread, time as stdtime};

mod fetch;
mod parse_xml;
mod database;



fn main() {

    // println!("RCM XML to MySQL started...");

    // Station data fetched once every day from DATEX II, parsed and inserted to MYSQL
    thread::spawn(|| loop {
        // Fetch file
        fetch::fetch_xml("https://datex.trafikverket.se/D2ClientPull/MetaDataBA/2_3/WeatherMetaData", "DatexLTU2018#", "LTU", "station_data_cache.xml");
        let opts = database::get_opts("mysql", "password", "127.0.0.1", "db");   
        // Parse XML file and insert to MYSQL
        let station_data = parse_xml::parse_station("station_data_cache.xml");
        database::insert_station_data(opts, station_data);
        
        // sleep 24 h
        thread::sleep(stdtime::Duration::from_secs(86400));

    });
    // Weather data fetched every 15 min from DATEX II, parsed and inserted to MYSQL
    thread::spawn(|| loop {
        // Fetch file
        fetch::fetch_xml("https://datex.trafikverket.se/D2ClientPull/WeatherPullServerBA/2_3/Weather", "DatexLTU2018#", "LTU", "weather_data_cache.xml");
        let opts = database::get_opts("mysql", "password", "127.0.0.1", "db");
        // Parse XML file and insert to MYSQL
        let weather_data = parse_xml::parse_weather("weather_data_cache.xml");
        database::insert_weather_data(opts, weather_data);

        // sleep 15 min
        thread::sleep(stdtime::Duration::from_secs(900));

    });
    // let opts = database::get_opts("mysql", "password", "127.0.0.1", "db");

    // database::create_mysql_tables(opts);

}