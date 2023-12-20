extern crate core;

use std::io::Write;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;

fn main() {
    let mut file_content = fs::read_to_string("TEXT").expect("problem with reading of file");
    let filtered = file_content
        .chars()
        .filter(|char| "абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ ".contains(|e| e == *char)
        )
        .map(|char| {
            match char {
                'ё' => 'е',
                'ъ' => 'ь',
                _ => char,
            }
        })
        .collect::<String>();
    let mut set_chars = HashSet::new();
    let mut char_amount_map = HashMap::new();
    let mut amount = 0usize;
    for char in filtered.chars() {
        let char = char.to_lowercase().next().unwrap();
        set_chars.insert(char);
        char_amount_map
            .entry(char)
            .and_modify(|counter| *counter += 1)
            .or_insert(1usize);
        amount += 1;
    }
    println!("всі символи що зустрічаються, відфільтроване: {set_chars:?}\n загальна кількість символів в відфільтрованому тексті: {amount}");

    println!("частота символів в відфільтрованому тексті: {char_amount_map:?}");
    let mut char_percent_map = char_amount_map
        .iter()
        .map(|(char, counter)| (*char, *counter as f64 / amount as f64))
        .collect::<HashMap<char, f64>>();
    println!("відсоток входжень символу в відфільтрованому тексті: {char_percent_map:?}");
    let one: f64 = char_percent_map
        .iter()
        .map(|(f1, f2)| *f2)
        .collect::<Vec<f64>>()
        .iter()
        .sum();
    // println!("{}", one);

    let mut char_per_sorted: Vec<(char, f64)> = char_percent_map.clone().into_iter().collect();
    char_per_sorted.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    let mut f_b_0 = File::create("char_percents.txt").unwrap();
    f_b_0.write_all(format!("сортовані відсотки входжень символу в відфільтрованому тексті: {char_per_sorted:?}").as_bytes()).unwrap();
    // println!("сортовані відсотки входжень символу в відфільтрованому тексті: {char_per_sorted:?}");
    let h1 = -char_per_sorted.iter().map(|(a,b )| *b * b.log2()).sum::<f64>();
    println!("H1 chars =  {}", h1);
    // let h2 = h1 * 0.5;
    // println!("H2 chars =  {}", h2);

    let chars: Vec<char> = filtered.chars().collect();

    let mut bigram1 = chars
        .chunks(2)
        .map(|f| String::from_iter(f).to_lowercase())
        .collect::<Vec<String>>();
    if bigram1.iter().last().unwrap().len() == 1 {
        bigram1.remove(bigram1.len());
    }
    let mut bigram_no_intersections = 0usize;
    let mut bigram_map1 = HashMap::new();
    bigram1.iter().for_each(|bigram| {
        bigram_no_intersections += 1;
        bigram_map1
            .entry(bigram.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1usize);
    });
    println!("кількість біграм (не сума входжень), без перетинів: {}",bigram_map1.len());

    let mut bigram_chastota_map1 = bigram_map1.iter()
        .map(|(s, amount)| (s.clone(), *amount as f64 / bigram_no_intersections as f64)).collect::<Vec<(String,f64)>>();
    //println!("{:?}", bigram_chastota_map1);

    let mut bigram1mapsorted: Vec<(String, f64)> = bigram_chastota_map1.clone();
    bigram1mapsorted.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    let mut f_b1 = File::create("bigrams_no_intersections_percents.txt").unwrap();
    f_b1.write_all(format!("сортована кількість біграм, без перетинів, у відсотках: {bigram1mapsorted:?}").as_bytes()).unwrap();

    // println!("сортована кількість біграм, без перетинів, у відсотках: {bigram1mapsorted:?}");

    let h1 = -bigram1mapsorted.iter().map(|(a,b )| *b * b.log2()).sum::<f64>();
    // println!("H1 bigram (no intersections) =  {}", h1);
    let h2 = h1 * 0.5;
    println!("H2 bigram (no intersections) =  {}", h2);

    let mut bigram2 = Vec::new();
    for i in 0..chars.len()-1 {
        let mut bigram = String::new();
        bigram.push(chars[i]);
        bigram.push(chars[i+1]);
        bigram2.push(bigram.to_lowercase());
    }
    let mut bigram_intersection_amount = 0usize;
    let mut bigram_map2 = HashMap::new();
    bigram2.iter().for_each(|bigram| {
        bigram_intersection_amount+=1;
        bigram_map2
            .entry(bigram.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1usize);
    });
    println!("кількість біграм (не сума входжень) з перетинами: {}",bigram_map2.len());

    // let mut bigram_chastota_map2 = bigram_map2.iter()
    //     .map(|(s, amount)| (s.clone(), amount ).collect::<Vec<(String,f64)>>();
    // println!("сортована кількість біграм, з перетинами: {:?}", bigram_chastota_map2);

    let mut bigram2mapsorted: Vec<(String, f64)> = bigram_map2.iter().map(|(a,b)| (a.clone(), (*b as f64 / bigram_intersection_amount as f64))).collect();
    bigram2mapsorted.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    let mut f_b2 = File::create("bigrams_intersections_percents.txt").unwrap();
    //println!("кількості входжень біграм, з перетинами, у відсотках: {bigram2mapsorted:?}");
    f_b2.write_all(format!("кількості входжень біграм, з перетинами, у відсотках: {bigram2mapsorted:?}").as_bytes()).unwrap();
    let h1 = -bigram2mapsorted.iter().map(|(a,b )| *b * b.log2()).sum::<f64>();
    // println!("H1 bigram (with intersections) =  {}", h1);
    let h2 = h1 * 0.5;
    println!("H2 bigram (with intersections) =  {}", h2);

}
