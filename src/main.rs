extern crate reqwest;
extern crate quick_xml;
#[macro_use] // params! 
extern crate mysql;


mod fetch;
mod parseXML;
mod database;



fn main() {
    // let user = "LTU";
    // let pass = "DatexLTU2018#";

    // let station_url = "https://datex.trafikverket.se/D2ClientPull/MetaDataBA/2_3/WeatherMetaData";
    // let weather_url = "https://datex.trafikverket.se/D2ClientPull/WeatherPullServerBA/2_3/Weather";

    // fetch::fetch_xml(station_url, user, pass, "station_data_cache.xml");
    // fetch::fetch_xml(weather_url, user, pass, "weather_data_cache.xml");


    // Create table station_data;
    let opts = database::get_opts("mysql", "password", "127.0.0.1", "db");
    // database::create_mysql_tables(opts);
// 
    // let stations = parseXML::parse_station("station_data_cache.xml");
    // let mut count = 0;
    let weather = parseXML::parse_weather("weather_data_cache.xml");
    // for i in weather.iter() {
               
    //     println!("{:?}", i.road_temperature);


    // }
    // println!("{}", count);
    
    // database::insert_station_data(opts, stations);
    // let opts = database::get_opts("mysql", "password", "127.0.0.1", "db");

    database::insert_weather_data(opts, weather);

    // fetch_xml();
    // database::create_mysql_tables(opts);
}