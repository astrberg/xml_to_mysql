use std::io;
use std::fs::File;

// Get the XML file from datex using basic auth
pub fn fetch_xml(url: &str, user: &str, pass: &str, file_name: &str) {
   
    let client = reqwest::Client::new();

    let mut response = client.get(url)
        .basic_auth(user, Some(pass))
        .send()
        .expect("Connection failed to Datex");
    assert!(response.status().is_success());


    let mut file = File::create(file_name)
        .expect("Error creating file, station_data");
    io::copy(&mut response, &mut file)
        .expect("Failed to read response to file");

}

