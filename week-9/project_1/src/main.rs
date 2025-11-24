use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Nigeria_Brewery_Limited {
    Lager: String,
    Stout: String,
    Non_Alcoholic: String,
}
fn main() {
    //Creating Sample Data
    let Brewery_Limited = vec![
        Nigeria_Brewery_Limited {
            Lager: "33 Export".to_string(),
            Stout: "Legend".to_string(),
            Non_Alcoholic: "Maltina".to_string(),
        },
        Nigeria_Brewery_Limited {
            Lager: "Desperados".to_string(),
            Stout: "Turbo King".to_string(),
            Non_Alcoholic: "Amstel Malta".to_string(),
        },
        Nigeria_Brewery_Limited {
            Lager: "Goldberg".to_string(),
            Stout: "Williams".to_string(),
            Non_Alcoholic: "Malta Gold".to_string(),
        },
        Nigeria_Brewery_Limited {
            Lager: "Gulder".to_string(),
            Stout: "".to_string(),
            Non_Alcoholic: "Fayrouz".to_string(),
        },
        Nigeria_Brewery_Limited {
            Lager: "Heineken".to_string(),
            Stout: "".to_string(),
            Non_Alcoholic: "".to_string(),
        },
        Nigeria_Brewery_Limited {
            Lager: "Star".to_string(),
            Stout: "".to_string(),
            Non_Alcoholic: "".to_string(),
        },
    ];

    let table = Table::new(Brewery_Limited);

    println!("{}", table);
}
