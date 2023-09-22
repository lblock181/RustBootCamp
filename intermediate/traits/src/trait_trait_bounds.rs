
trait Vehicle: Paint {
    fn park(&self);

    fn get_default_color() -> String {
        "black".to_owned()
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting {}", color);
    }
}


struct VehicleInfo { 
    make: String,
    model: String,
    year: u16
}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house {}",color);
    }
}

struct Car {
    info: VehicleInfo
}

impl Vehicle for Car {
    fn park(&self) {
        println!("parking car");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo
}

impl Truck {
    fn unload(&self) {
        println!("Unloading truck")
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("{} truck parking", self.info.make);
    }
}

impl Paint for Truck {}

// setting up polymorphism using Rust Traits instead of inheritance

trait Park {
    fn park(&self); // declares method that needs to be defined on impl
}

// this has default implementation which means no need to define when declaring trait
// allows for use with other impl blocks like house struct

impl Park for Car {
    fn park(&self) {
        println!("{} car parking", self.info.make);
    }
}

// trait bounds
// three ways to do it

// fn paint_red<T: Paint>(obj: T) {
//     obj.paint("red".to_owned());
// }

// fn paint_red(obj: &impl Paint) {
//     obj.paint("red".to_owned());
// }


// fn paint_red<T>(obj: Paint)
// where T: Paint + Park {
    // obj.paint("red".to_owned());
    // }
    
// allows for the Box smart pointer to be used in the call becuase it implements the Paint trait
fn paint_red(obj: &dyn Paint) {
    obj.paint("red".to_owned());
}

fn paint_vehicle_red<T>(obj: &T) 
where T: Vehicle {
    obj.paint("red".to_owned());
}

    // if you want to return different types from impl block
// to fix you can return trait object Box<dyn Paint> & return a box smart pointer Box::new() using dynamic dispatch
// dynamic dispatch means compiler doesn't know what the type is until runtime
// if using dynamic dispatch, will take a runtime penalty but allows for more dynamic return types (like below)
// fn create_paintable_object<T>() -> Box<dyn Paint> {
    // Box::new(House{})
// }
// fn create_paintable_object<T>() -> impl Paint {
//     House{}
// }

fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            info: VehicleInfo { make: "Chevy".to_owned(), model: "Suburban".to_owned(), year: 2016 }
        }
        )
    } else {
        Box::new(House{})
    }
    
}


fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Subaru".to_owned(),
            model: "Forester".to_owned(),
            year: 2015
        }
    };
    let house = House {};
    let obj = create_paintable_object(true);

    // cannot do below as mismatched types
    // let paintable_obj = vec![car, house];
    let paintable_obj: Vec<&dyn Paint> = vec![&car, &house];

    paint_red(&car);
    paint_red(&house);
    paint_red(obj.as_ref());
    paint_vehicle_red(&car);

}