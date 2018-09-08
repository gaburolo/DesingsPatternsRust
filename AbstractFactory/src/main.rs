
//Se crea un objeto cualquiera

trait ConsolaPortable {
    fn createStation(&self);

}

//Se crea un objeto de difetente tipo

trait ConsolaFija {
    fn createStation(&self);

}

//Se crea una fabrica

trait FActory<F: ConsolaFija, P: ConsolaPortable> {
    fn new_consolaPortable(&self) -> P;
    fn new_consolaFija(&self) -> F;

}

//Se define la fabrica abstarcta

struct PlayStation4;

impl ConsolaFija for PlayStation4 {
    fn createStation(&self){
        println!("Consola creada (Play Station 4)");
    }

}

struct PlayStationVita;

impl ConsolaPortable for PlayStationVita {
    fn createStation(&self){
        println!("Consola creada (Play Station Vita)")
    }

}

struct NintendoWii;

impl ConsolaFija for NintendoWii {
    fn createStation(&self){
        println!("Consola creada (Nintendo Wii)");
    }

}

struct NintendoSwitch;

impl ConsolaPortable for NintendoSwitch {
    fn createStation(&self){
        println!("Consola creada (Nintendo Switch)")
    }

}


struct SonyFactory;

impl FActory<PlayStation4, PlayStationVita> for SonyFactory{
    fn new_consolaFija(&self) -> PlayStation4{
        return PlayStation4;
    }
    fn new_consolaPortable(&self) -> PlayStationVita{
        return PlayStationVita
    }
}

struct NintendoFactory;


impl FActory<NintendoWii, NintendoSwitch> for NintendoFactory{
    fn new_consolaFija(&self) -> NintendoWii{
        return NintendoWii;
    }
    fn new_consolaPortable(&self) -> NintendoSwitch{
        return NintendoSwitch
    }
}

fn main(){
    //Se crean diferentes fabricas
    let sony = SonyFactory;
    let nintendo = NintendoFactory;

    //Se crean las consolas

    let consolaFija = sony.new_consolaFija();
    consolaFija.createStation();

    let consolaFija = nintendo.new_consolaFija();
    consolaFija.createStation();

    let consolaPortable = sony.new_consolaPortable();
    consolaPortable.createStation();

    let consolaPortable = nintendo.new_consolaPortable();
    consolaPortable.createStation();
}


