extern crate reqwest;
extern crate quick_xml;
#[macro_use] // params! 
extern crate mysql;



use std::{thread, time::{Duration}};
use mysql::chrono::{Local};

mod auth;
mod fetch;
mod parse_xml;
mod database;


fn main() {

    let opts = database::get_opts(auth::USER_DB, auth::PASS_DB, auth::ADDR_DB, auth::NAME_DB);

    // Create new pool connections 
    let pool = mysql::Pool::new(opts).expect("Pool failed to get opts!");
    database::create_mysql_tables(pool.clone());
    let station_pool = pool.clone();
    let weather_pool = pool.clone();

    
    // Station data fetched once every day from DATEX II, parsed and inserted to MYSQL
    let station_thread = thread::spawn(move || loop {
         println!("station thread running...");
        let fetch_thread = thread::spawn(|| {
            fetch::fetch_xml(auth::URL_S, auth::USER_DATEX, auth::PASS_DATEX, "station_data_cache.xml");
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
            fetch::fetch_xml(auth::URL_W, auth::USER_DATEX, auth::PASS_DATEX, "weather_data_cache.xml");
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
    

    // database::create_mysql_tables(opts);


}