#[allow(dead_code)]
#[derive(Debug)]
pub struct AverageCollection {
    list_item: Vec<f64>, // there is opp to use generics
    average: f64
}

impl AverageCollection {
    pub fn add(&mut self, elem: f64) {
        self.list_item.push(elem);
        self.update_avg();
    }

    pub fn remove(&mut self) -> Option<f64> {
        let elem = self.list_item.pop();
        match elem {
            Some(value) => {
                self.update_avg();
                Some(value)
            },
            None => None
        }
    }

    pub fn update_avg(&mut self) {
        let mut sum = 0.0;
        for elem in &self.list_item{
            sum += elem;
        }
        // learnt about the 
        let vec_len = self.list_item.len() as i32;
        self.average = sum / vec_len as f64; 
    }
}

fn main() {
    let mut coll_list = AverageCollection{
        list_item: vec![5.0, 6.0, 2.0, 3.0, 1.0, 9.0, 7.0, 5.0],
        average:0.0
    };

    println!("Before update: {:?}", coll_list);
    coll_list.update_avg();
    println!("After update: {:?}", coll_list);
    coll_list.remove(); 
    println!("After remove: {:?}", coll_list);
    
}
