// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// struct BackstageTicket{
    // event: String,
    // name: String,
    // price: i32,
// }
// struct VipTicket{
    // event: String,
    // name: String,
    // price: i32,
// }
struct StandardTicket{
    event: String,
    name: String,
    price: i32,
}

// * Use an enum for the tickets with data associated 
// with each variant
enum Ticket{
    Backstage(StandardTicket), // Backstage(f64, String)
    Vip(StandardTicket), // Vip(f64, String)
    Standard(StandardTicket), // Standard(f64)
}
fn main() {
    // * Create one of each ticket and place into a vector
    let mut ticket_vector = Vec::new();
    // let bsticket = Ticket::Backstage(52.6, "new year party")
    let bsticket = Ticket::Backstage(StandardTicket{
        event:"New Year show".to_owned(),
        name: "Prince Charles".to_owned(),
        price: 250,
    });
    // let vipticket = Ticket::Vip(12.6, "Charity event")
    let vipticket = Ticket::Vip(StandardTicket{
        event:"Farewell Magic show".to_owned(),
        name: "Nigara Nautanki".to_owned(),
        price: 20,
    });
    // let stdticket = Ticket::Standard(10.6, "Fair")
    let stdticket = Ticket::Standard(StandardTicket{
        event:"Science Exhibition".to_owned(),
        name: "".to_string(),
        price: 200,
    });
    ticket_vector.push(bsticket);
    ticket_vector.push(vipticket);
    ticket_vector.push(stdticket);
// * Use a match expression while iterating the vector to print the ticket info
    for tkt in ticket_vector{
        match tkt {
            // this is where it gets interesting, the data is 
            // named inside the match, and then referred 
            // in println!
            // Ticket::Backstage(price, holder) => {
            Ticket::Backstage(bsticket) => println!("event is: {} \n name is: {} \n price is: {}",
                                                                        bsticket.event,
                                                                        bsticket.name,
                                                                        bsticket.price),
            Ticket::Standard(stdticket) => println!("Event is: {} \n Price is: {}",
                                                                        stdticket.event,
                                                                        stdticket.price),
            Ticket::Vip(vipticket) => println!("Event is: {} and \n name is: {} and \n price is: {}",
                                                                    vipticket.event,
                                                                    vipticket.name,
                                                                    vipticket.price),
        }
    }

}
