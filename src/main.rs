use std::f64::NAN;

use lazy_static::lazy_static;
use regex::Regex;
use slint::{include_modules, SharedString};
use evalexpr::{eval, eval_float, eval_int};

include_modules!();


lazy_static! {
    static ref VALID_EXPRESSION: Regex = Regex::new(r"(\+|-|\*|\/)[0-9]+").unwrap();
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    // TASK: Adaugă logica pentru `on_add_to_text_area`, pentru a manevra cazurile "C", "=", și alte input-uri
    ui.global::<Logic>().on_add_to_text_area(move |current_text, new_input| {
        let ui = ui_handle.unwrap();

        // TASK: Adaugă logica pentru cazurile:
        // - "C": Golirea zonei de text
        // - "=": Calcularea rezultatului și afișarea acestuia
        // - Altele: Adăugarea input-ului curent la zona de text
        // HINT: Folosește un `match` pentru a verifica valoarea `new_input`.
        match new_input.as_str() {
            "C" => {
                ui.global::<Logic>().set_textarea(SharedString::from(""));
            }
            "=" => {
                let current_text = current_text.as_str();
                let result = evaluate(current_text);
                ui.global::<Logic>().set_textarea(SharedString::from(result));
            }
            _ => {
                let new_text = format!("{}{}", current_text, new_input);
                ui.global::<Logic>().set_textarea(SharedString::from(new_text));
            }
        }
    });

    ui.run()
}

// TASK: Completează funcția `evaluate`, care verifică validitatea expresiei și returnează rezultatul sau un mesaj de eroare
// HINT: Folosește regex-ul `VALID_EXPRESSION` pentru a verifica dacă `input` este o expresie validă.
// Dacă expresia este validă, apelează funcția `compute`. Dacă nu, returnează un mesaj de eroare, cum ar fi "Invalid Expression".
fn evaluate(input: &str) -> String {
    match compute(input) {
        Some(result) => format!("{result:.2}"),
        None => "Invalid Expression".to_string(),
    }
}

// TASK: Implementează funcția `compute` pentru a realiza operațiile de bază (+, -, *, /) și a returna rezultatul
// HINT: Parcurge simbolurile de operare și folosește `.split()` pentru a împărți `input` în două părți: înainte și după simbol.
// Convertește fiecare parte în `f64` și returnează rezultatul în funcție de simbol.
fn compute(input: &str) -> Option<f64> {
    println!("evaluating: {}", input);
    match eval_float(input.trim()) {
        Err(_) => {
            eval_int(input.trim()).map(|x| x as f64).ok()
        }
        Ok(x) => Some(x),
    }
}
