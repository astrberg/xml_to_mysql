extern crate reqwest;
extern crate quick_xml;
#[macro_use] // params! 
extern crate mysql;


mod fetch;
mod parse;

fn main() {
    // let user = "LTU";
    // let pass = "DatexLTU2018#";

    // let station_url = "https://datex.trafikverket.se/D2ClientPull/MetaDataBA/2_3/WeatherMetaData";
    // let weather_url = "https://datex.trafikverket.se/D2ClientPull/WeatherPullServerBA/2_3/Weather";

    // fetch::fetch_xml(station_url, user, pass, "station_data_cache.xml");
    // fetch::fetch_xml(weather_url, user, pass, "weather_data_cache.xml");


    // Create table station_data;
    // let opts = parse::get_opts("mysql", "password", "127.0.0.1", "db");
    // parse::create_mysql_tables(opts);

    // let stations = parse::parse_station("station_data_cache.xml");
    let weather = parse::parse_weather("weather_data_cache.xml");
    for i in weather.iter() {
        println!("{:?}", i);

    }
    // parse::insert_station_data(opts, stations);
    // fetch_xml();
    // create_mysql_tables();
}