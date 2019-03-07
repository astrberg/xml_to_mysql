extern crate reqwest;
extern crate quick_xml;
#[macro_use] // params! 
extern crate mysql;

use std::{thread, time as stdtime};

mod fetch;
mod parse_xml;
mod database;



fn main() {

    println!("RCM XML to MySQL started...");


    let user = "LTU";
    let pass = "DatexLTU2018#";
    let station_url = "https://datex.trafikverket.se/D2ClientPull/MetaDataBA/2_3/WeatherMetaData";
    let weather_url = "https://datex.trafikverket.se/D2ClientPull/WeatherPullServerBA/2_3/Weather";   
    let opts = database::get_opts("mysql", "password", "127.0.0.1", "db");


    // Station data fetched once every day from DATEX II, parsed and inserted to MYSQL
    let station_thread = thread::spawn(move || loop {
        // Fetch file
        fetch::fetch_xml(&station_url, user, pass, "station_data_cache.xml");
       
        // Parse XML file and insert to MYSQL
        let station_data = parse_xml::parse_station("station_data_cache.xml");
        database::insert_station_data(opts.clone(), station_data);
        
        // sleep 24 h
        thread::sleep(stdtime::Duration::from_secs(86400));

    });
    // Weather data fetched every 15 min from DATEX II, parsed and inserted to MYSQL
    let weather_thread = thread::spawn(move || loop {
        // Fetch file
        fetch::fetch_xml(weather_url, user, pass, "weather_data_cache.xml");

        // Parse XML file and insert to MYSQL
        let weather_data = parse_xml::parse_weather("weather_data_cache.xml");
        database::insert_weather_data(opts.clone(), weather_data);

        // sleep 15 min
        thread::sleep(stdtime::Duration::from_secs(900));

    });


    station_thread.join().unwrap();
    weather_thread.join().unwrap();

}