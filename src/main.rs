// Pakettien tilat, InTransit / Kuljetuksessa, Delivered / Toimitettu, Returned / Palautettu.
enum ParcelStatus {
    Preparing,
    InTransit { courier: String },
    Delivered { recipient: String },
    Returned { reason: String },
}

// Tilauksen kuvaus.
fn status_message(status: ParcelStatus) -> String {
    match status {
        ParcelStatus::Preparing => String::from("Status: The parcel is being prepared."),
        ParcelStatus::InTransit { courier } => {
            format!("Status: The parcel is with courier {}.", courier)
        }
        ParcelStatus::Delivered { recipient } => {
            format!("Status: The parcel was delivered to {}.", recipient)
        }
        ParcelStatus::Returned { reason } => {
            format!("Status: The parcel was returned: {}.", reason)
        }
    }
}

// Toimitus ohje
fn print_delivery_note(note: Option<&str>) {
    if let Some(text) = note {
        println!("Delivery note: {}", text);
    } else {
        println!("There is no delivery note.");
    }
}

// Toimitusksen tarkistus
fn announce_delivery(status: &ParcelStatus) {
    if let ParcelStatus::Delivered { recipient } = status {
        println!("The parcel was delivered to {}.", recipient);
    } else {
        println!("The parcel has not been delivered yet.");
    }
}

// Kuitti.
fn create_receipt(status: ParcelStatus) {
    let ParcelStatus::Delivered { recipient } = status else {
        println!("Cannot create a receipt: the parcel was not delivered.");
        return;
    };

    println!("Receipt created for {}.", recipient);
}

fn main() {
    // Esimerkit.
    let prep = ParcelStatus::Preparing;
    let transit = ParcelStatus::InTransit {
        courier: String::from("Aino"),
    };
    let delivered = ParcelStatus::Delivered {
        recipient: String::from("Mikko"),
    };
    let returned = ParcelStatus::Returned {
        reason: String::from("address not found"),
    };

    // Tilat
    println!("{}", status_message(prep));
    println!("{}", status_message(transit));
    println!("{}", status_message(delivered));
    println!("{}", status_message(returned));

    // Toimitusohje test
    print_delivery_note(Some("Leave the parcel at reception."));
    print_delivery_note(None);

    // Ilmoitus test.
    let sample_delivered = ParcelStatus::Delivered {
        recipient: String::from("Mikko"),
    };
    let sample_not_delivered = ParcelStatus::Preparing;

    announce_delivery(&sample_delivered);
    announce_delivery(&sample_not_delivered);

    // Kuitti test.
    create_receipt(sample_delivered);
    create_receipt(sample_not_delivered);
}