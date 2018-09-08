// Rocket Ship
trait RocketShip {
    fn turn_on(&self);
    fn turn_off(&self);
    fn blast_off(&self);
    fn fly(&self);
}

// estructura de la NASA Ship
struct NASAShip;

// Implementacion de la NASAShip
impl RocketShip for NASAShip {
    fn turn_on(&self) {
        println!("NASA Ship is turning on.")
    }

    fn turn_off(&self) {
        println!("NASA Ship is turning off.")
    }

    fn blast_off(&self) {
        println!("NASA Ship is blasting off.")
    }

    fn fly(&self) {
        println!("NASA Ship is flying away.")
    }
}

// SpaceX ship
trait SpaceXShip {
    fn ignition(&self);
    fn on(&self);
    fn off(&self);
    fn launch(&self);
    fn fly(&self);
}

// estructura SpaceX Dragon
struct SpaceXDragon;

// implementacion de SpaceX con Space X Dragon
impl SpaceXShip for SpaceXDragon {
    fn ignition(&self) {
        println!("Turning Dragon's ignition.")
    }

    fn on(&self) {
        println!("Turning on the Dragon.")
    }

    fn off(&self) {
        println!("Turning off the Dragon.")
    }

    fn launch(&self) {
        println!("Launching the Dragon")
    }

    fn fly(&self) {
        println!("The Dragon is flying away.")
    }
}

// Adapter para adaptar SpaceXShip
struct SpaceXAdapter {
    ship: SpaceXDragon
}

// SpaceX Adapter que agrega RocketShip metodos a SpaceXShip
impl RocketShip for SpaceXAdapter {
    fn turn_on(&self) {
        self.ship.ignition();
        self.ship.on();
    }

    fn turn_off(&self) {
        self.ship.off();
    }

    fn blast_off(&self) {
        self.ship.launch();
    }

    fn fly(&self) {
        self.ship.fly();
    }
}

// funcion pilot que implementa RocketShip metodos

fn pilot<S: RocketShip>(ship: &S) {
    ship.turn_on();
    ship.blast_off();
    ship.fly();
    ship.turn_off();
    print!("\n");
}

fn main() {
    // crea una nueva NASAShip
    let saturnShip = NASAShip;

    // hace volar la NASAShip
    println!("Piloting saturn ship.");
    pilot(&saturnShip);

    // Crea un Dragon
    let dragon = SpaceXDragon;

    // adapta dragon como una SpaceXDragon
    let dragon_adapter = SpaceXAdapter {
        ship: dragon
    };

    // piloto pilotea dragon
    println!("Piloting the Dragon Adapter.");
    pilot(&dragon_adapter);
}