#![allow(dead_code)]

fn main(){
    /* Can test the code here */
}
fn tax_calculator(state: String, amount: f64) -> f64 { 
    let state = state.to_uppercase();
    if state == "WI"{
        let subtot = amount * 0.055;
        subtot + amount
    } else {
        amount
    }
}

fn tax_calculator_noelse(state: String, amount: f64) -> f64 { 
    let state = state.to_uppercase();
    if state == "WI"{
        let subtot = amount * 0.055;
        return subtot + amount;
    }
    amount
}

#[cfg(test)]
mod tests{
    use crate::*;
    #[test]
    fn wi_tax(){
        assert_eq!(10.55, tax_calculator("WI".to_owned(), 10.00));
    }
    #[test]
    fn wi_check_lower_case(){
        assert_eq!(10.55, tax_calculator("wi".to_owned(), 10.00));
    }
    #[test]
    fn wi_check_mix_case(){
        assert_eq!(10.55, tax_calculator("wI".to_owned(), 10.00));
    }
    #[test]
    fn mn_tax(){
        assert_eq!(10.00, tax_calculator("MN".to_owned(), 10.00));
    }
    #[test]
    fn wi_tax_noelse(){
        assert_eq!(10.55, tax_calculator_noelse("WI".to_owned(), 10.00));
    }
    #[test]
    fn wi_check_lower_case_ne(){
        assert_eq!(10.55, tax_calculator_noelse("wi".to_owned(), 10.00));
    }
    #[test]
    fn wi_check_mix_case_ne(){
        assert_eq!(10.55, tax_calculator_noelse("wI".to_owned(), 10.00));
    }
    #[test]
    fn mn_tax_ne(){
        assert_eq!(10.00, tax_calculator_noelse("MN".to_owned(), 10.00));
    }
}