/* Dit deel niet aanpassen; dit zorgt ervoor dat de sum/average polymorph over getallen te maken zijn */
mod prelude;

use prelude::Num;

/* Vanaf hier je eigen code */

// Opdracht 1
// Schrijf een functie die een piramide van variabele lengte afdrukt, zoals in het voorbeeld. Elke
// piramide bestaat uit asterixen ('*') en spaties (' '). Dus de eerste rij in het voorbeeld rechts
// bestaat uit 1 * gevolgd door 4 spaties. Voor extra uitdaging kan je ook gaan voor een versie met
// slechts één loop. Geef tot slot de mogelijkheid de piramide om te draaien m.b.v. een
// functieargument. Een pyramide van grootte 0 moet niet de code laten crashen. Je returnt dan de
// functie direct.
//
// Voorbeeld: pyramid(5, false) geeft:
// *
// **
// ***
// ****
// *****
// ****
// ***
// **
// *

pub fn pyramid(pyramid_size: usize, reverse: bool) {
    todo!()
}

// Opdracht 2
// Schrijf een functie die de eerste index teruggeeft waarop twee strings een verschillende waarde
// hebben. De functie geeft het resultaat in een Option. Als twee strings identiek zijn geeft de
// functie de waarde None terug.

pub fn first_difference(s1: &str, s2: &str) -> Option<usize> {
    todo!()
}

// Opdracht 3
// a. Schrijf een functie count() die berekent en returnt hoe vaak een geheel getal x in een lijst voorkomt. Gebruik hiervoor een iterator.
// 
// b. Schrijf een functie sequential_pairs_biggest_difference() die in een gegeven lijst
// (Vec<i64>) het grootste verschil tussen twee op een volgende getallen bepaalt en
// returnt. Schrijf hiervoor een handmatige loop.
// 
// c. Schrijf een functie in_binary_language(),  die bepaalt of een gegeven lijst met alleen 1’en en
// 0’en aan de volgende eisen voldoet:   - Het aantal enen is groter dan aan het aantal nullen   -
// Er mogen niet meer dan 12 nullen zijn.
// 
// De functie moet altijd false teruggeven als er ook maar één waarde anders dan 1 en 0 in de
// lijst zit.
// 
// Gebruik in je functie de functie count() die je hebt geschreven bij de vorige opgave.

pub fn count(x: i64, list: Vec<i64>) -> usize {
    todo!()
}

pub fn sequential_pairs_biggest_difference(list: Vec<i64>) -> i64 {
    todo!()
}

pub fn binary_language(list: Vec<i64>) -> bool {
    todo!()
}

// Opdracht 4
// Schrijf een functie die het gemiddelde van een lijst met floats berekent. Schrijf er ook een die
// als input een lijst van lijsten met floats krijgt, en als uitvoer een gemiddelde geeft. Het
// gemiddelde van een lege lijst is 0.

pub fn average(values: Vec<f64>) -> f64 {
    todo!()
}

pub fn average_lists(values: Vec<Vec<f64>>) -> f64 {
    todo!()
}

// Opdracht 5
// De rij van Fibonacci is genoemd naar Leonardo van Pisa, bijgenaamd Fibonacci, die de rij noemt in
// zijn boek Liber abaci uit 1202. De rij begint met 0 en 1 en vervolgens is elk volgende element
// van de rij steeds de som van de twee voorgaande elementen. Bij de rij gebruiken we de notatie fn
// voor het aangeven van het n-de element van de rij. f9 is bijvoorbeeld gelijk aan 34. De eerste
// elementen van de rij zijn dan als volgt:
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584
//
// Implementeer een functie die fn uitrekent gegeven integer n. De functie moet recursief zijn.
// Houdt tot slot rekening met integer overflow.

pub fn fibonacci(n: u64) -> u64 {
    todo!()
}

// Opdracht 6
// Schrijf een functie die de getallen 1 tot 100 print, maar print voor veelvouden van drie “fizz”
// in plaats van het getal en voor veelvouden van vijf print “buzz” in plaats van het getal.
// Getallen die zowel veelvoud zijn van drie als van vijf worden afgedrukt als “fizzbuzz”

pub fn fizzbuzz() {
    todo!()
}

// Opdracht 7
// Maak een generieke functie add, die een lijst van waardes bij elkaar optelt, gegeven dat de
// waardes `Num` implementeren. Deze waardes kun je met operatoren optellen, aftrekken, etc.
//
// Merk op dat het lastig is zomaar een concrete waarde (zoals 0 of 1) te maken met een dergelijk
// niet-gespecialiseerd type; hiervoor zijn de associated functies T::zero() en T::one()
// meegeleverd.
pub fn sum_poly<T: Num>(list: Vec<T>) -> T {
    todo!()
}

// Opdracht 8
// Schrijf een functies die gemiddeldes kan berekenen van alle lijsten van Num types. Houdt er
// rekening mee dat het gemiddelde van een lege lijst (nog steeds) 0 is.

pub fn average_poly<T: Num>(list: Vec<T>) -> T {
    todo!()
}

// Opdracht 9
// Gegeven de twee functies `describe_borrow` en `describe_clone`, maak een functie `use_both` die:
//  - een String `original` aanmaakt
//  - een waarde `borrowed_len` aanmaakt door `describe_borrow` op de original aan te roepen
//  - een waarde `moved_len` aanmaakt door `describe_move` op de original aan te roepen
//  - uiteindelijk de originele string en de twee berekende lengtes print.
//  Zoek zelf uit hoe de parameters mee te geven en waar en wanneer er wel/niet gecloned moet
//  worden.

fn describe_borrow(s: &String) -> usize {
    s.len()
}

fn describe_move(s: String) -> usize {
    s.len()
}

pub fn use_both() {
    todo!()
}

// !!! LET OP !!!
// Voor de laatste twee opdrachten moet je zelf de hele functie schrijven! Er is geen functie
// declaratie die je in moet vullen! Om deze reden geven de bijbehorende tests compile errors en
// staat deze uitgecomment. Zodra je de functies hieronder hebt gemaakt, ga dan naar naar
// tests/test.rs en uncomment steeds de test die bij je functie hoort.

// Opdracht 10
// Schrijf een functie `bump_score` die een tuple van een score (u32) en een naam (&str) accepteert,
// en de score met 1 ophoogt. Gebruik hiervoor een mutable borrow.
// Schrijf een functie `score_simulation` die een score aanmaakt met je eigen naam en een
// start-waarde 10. Roep vervolgens net zo vaak bump_score hier op aan tot de score je huidige
// leeftijd heeft bereikt.

// Schrijf hier je code.

// Opdracht 11
// a. Maak een functie `make_counter` die een u32 op de heap alloceert.
// b. Maak een functie `increment` die een mutable borrow accepteert van een heap-gealoceerde u32,
// en de waarde daarbinnen ophoogt.
// c. Maak een functie `do_count` die een nieuwe counter maakt, deze driemaal ophoogt, en het
// resultaat print.

// Schrijf hier je code.
