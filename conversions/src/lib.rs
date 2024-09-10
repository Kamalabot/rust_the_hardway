use ::std::fmt;
use ::std::num::ParseFloatError;
use ::std::num::ParseIntError;
use ::std::str::FromStr;

pub fn basic_as_conv() {
    let int_val = 10;
    let float_val = int_val as f64;
    println!("Int val {} is float now: {}", int_val, float_val);
}

pub fn from_conv() {
    let int_val: i32 = 5;
    let fl_val: f64 = int_val.into();
    println!("Int val {} is float now:{}", int_val, fl_val);
}
#[derive(Debug)]
struct Celsius(f64);

#[derive(Debug)]
struct Farenheit(f64);

impl From<Farenheit> for Celsius {
    fn from(f: Farenheit) -> Celsius {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

pub fn custom_from() {
    let f = Farenheit(100.0);
    let c: Celsius = f.into();
    println!("Celsius {:?}", c);
}
// try_from is used for conversion
impl TryFrom<&str> for Celsius {
    type Error = ParseFloatError;
    // need to use ParseFloatError to work

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let temp = s.parse::<f64>()?;
        Ok(Celsius(temp))
    }
}

pub fn try_from_conversion() {
    match Celsius::try_from("27.5") {
        Ok(c) => println!("Printing celsius: {:?}", c),
        Err(_) => println!("Conversion did not work"),
    }
}

pub fn from_str_conv() {
    let int_res = i32::from_str("32");
    match int_res {
        Ok(nl) => println!("int is {}", nl),
        Err(e) => println!("Error is {:?}", e),
    }
}

pub fn to_string() {
    let num = 42;
    let str_val = num.to_string();
    println!("Coverted num is {}", str_val);
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
pub fn display_conversion() {
    let point = Point { x: 10, y: 10 };
    // since display trait is implemented
    println!("Point display: {}", point)
    // the {} are kept empty
}

// Custom Error handling in conversion
#[derive(Debug)]
struct ParseCoordinateError;
// a new type is created

impl fmt::Display for ParseCoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in parsing coords")
    }
}
// Display fmt function is implemented

impl From<ParseIntError> for ParseCoordinateError {
    fn from(value: ParseIntError) -> Self {
        ParseCoordinateError
    }
}
// From trait is implemented for ParseCoordinateError
// to convert ParseIntError to ParseCoordinateError.
// Implementing From trait enables usage of fn from()
// After this implementation, you can use
// ParseCoordinateError::from(some_parse_int_error) or just some_parse_int_error.into()

struct Coordinate {
    x: i32,
    y: i32,
}

// in order to use the above error, we need to
// have a conversion process

impl TryFrom<&str> for Coordinate {
    type Error = ParseCoordinateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(",").collect();
        println!("vector parts are: {:?}", parts);
        if parts.len() != 2 {
            return Err(ParseCoordinateError);
        }
        // let x: i32 = parts[0].parse()?;
        let x: i32 = parts[0].parse::<i32>()?;
        // let y: i32 = parts[1].parse()?;
        // the above threw an error
        let y: i32 = parts[0].parse::<i32>()?;
        Ok(Coordinate { x, y })
    }
}

pub fn custom_try_from_error() {
    match Coordinate::try_from("10, 20") {
        Ok(coord) => println!("Parsed Coordinate: {} {}", coord.x, coord.y),
        Err(e) => println!("Error: {}", e),
        // the display of the error is implemented
    }
}

pub fn opt_to_res() {
    // let s_vl: Option<i32> = Some(10);
    let s_vl: Option<i32> = None;
    let rs: Result<i32, &str> = s_vl.ok_or("val not found");
    println!("The result is :{:?}", rs);
}
// ok_or option to res
pub fn res_to_opt() {
    let re: Result<i32, &str> = Ok(10);
    let opt = re.ok();
    println!("Option is {:?}", opt);
}
// ok for res to option

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl FromStr for Person {
    type Err = &'static str;
    // static is longest lifetime possible
    // is immuatable ref stored in binary
    // used here for simple error types as
    // it requires no heap allocation, or ownership mgmt
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();
        if parts.len() != 2 {
            return Err("Invalid Format");
        }
        let name = parts[0].to_string();
        let age = parts[1].parse::<u32>().map_err(|_| "invalid age")?;
        Ok(Person { name, age })
    }
}

fn cus_fromstr_type() {
    let pe: Person = "Alice, 30".parse().unwrap();
    println!("Parsed Person: {:?}", pe);
}

pub fn auto_conversion() -> Result<(), Box<dyn std::error::Error>> {
    // Simple parse
    // let res: Result<i32, _> = "32".parse::<i32>().try_into();
    let res: Result<i32, ParseIntError> = "32".parse::<i32>();
    match res {
        Ok(val) => println!("Parsed {}", val),
        Err(_) => println!("parse error"),
    }
    // Try_into where the result could fail
    let big_num: i64 = 500;
    let small: Result<i8, _> = big_num.try_into();
    match small {
        Ok(val) => println!("If converted: {}", val),
        Err(e) => println!("Errored out"),
    }
    // ? usage
    let val: i32 = "42".parse()?;
    // the above line will return error
    println!("Its parsed ? : {}", val);

    //slice to string
    let s: &str = "hello";
    let string_val: String = s.into();
    println!("String slice: {}", string_val);

    let v = vec![1, 2, 3];
    let sl: &[i32] = &v;
    println!("Vec to slice {:?}", sl);
    let arr = [5, 2, 3, 7, 8];

    let sl2 = &arr[1..3];
    println!("Sliced with .. {:?}", sl2);

    let tp1 = (1, 2);
    let pttupl: Point = tp1.into();
    // for the above to work, additional From
    // has to be implemented
    println!("Into Point done: {}", pttupl);

    let result: Result<Celsius, _> = "100.0".try_into();
    // look the type used, its Celsius
    // Celsius(i64) is struct at line 6 in this file

    match result {
        Ok(celsius) => println!("Converted to Celsius: {}", celsius.0),
        Err(e) => println!("Failed to convert: {:?}", e),
    }
    Ok(())
}
