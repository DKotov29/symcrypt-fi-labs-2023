use std::collections::HashMap;
use std::fs;
use std::fs::File;

// fn main() {
//     let russian_char = 'б'; // The starting Russian character
//
//     let incremented_char = char::from_u32(((russian_char as u32) + 2)).unwrap(); // Increment the Unicode scalar value by 2
//
//     println!("Original: {}", russian_char);
//     println!("Incremented: {}", incremented_char);
// }
use std::io::Read;
use std::iter::Iterator;

const ALPH: &str = "абвгдежзийклмнопрстуфхцчшщъыьэюя";
const ALPH_LEN: usize = 32usize;
const PR: [f64; 32] = [0.0792, 0.0171, 0.0433, 0.0174, 0.0305, 0.0841, 0.0105, 0.0175, 0.0683, 0.0112, 0.0336, 0.0500, 0.0326, 0.0672, 0.1108, 0.0281, 0.0445, 0.0533, 0.0618, 0.0280, 0.0019, 0.0089, 0.0036, 0.0147, 0.0081, 0.0037, 0.0002, 0.0196, 0.0192, 0.0038, 0.0061, 0.0213];
// static mut RING: Lazy<HashMap<char, usize>, HashMap<char, usize>> = Lazy::new(HashMap::from_iter(ALPH.chars().enumerate().map(|(i1, i2)| (i2, i1))));

fn main() {
    let ring: HashMap<char, usize> = HashMap::from_iter(ALPH.chars().enumerate().map(|(i1, i2)| (i2, i1)));
    println!("{ring:#?}");
    // for (e, char) in ALPH.chars().enumerate() {
    //     unsafe { RING.insert(char, e); }
    // }
    let i_expected = PR.iter().map(|p| p.powf(2.0f64)).sum::<f64>();
    let i_0 = 1.0f64 / ALPH_LEN as f64;

    // let mut file = File::create("text.txt").unwrap();
    let mut file_text = fs::read_to_string("text.txt").unwrap();
    // file.read_to_string(&mut file_text).unwrap();
    // let mut to_encrypt_file = File::open("to_encrypt_text.txt").unwrap();
    let mut to_encrypt_file_text = fs::read_to_string("to_encrypt_text.txt").unwrap();
    // to_encrypt_file.read_to_string(&mut to_encrypt_file_text).unwrap();

    let key2 = "на";
    let key3 = "ана";
    let key4 = "кара";
    let key5 = "шфата";
    let keyn = "фалктщуслаиьакз";

    let original = process_text(to_encrypt_file_text.as_str());
    println!("processed: \"{original}\"");
    let encrypted_2 = unsafe { encrypt(original.as_str(), key2, &ring) };
    let encrypted_3 = unsafe { encrypt(original.as_str(), key3, &ring) };
    let encrypted_4 = unsafe { encrypt(original.as_str(), key4, &ring) };
    let encrypted_5 = unsafe { encrypt(original.as_str(), key5, &ring) };
    let encrypted_n = unsafe { encrypt(original.as_str(), keyn, &ring) };
    println!("enc: {encrypted_2}");
    println!("Indicies of coincidence");
    println!("I_0 in theory: {i_0}");
    println!("I_M in theory: {i_expected}");
    println!("I for original message: {}", coincidence_index(original.as_str()));
    println!("I_{}: {}", key2.len(), coincidence_index(encrypted_2.as_str()));
    println!("I_{}: {}", key3.len(), coincidence_index(encrypted_3.as_str()));
    println!("I_{}: {}", key4.len(), coincidence_index(encrypted_4.as_str()));
    println!("I_{}: {}", key5.len(), coincidence_index(encrypted_5.as_str()));
    println!("I_{}: {}", keyn.len(), coincidence_index(encrypted_n.as_str()));
}

fn process_text(s: &str) -> String {
    let mut text = String::with_capacity(s.len());

    for i in s.to_lowercase().chars() {
        let mut new = i;
        if new == 'Ё' || new == 'ё' {
            new = 'е';
        } else if ('а'..='я').contains(&new) {} else {
            continue;
        }
        text.push(new);
    }
    text
}

fn coincidence_index(text: &str) -> f64 {
    let mut sum = 0usize;
    for i in ALPH.chars() {
        let occ = text.chars().filter(|c| c == &i).count();
        sum += occ * (occ - 1)
    }
    sum as f64 / ((text.chars().count() * (text.chars().count() - 1)) as f64)
}

unsafe fn encrypt(s: &str, key: &str, ring: &HashMap<char, usize>) -> String {
    let mut res = String::with_capacity(s.len());
    for i in (0..s.chars().count()).step_by(key.chars().count()) {
        for j in 0..key.chars().count() {
            if i + j == s.chars().count() {
                break;
            }
            res.push(ALPH.chars().nth((*ring.get(&(s.chars().nth(i + j).unwrap())).unwrap()
                + ring.get(&key.chars().nth(j).unwrap()).unwrap()) % ALPH_LEN).unwrap());

        }
    }
    res
}