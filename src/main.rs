extern crate reqwest;
extern crate quick_xml;
#[macro_use] // params! 
extern crate mysql;



use std::{thread, time::{Duration}};
use mysql::chrono::{Local};

mod fetch;
mod parse_xml;
mod database;


fn main() {
    // let opts = Arc::new(RwLock::new(database::get_opts("mysql", "password", "127.0.0.1", "db")));
    let opts = database::get_opts("mysql", "password", "127.0.0.1", "db");

    // Create new pool connections 
    let pool = mysql::Pool::new(opts).expect("Pool failed to get opts!");
    let station_pool = pool.clone();
    let weather_pool = pool.clone();

    
    // Station data fetched once every day from DATEX II, parsed and inserted to MYSQL
    let station_thread = thread::spawn(move || loop {
         println!("station thread running...");
        let fetch_thread = thread::spawn(|| {
            fetch::fetch_xml("https://datex.trafikverket.se/D2ClientPull/MetaDataBA/2_3/WeatherMetaData", "LTU", "DatexLTU2018#", "station_data_cache.xml");
            println!("{:?}: station file fetched from DATEX II", Local::now().naive_local());
        });
        // Wait for fetch to complete
        fetch_thread.join().unwrap();
       

        let station_data = parse_xml::parse_station("station_data_cache.xml");
        database::insert_station_data(station_pool.clone(), station_data);
        
        // Sleep for 24 h
        thread::sleep(Duration::from_secs(86400));



    });
    // Weather data fetched every 15 min from DATEX II, parsed and inserted to MYSQL
    let _weather_thread = thread::spawn(move || loop {
        println!("weather thread running...");
        let fetch_thread = thread::spawn(|| {
            fetch::fetch_xml("https://datex.trafikverket.se/D2ClientPull/WeatherPullServerBA/2_3/Weather", "LTU", "DatexLTU2018#", "weather_data_cache.xml");
            println!("{:?}: weather file fetched from DATEX II", Local::now().naive_local());

        });
        // Wait for fetch to complete
        fetch_thread.join().unwrap();

        let weather_data = parse_xml::parse_weather("weather_data_cache.xml");
        database::insert_weather_data(weather_pool.clone(), weather_data);
    
        // Sleep for 15 min
        thread::sleep(Duration::from_secs(900));
    
    
    });
    station_thread.join().unwrap();

    

  

    
    println!("RCM XML to MySQL is running...");
    // weather_thread.join().unwrap();



    

    // database::create_mysql_tables(opts);

}