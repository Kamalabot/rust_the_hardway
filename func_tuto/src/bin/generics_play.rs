use std::fmt::Debug;

#[derive(Debug)]
struct Container<E>{
    value: E,
}

// will first do for single val struct, 
// then do for double value one

impl<E> Container<E>{
    fn new(value: E) -> Self {
        Container { value }
    }

    fn get_val(&self) -> &E {
        &self.value
    }

}

#[derive(Debug)]
struct MapCtr<E, V>{
    value: E,
    key: V,
}

impl<E: Debug, V: Debug> MapCtr<E, V> {
    fn new(key:V, value:E) -> Self {
        MapCtr { value, key }
    }

    fn print_kv(&self) {
        println!("{:?}: {:?}",&self.key, &self.value);
    }
}

// the types that implement the Debug trait
// can be processed
fn process_ref<E: std::fmt::Debug>(ctr: &Container<E>){
    println!("Processing container by ref: {:?}", ctr.get_val());
}

fn process_val<E: std::fmt::Debug>(ctr: Container<E>){
    println!("Processing container by ref: {:?}", ctr.value);
}

fn main(){
    let int_ctr = Container::new(5);

    let str_ctr = Container::new(String::from("Hey there..."));

    process_val(int_ctr);

    // how to capture the error that occurs by 
    // calling a macro

    // println!("{:?}", int_ctr);
    // rustc: borrow of moved value: `int_ctr`
    // value borrowed here after move [E0382
    //
    process_ref(&str_ctr);

    println!("{:?}", str_ctr);

    println!("Lets see how well I have understood... Traits & Generics");

    let kv = MapCtr::new(55, "there".to_string()); 

    kv.print_kv();

}












