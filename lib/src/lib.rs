extern crate hidapi;


pub fn hello_from_lib() {
    let api = hidapi::HidApi::new().unwrap();
// Print out information about all connected devices
    for device in &api.devices() {
        println!("{:#?}", device);

        let result = api.open_path(&device.path);
        if result.is_err() {
            println!("{:#?}", result.err());
        } else {
            let device = result.unwrap();

            println!("{:#?}", device.check_error());

            println!("{:#?}", device.get_manufacturer_string());
            println!("{:#?}", device.check_error());

            println!("{:#?}", device.get_product_string());
            println!("{:#?}", device.check_error());
        }
    }
}

