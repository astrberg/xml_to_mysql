use mysql::{Pool, Opts};
use mysql::OptsBuilder;
use mysql::chrono::{DateTime, FixedOffset};

use crate::parse_xml::{StationData, WeatherData};


// Insert the data to MYSQL, TABLE assumed to exist
pub fn insert_station_data(pool: Pool, station_data: Vec<StationData>) {



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
            }).expect("Failed to execute statement when reading from station_data");
        }
    }
}
// Insert the data to MYSQL, TABLE assumed to exist
pub fn insert_weather_data(pool: Pool, weather_data: Vec<WeatherData>) {

    let insert_stmt = r"INSERT IGNORE INTO weather_data 
                        (station_id, timestamp, air_temperature, road_temperature, air_humidity, wind_speed, wind_direction) 
                        VALUES (:station_id, NULLIF(:timestamp,''), NULLIF(:air_temperature,''), NULLIF(:road_temperature, ''),
                        NULLIF(:air_humidity, ''), NULLIF(:wind_speed, ''), NULLIF(:wind_direction, ''));";
    
    for mut stmt in pool.prepare(insert_stmt).into_iter() { 
        
        for i in weather_data.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            stmt.execute(params!{
                "station_id" => i.station_id.clone(),
                "timestamp" => DateTime::<FixedOffset>::parse_from_rfc3339(&i.timestamp.clone()).unwrap().naive_utc(),
                "air_temperature" => i.air_temperature.clone(),
                "road_temperature" => i.road_temperature.clone(),
                "air_humidity" => i.air_humidity.clone(),
                "wind_speed" => i.wind_speed.clone(),
                "wind_direction" => i.wind_direction.clone(),

            }).expect("Failed to execute statement when reading from weather_data");
        }
    }

}
    
// Setup connection to mysql, IMPORTANT! mysql port default is 3306 (consider to change for security)
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

// Create the tables, only for new db!
pub fn create_mysql_tables(pool: Pool) {


    pool.prep_exec(r"CREATE TABLE IF NOT EXISTS station_data (
                        id CHAR(20) NOT NULL,
                        lat FLOAT DEFAULT NULL,
                        lon FLOAT DEFAULT NULL,
                        name VARCHAR(30) DEFAULT NULL,
                        road_number INT(10) DEFAULT NULL,
                        county_number INT(10) DEFAULT NULL,
                        PRIMARY KEY (id)
                    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=COMPACT;", ()).expect("Failed to create table: station_data");
    pool.prep_exec(r"CREATE TABLE IF NOT EXISTS weather_data (
                    id INT NOT NULL AUTO_INCREMENT,
                    station_id CHAR(20) NOT NULL,
                    timestamp TIMESTAMP NULL DEFAULT NULL,
                    road_temperature FLOAT DEFAULT NULL,
                    air_temperature FLOAT DEFAULT NULL,
                    air_humidity FLOAT DEFAULT NULL,
                    wind_speed FLOAT DEFAULT NULL,
                    wind_direction CHAR(10) DEFAULT NULL,
                    PRIMARY KEY (id),
                    KEY station_id (station_id),
                    FOREIGN KEY (station_id) REFERENCES station_data (id)
                    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=COMPACT;", ()).expect("Failed to create table: weather_data");
}

