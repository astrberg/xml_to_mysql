extern crate reqwest;
extern crate quick_xml;
#[macro_use] // params! 
extern crate mysql;


mod station;

fn main() {
    let url = "https://datex.trafikverket.se/D2ClientPull/MetaDataBA/2_3/WeatherMetaData";
    let user = "LTU";
    let pass = "DatexLTU2018#";

    station::fetch_xml(url, user, pass);

    // Create table station_data;
    let opts = station::get_opts("mysql", "password", "127.0.0.1", "db");
    // create_mysql_tables(opts);

    let stations = station::read_file("station_data.xml");
    // for i in stations.iter() {
    //     println!("{:?}", i);

    // }
    // insert_station_data(opts, stations);
    // fetch_xml();
    // create_mysql_tables();
}