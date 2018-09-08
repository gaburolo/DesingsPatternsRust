// Observer
trait Observer {
    fn update(&self);
}

// Guarda las funciones del observer
trait Observable<'a, T: Observer> {
    fn add_observer(&mut self, observer: &'a T);
    fn delete_observer(&mut self, observer: &'a T);
    fn notify_observers(&self);
}

//Se define la pantalla
struct Display {
    name: String,
}
struct Weather<'a, T:'a> {
    temperature: f64,
    observers: Vec<&'a T>
}
impl<'a> Weather<'a, Display> {
    fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
        self.notify_observers();
    }
}

//Se implementa en pantalla las funciones del Observer

impl Observer for Display {
    fn update(&self) {
        println!("Pantalla {} actualizada!", self.name);
    }
}
impl Display {
    fn new(name: String) -> Display {
        Display{name: name}
    }
}
impl std::cmp::PartialEq for Display {
    fn eq(&self, other: &Display) -> bool {
        self.name == other.name
    }
}
impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pantalla {}", self.name)
    }
}
impl<'a, T: Observer+PartialEq+std::fmt::Display> Observable<'a, T> for Weather<'a, T> {
    fn add_observer(&mut self, observer: &'a T) {
        println!("Se añadio Observer({});", observer);
        self.observers.push(observer);
    }
    fn delete_observer(&mut self, observer: &'a T) {
        let mut index = 0;
        let mut found = false;
        for &obs in self.observers.iter() {
            if obs == observer {
                println!("Se borro el observer({});", observer);
                found = true;
                break;
            }
            index += 1;
        }
        if found {
            self.observers.remove(index);
        }
    }
    fn notify_observers(&self) {
        for &observer in self.observers.iter() {
            observer.update();
        }
    }
}

fn main() {
    // reference must be valid for the block
    // so all bindings display must exist before weather
    let display = Display::new("Escritorio".to_string());
    let display2 = Display::new("Escritorio2".to_string());
    let mut weather = Weather{temperature: 19.0, observers: Vec::new()};
    weather.add_observer(&display);
    weather.add_observer(&display2);
    weather.set_temperature(20.0);
    weather.delete_observer(&display2);
    weather.set_temperature(21.0);
}
