struct Laptop {
    brand: String,
    cost: u32,
    inventory: u32,
}


impl Laptop {
    
    fn calculate_purchase_cost(&self, units_purchased: u32) -> u64 {
        (self.cost as u64) * (units_purchased as u64)
    }
}

fn main() {

    const UNITS_PER_BRAND: u32 = 3;
    let mut grand_total_cost: u64 = 0;

    
    let hp = Laptop {
        brand: String::from("HP"),
        cost: 650_000,
        inventory: 10,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        cost: 755_000,
        inventory: 6,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        cost: 550_000,
        inventory: 10,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        cost: 850_000,
        inventory: 4,
    };

    let hp_cost = hp.calculate_purchase_cost(UNITS_PER_BRAND);
    println!("Cost for 3 {} laptops: N{}", hp.brand, hp_cost);
    grand_total_cost += hp_cost;

    let ibm_cost = ibm.calculate_purchase_cost(UNITS_PER_BRAND);
    println!("Cost for 3 {} laptops: N{}", ibm.brand, ibm_cost);
    grand_total_cost += ibm_cost;

    let toshiba_cost = toshiba.calculate_purchase_cost(UNITS_PER_BRAND);
    println!("Cost for 3 {} laptops: N{}", toshiba.brand, toshiba_cost);
    grand_total_cost += toshiba_cost;

    let dell_cost = dell.calculate_purchase_cost(UNITS_PER_BRAND);
    println!("Cost for 3 {} laptops: N{}", dell.brand, dell_cost);
    grand_total_cost += dell_cost;

    
    println!("\n---");
    println!("The Grand Total cost for purchasing 3 from each brand is: N{}", grand_total_cost);
    println!("---");
}
