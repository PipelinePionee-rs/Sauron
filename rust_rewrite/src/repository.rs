//TODO: Lav en pageRepository struckt. Den skal have 1 field som skal være en Connection type læs længere nede 

//TODO: Lav en implementation af pageRepository impl

// TODO: Du skal lave 1 asosiated function new() som er en slags construktor den skal nok tage en path til DB filen
    //så du kan lave en connetion. bliver kaldt ved at sige pageRepository::new()
    // TODO: new funtionen skal lave forbindelse til DB så den selv holder styr på sin egen forbindelse. du kan/skal bruge samme logik som
    // Lars burge i DB.rs

//TODO: Du skal lave en metode som er den logik Lars allerede har lavet i api 
let myrepo = pageRepository::new("sauron.db")