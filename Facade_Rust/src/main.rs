//Clase formateo, puedes elegir entre respaldar la infomacion de tu disco y formatearlo
struct Formateo {
    con: i32,
}

impl Formateo {
    fn new() -> Formateo {
        Formateo { con: 0 }
    }
    fn Respaldar(&mut self) {
        println!("Respaldado");
        println!("Puede Formatear ingresando Formatear(1)")
    }
    fn Formatear(&mut self, cont: i32) {
        if cont == 1 {
            println!("Formateado");
        } else {
            println!("Se debe respaldar primero")
        }
    }
}
//Clase instalarSo, puedes elegir instalar el SO y sus drivers
struct InstalarSO {
    con: i32,
}

impl InstalarSO {
    fn new() -> InstalarSO {
        InstalarSO { con: 0 }
    }
    fn InstalarSO(&mut self) {
        println!("Instalado el SO correctamente");
        println!("Puede Instalar drivers ingresando InstalarDrivers(1)")
    }
    fn InstalarDrivers(&mut self, con: i32) {
        if con == 1 {
            println!("Drivers Instalados");
        } else {
            println!("Debe instalar el SO primero")
        }
    }
}


//Clase Revisión, revisas el estado actual de tu pc
struct Revision {
    con: i32,
}

impl Revision {
    fn new() -> Revision {
        Revision { con: 0 }
    }
    pub fn RevisarPC(&mut self, estado: i32) {
        if estado < 50 {
            println!("Debe reparar Tarjeta Madre");
        } else {
            println!("Debe reparar Disco Duro");
        }
    }
}

//Clase reparar, en la que puedes reparar tanto el disco duro, como la tarjeta madre
struct Reparacion {
    con: i32,
}

impl Reparacion {
    fn new() -> Reparacion {
        Reparacion { con: 0 }
    }

    fn RepararDiscoDuro(&mut self) {
        println!("Disco duro Reparado al 100%");
    }

    fn RepararTarjetaMadre(&mut self) {
        println!("Tarjeta madre Reparada al 100%");
    }
}



//Patron de diseño Facade, crea un sistema que interacciona con las clases, falitando el uso al usuario
struct PcFacade {
    con: i32,
}

impl PcFacade {
    fn new() -> PcFacade {
        PcFacade { con: 0 }
    }
    pub fn FacadeLimpieza(&mut self) {
        println!("Iniciando Reparaciones");
        let mut form = Formateo::new();
        form.Respaldar();
        form.Formatear(1);
        let mut instalar = InstalarSO::new();
        instalar.InstalarSO();
        instalar.InstalarDrivers(1);
    }

    pub fn FacadeReparacionCompleta(&mut self, con: i32) {
        println!("Iniciando Reparaciones");
        let mut revisa = Revision::new();
        revisa.RevisarPC(con);
        let mut repara = Reparacion::new();
        repara.RepararDiscoDuro();
        repara.RepararTarjetaMadre();
        let mut form = Formateo::new();
        form.Respaldar();
        form.Formatear(1);
        let mut instalar = InstalarSO::new();
        instalar.InstalarSO();
        instalar.InstalarDrivers(1);
    }
    pub fn FacadeDiscoDuro(&mut self, con: i32) {
        println!("Iniciando Reparaciones");
        let mut revisa = Revision::new();
        revisa.RevisarPC(con);
        let mut repara = Reparacion::new();
        repara.RepararDiscoDuro();
        let mut form = Formateo::new();
        form.Respaldar();
        form.Formatear(1);
        let mut instalar = InstalarSO::new();
        instalar.InstalarSO();
        instalar.InstalarDrivers(1);
    }
}

fn main() {
    println!("-------------Sin Facade-------------");
    let mut form = Formateo::new();
    form.Respaldar();
    form.Formatear(1);
    let mut instalar = InstalarSO::new();
    instalar.InstalarSO();
    instalar.InstalarDrivers(1);
    println!("-------------Con Facade-------------");
    let mut pcFacade = PcFacade::new();
    pcFacade.FacadeLimpieza();
}