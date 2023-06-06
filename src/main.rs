use std::collections::HashMap;
use csv::Reader;

//A lot of these could probably be done with a really clean map function, but I'm not experienced
//enough to make those from scratch. Also, I'm using match to check Options, but there are cleaner
//ways; this was just a time-saving decision.

//There's a LOT of string stuff here. I can describe differences between String (basically an object)
//and &str (reference to a fixed sized character sequence), but actually working with them in Rust
//is a bit of a pain (for a newbie like me) and I don't know how to do it most efficiently.

//Structure as the basis of objects in Rust.
//Lists the fields associated with the struct.
struct Cell {
    oem: String,
    model: String,
    launch_announced: Option<u32>,
    launch_status: Option<String>,
    body_dimensions: Option<String>,
    body_weight: Option<f32>,
    body_sim: Option<String>,
    display_type: Option<String>,
    display_size: Option<f32>,
    display_resolution: Option<String>,
    features_sensors: Option<String>,
    platform_os: Option<String>,
}

//Methods associated with a structure/object are declared separately. This allows for some really
//interesting behavior as an alternative to traditional inheritance.
impl Cell {
    //To_String method. This is for the assignment, the Rust equivalent to Java's toString would be
    //to implement the Display behavior (think of it as a stand-alone interface).
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("OEM: {}\n", self.oem).as_str());
        s.push_str(format!("Model: {}\n", self.model).as_str());
        match self.launch_announced {
            None => {}
            _ => s.push_str(format!("Launch Announced: {}\n", self.launch_announced.unwrap()).as_str())
        }
        match self.launch_status {
            None => {}
            _ => s.push_str(format!("Launch Status: {}\n", self.launch_status.clone().unwrap()).as_str())
        }
        match self.body_dimensions{
            None => {}
            _ => s.push_str(format!("Body Dimensions: {}\n", self.body_dimensions.clone().unwrap()).as_str())
        }
        match self.body_weight {
            None => {}
            _ => s.push_str(format!("Body Weight: {}\n", self.body_weight.unwrap()).as_str())
        }
        match self.body_sim {
            None => {}
            _ => s.push_str(format!("Body Sim: {}\n", self.body_sim.clone().unwrap()).as_str())
        }
        match self.display_type{
            None => {}
            _ => s.push_str(format!("Display Type: {}\n", self.display_type.clone().unwrap()).as_str())
        }
        match self.display_size {
            None => {}
            _ => s.push_str(format!("Display Size: {}\n", self.display_size.unwrap()).as_str())
        }
        match self.display_resolution{
            None => {}
            _ => s.push_str(format!("Resolution: {}\n", self.display_resolution.clone().unwrap()).as_str())
        }
        match self.features_sensors {
            None => {}
            _ => s.push_str(format!("Features/Sensors: {}\n", self.features_sensors.clone().unwrap()).as_str())
        }
        match self.platform_os {
            None => {}
            _ => s.push_str(format!("Platform OS: {}\n", self.platform_os.clone().unwrap()).as_str())
        }
        s
    }
}

//My parsing isn't right. I believe it does what I told it to, but there's a lot of inconsistencies
//in the provided file and I didn't make it general enough to find all of them.
fn parse_year(value: &str) -> Option<u32> {
    let mut s: String = value.to_string();
    if s.len() < 4 {
        return None;
    }
    if s.contains(",") {
        s = s.split_once(",").unwrap().0.parse().unwrap();
    }

    s.retain(|x| x.is_digit(10));
    match s.len() {
        4 => Some(s.parse::<u32>().unwrap()),
        _ => None
    }
}

//If there is a value given in grams, parses the number preceding it and returns it.
//Otherwise, returns None.
fn parse_weight(value: &str) -> Option<f32> {
    let index = value.find("g");
    match index {
        None => None,
        _ => Some(
            value[0..index.unwrap()]
                .trim()
                .parse::<f32>()
                .unwrap()),
    }
}

//If the string is only Discontinued or Cancelled, returns that. Otherwise, attempts to parse the
//year.
fn parse_announce_year(value: &str) -> Option<String> {
    match value {
        "Discontinued" | "Cancelled" => Some(value.to_string()),
        _ => match parse_year(value) {
            Some(x) => Some(x.to_string()),
            None => None
        }
    }
}

//If the value is "Yes" or "No", returns None. Otherwise, returns the string.
fn parse_sim_type(value: &str) -> Option<String> {
    match value {
        "Yes" | "No" => None,
        _ => Some(value.to_string())
    }
}

//Looks for the word "inches" to ensure the unit is correct, then returns the preceding value.
//Otherwise, returns None.
fn parse_display_size(value: &str) -> Option<f32> {
    let index = value.find("inches");
    match index {
        None => None,
        _ => Some(
            value.get(0..index.unwrap()).unwrap().trim()
                .parse::<f32>().unwrap()),
    }
}

//Returns any value that isn't just a number.
fn parse_sensors(value: &str) -> Option<String> {
    if value.chars().any(|x| x.is_alphabetic()) {
        Some(value.to_string())
    } else { None }
}

//Returns anything preceding a ",". This means it actually misses some values that don't include ","
fn parse_os(value: &str) -> Option<String> {
    let index = value.find(",");
    match index {
        None => None,
        _ => Some(
            value[0..index.unwrap()].to_string()),
    }
}

//Uses a pair of hash maps to find the OEM with the heaviest devices.
//It would probably be better to use a tuple for the value, but I really want to maintain the
//current "update or insert" because honestly I might use this as a reference later.
//The cleanest option is actually probably a struct with a method for updating both values.
fn find_heaviest_oem(phones: &Vec<Cell>) {
    let mut oems = HashMap::new();
    let mut oem_count = HashMap::new();
    for phone in phones {
        match phone.body_weight {
            None => continue,
            Some(x) => {
                oems.entry(phone.oem.to_string())
                    .and_modify(|weight| *weight += x).or_insert(x);
                oem_count.entry(phone.oem.to_string())
                    .and_modify(|count| *count += 1).or_insert(1);
            }
        }
    }
    let mut heaviest: f32 = 0.0;
    let mut s = String::new();
    for (key, val) in oems.iter() {
        let weight = val / *oem_count.get(key).unwrap() as f32;
        if weight > heaviest {
            heaviest = weight;
            s = key.to_string();
            println!("New heaviest: key: {key} val: {heaviest}");
        }
    }
}

//Finds phones that were announced in one year, and released in another.
//This is very limited by a combination of my poor parsing and the problems in the provided data.
//I imagine there's a cleaner way to ensure both values exist and then compare, but I don't know it.
fn find_year_mismatch(phones: &Vec<Cell>) {
    for phone in phones {
        match phone.launch_announced {
            None => {}
            Some(x) => {
                match phone.launch_status {
                    None => {}
                    Some(_) => {
                        match parse_year(phone.launch_status.as_ref().unwrap()) {
                            None => {}
                            Some(y) => {
                                if x != y {
                                    println!("{oem}: {model}: Announced in {x}, released in {y}", oem = phone.oem, model = phone.model)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

//Counts phones with only one listed feature.
fn find_mono_feature(phones: &Vec<Cell>) {
    let mut count = 0;
    for phone in phones {
        match &phone.features_sensors {
            None => {}
            Some(x) => {
                if !x.contains(",") {
                    count += 1;
                    // println!("{}", x)
                }
            }
        }
    }
    println!("{}", count);
}

//Finds the year with the most phones released. Because my parsing is off and the provided data
//lists the release year in the announced field for many phones, this is of questionable accuracy.
fn find_busiest_year(phones: &Vec<Cell>) {
    let mut years = HashMap::new();
    for phone in phones {
        match &phone.launch_status {
            None => {}
            Some(x) => {
                // println!("{}", x);
                match x.parse::<u32>()
                {
                    Ok(val) => {
                        years.entry(val).and_modify(|year| *year += 1).or_insert(1);
                    }
                    Err(_) => { continue; }
                }
            }
        }
    }
    let mut busiest: i32 = 0;
    let mut year: u32 = 0;
    for (key, value) in years.iter() {
        // println!("New busiest year: key: {key} val: {value}");
        if value > &busiest {
            busiest = *value;
            year = *key;
            println!("New busiest year: key: {key} val: {value}");
        }
    }
}

//Checks for "-" as a value.
fn dash_check(value: &str) -> Option<String> {
    match value {
        "-" => None,
        x => {Some(x.to_string())} }
}

fn make_phone_vec<R>(mut reader: Reader<R>) -> Vec<Cell> where R: std::io::Read{
    let mut vec = Vec::new();
    for result in reader.records() {
        let record = result.unwrap();

        let phone = Cell {
            oem: record.get(0).unwrap().to_string(),
            model: record.get(1).unwrap().to_string(),
            launch_announced: parse_year(record.get(2).unwrap()),
            launch_status: parse_announce_year(record.get(3).unwrap()),
            body_dimensions: dash_check(record.get(4).unwrap()),
            body_weight: parse_weight(record.get(5).unwrap()),
            body_sim: parse_sim_type(record.get(6).unwrap()),
            display_type: dash_check(record.get(7).unwrap()),
            display_size: parse_display_size(record.get(8).unwrap()),
            display_resolution: dash_check(record.get(9).unwrap()),
            features_sensors: parse_sensors(record.get(10).unwrap()),
            platform_os: parse_os(record.get(11).unwrap()),
        };
        vec.push(phone);
    }
    vec
}

//Main method, similar to Java. I used a library ("crate" in Rust) to parse the CSV file because
//it's finicky. If I didn't need to parse the values (or was more comfortable with unchecked data,
//and instead parsed on every read) there's actually a nifty way to directly create a struct from
//each CSV line, rather than having that huge block of code.
fn main() {
    let mut reader = csv::Reader::from_path("resources/cells.csv").unwrap();
    let mut vec = make_phone_vec(reader);

    // find_heaviest_oem(&vec);
    // find_year_mismatch(&vec);
    // find_mono_feature(&vec);
    find_busiest_year(&vec);
}

//I don't actually remember how to run these in Intellij, but you can run them in the command line
//by running
//$ cargo test
//I'm only testing the first 5 or so phones in the tests that actually parse.
#[cfg(test)]
mod tests {
    use crate::make_phone_vec;

    //I really REALLY avoid doing this, but I took this approach from stack overflow.
    fn string_assert<T>(_: &T) {
        assert_eq!(std::any::type_name::<T>(), std::any::type_name::<String>())
    }

    //Properly this is an unsigned integer.
    fn int_assert<T>(_: &T) {
        assert_eq!(std::any::type_name::<T>(), std::any::type_name::<u32>())
    }

    fn float_assert<T>(_: &T) {
        assert_eq!(std::any::type_name::<T>(), std::any::type_name::<f32>())
    }

    //The Garmin-Asus nuvifone M10 has a body dimension value of "-", so this ensure it was removed.
    #[test]
    fn empty_check(){
        let mut reader = csv::Reader::from_path("resources/basic.csv").unwrap();
        let mut vec = make_phone_vec(reader);
        for phone in vec{
            let invalid = String::from("-");
            if phone.body_dimensions.is_some(){
                assert_ne!(phone.body_dimensions.unwrap(), invalid);
            }
            if phone.display_type.is_some(){
                assert_ne!(phone.display_type.unwrap(), invalid);
            }
            if phone.display_resolution.is_some(){
                assert_ne!(phone.display_resolution.unwrap(), invalid);
            }
        }
    }

    //The unwrap function being called is functionally an implicit file check, since it will fail
    //if there's an error (it's roughly an "assert this worked") (and is why some people vehemently
    //oppose using unwrap in production code). There's also an argument that, in Rust, it's better
    //to just try and do something and deal with the result than see if you can do it first, but I'm
    //not experienced enough to have an opinion on that.
    #[test]
    fn file_check(){
        assert!(std::path::Path::new("resources/cells.csv").exists());
        let mut reader = csv::Reader::from_path("resources/cells.csv").unwrap();
        let mut vec = make_phone_vec(reader);
        assert!(vec.len() > 0);
    }

    //Rust is so strict with types it make Java look drunk; I don't know that I ever would need to
    //check the type of something like this. I don't think it's actually possible for this to fail.*
    #[test]
    fn type_check(){
        let mut reader = csv::Reader::from_path("resources/basic.csv").unwrap();
        let mut vec = make_phone_vec(reader);
        for phone in vec{
            string_assert(&phone.oem);
            string_assert(&phone.model);
            match &phone.launch_announced {
                None => {continue}
                Some(x) => {int_assert(x)}
            }
            match &phone.launch_status {
                None => {continue}
                Some(x) => {string_assert(x)}
            }
            match &phone.body_dimensions {
                None => {continue}
                Some(x) => {string_assert(x)}
            }
            match &phone.body_weight {
                None => {continue}
                Some(x) => {float_assert(x)}
            }
            match &phone.body_sim {
                None => {continue}
                Some(x) => {string_assert(x)}
            }
            match &phone.display_type {
                None => {continue}
                Some(x) => {string_assert(x)}
            }
            match &phone.display_size {
                None => {continue}
                Some(x) => {float_assert(x)}
            }
            match &phone.display_resolution {
                None => {continue}
                Some(x) => {string_assert(x)}
            }
            match &phone.features_sensors {
                None => {continue}
                Some(x) => {string_assert(x)}
            }
            match &phone.platform_os {
                None => {continue}
                Some(x) => {string_assert(x)}
            }
        }
    }
}