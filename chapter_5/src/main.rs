fn main() {
    //! impl -> merupakan turunan dari struct
    //! struct -> sebuah tipe data kalo di TS, kaya type / interface
    //! method merupakan function dari struct yang bisa diakses langsung
    
    let toyota_avanza = Car {
        brand: String::from("toyota"),
        color: String::from("red")
    };

    let brand = toyota_avanza.get_brand();
    println!("{brand}");

    print!("{}", toyota_avanza.get_car_info());
}

struct Car {
    brand: String,
    color: String
}

impl Car {
    fn get_car_info(&self) -> String {
        let mut car_info = String::new();

        car_info.push_str(self.brand.as_str());
        car_info.push_str(self.color.as_str());

        return car_info
    }

    fn get_brand(&self) -> String {
        self.brand.clone()
    }
}