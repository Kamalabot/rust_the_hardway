struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freeze() -> Self {
        Self {
            degrees_f: 32.0,
        }
    }
    fn boiling() -> Self {
        Self {
            degrees_f: 212.0,
        }
    }
    fn show_temp(&self){
        println!("{:?}", self.degrees_f);
    }
}

fn main(){
    let hot = Temperature{
        degrees_f: 56.7
    };
    hot.show_temp();
    let colder = Temperature::freeze();
    colder.show_temp();
    let boilr = Temperature::boiling();
    boilr.show_temp();

}