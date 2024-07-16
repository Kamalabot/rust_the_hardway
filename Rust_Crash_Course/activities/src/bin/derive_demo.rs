#[allow(dead_code)]

#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker
}
impl Position {
    fn print(&self){
        match &self {
            Position::Manager => println!("{}", "Manager"),
            Position::Supervisor => println!("{}", "Supervisor"),
            Position::Worker => println!("{}", "Worker"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Worker {
    position: Position,
    work_hrs: i64,
}

// without derive
fn print_worker_nodrive(wkr: &Worker){
    println!("work hours is {}", wkr.work_hrs);
    wkr.position.print();
}
// with derive
fn print_worker(wkr: Worker){
    println!("work hours is {}", wkr.work_hrs);
    wkr.position.print();
}
fn main(){
    let wkr1 = Worker{
        position: Position::Manager,
        work_hrs: 65,
    };
    print_worker(wkr1); // with derive
    print_worker_nodrive(&wkr1);
    print_worker_nodrive(&wkr1);
}