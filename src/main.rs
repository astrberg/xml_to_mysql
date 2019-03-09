extern crate reqwest;
extern crate quick_xml;
#[macro_use] // params! 
extern crate mysql;



use std::{thread, time::Duration};
use std::sync::{Mutex, Arc, MutexGuard};
use std::ops::Deref;

mod fetch;
mod parse_xml;
mod database;


fn main() {
    // let opts = Arc::new(RwLock::new(database::get_opts("mysql", "password", "127.0.0.1", "db")));
    let opts = database::get_opts("mysql", "password", "127.0.0.1", "db");

    // Create new pool connection 
    let pool = mysql::Pool::new(opts).expect("Pool failed to get opts!");
    // Mutex locks
    let mysql_pool = Arc::new(Mutex::new(pool));
    let station_pool = Arc::clone(&mysql_pool);
    let weather_pool = Arc::clone(&mysql_pool);
    
    // Station data fetched once every day from DATEX II, parsed and inserted to MYSQL
    let station_thread = thread::spawn(move || loop {

            let mut _mysql_pool = station_pool.lock().unwrap();
            println!("station thread running...");
            fetch::fetch_xml("https://datex.trafikverket.se/D2ClientPull/MetaDataBA/2_3/WeatherMetaData", "LTU", "DatexLTU2018#", "station_data_cache.xml");
            let station_data = parse_xml::parse_station("station_data_cache.xml");
            database::insert_station_data(_mysql_pool.clone(), station_data);
            
            // Release mutex
            drop(_mysql_pool);
            // Sleep for 24 h
            thread::sleep(Duration::from_secs(86400));
     });
    let weather_thread = thread::spawn(move || loop {

         let mut _mysql_pool = weather_pool.lock().unwrap();
        // Weather data fetched every 15 min from DATEX II, parsed and inserted to MYSQL
        println!("weather thread running...");
        fetch::fetch_xml("https://datex.trafikverket.se/D2ClientPull/WeatherPullServerBA/2_3/Weather", "LTU", "DatexLTU2018#", "weather_data_cache.xml");
        let weather_data = parse_xml::parse_weather("weather_data_cache.xml");
        database::insert_weather_data(_mysql_pool.clone(), weather_data);
        
        // Release mutex
        drop(_mysql_pool);
        // Sleep for 15 min
        thread::sleep(Duration::from_secs(900));
    
    
    });
 
    weather_thread.join().unwrap();
    station_thread.join().unwrap();


    println!("RCM XML to MySQL is running...");
    

    // database::create_mysql_tables(opts);

}