use crate::geffe::Geffe;
use crate::lfsr::LFSR;
use std::time::Instant;

mod geffe;
mod lfsr;

const STRING_DUMMIES: &str = "01010100000100011110010000111010011110110010111111010101111100100000110000011100001001010000000110011100000011001110100110000100011011101000111010001010011100110001011111101010111001011001101001011001110001010101010001111110011001001100110000101000011111011001010101110011010001000000010001010000111100100000001100100001001101000110010100000101001001011000111011110010101110001000000010111110111101110000001010010010011111100000101001011100010001110010010011010111110101011000111000010111010000100000001001010100001111101000010011111011101010011111010111110110111011011000010100010000101001000110001000010100101010111010100100101001110010110011100100011111110000101101110001111111001000010000010001100101111001001101000001011001101001000110101100000000100101100011000010000100100010111111111001011011010010110001011101000001001111011101001110101100011000101101000100000011100100101110011011001101101011101100000001111001010010001001000111110001000001011010110110101010100011000101000111111110001011110001000101100101110101010101010000010010101110110110010110000101100111000111111010100110110101100110001010010011101101101101111100000110000110110101111110011110101001101101100000111110101100000111101101000100001000010001001000001111100011110010101110101010100010100110101001010110100111111110010100110010010111011100001000000001110000000001011111001111101000001001001110111000000001010101010100100000011111100110000011100001011111101001110011100000100111110010001010101100101110110000101111111011101101011000111111000101000010010011110001110010001011011101110010010011011101111000100100111110111110001110010001010101010110000110100010111000111110010010110001101011101010110010011001001111011001001011010001010111001111010000110111111111010001110010000011001110101011110101000100011111011110101100010011111000111100111011110100011101110101000101110111111010101110001110100010111111001110110111010100011001111100000111010100001111111010001010110010111101000110110001011110011100111011000011000000000011011001101100100110010111010110100100011100001100";
const STRING_LEN: usize = 2048;

fn main() {
    let start = Instant::now();

    let l1_rec: u32 = (1u32 << 3) ^ 1;
    const L1_DEG: u8 = 25;

    let l2_rec: u32 = (1u32 << 6) ^ (1u32 << 2) ^ (1u32 << 1) ^ 1u32;
    const L2_DEG: u8 = 26;
    let l3_rec: u32 = (1u32 << 5) ^ (1u32 << 2) ^ (1u32 << 1) ^ 1u32;
    const L3_DEG: u8 = 27;

    let n = STRING_DUMMIES.chars().count();

    let mut r_seq: Vec<u8> = vec![0; n];
    for (i, c) in STRING_DUMMIES.chars().enumerate() {
        r_seq[i] = c.to_digit(10).unwrap() as u8;
    }
    let n1_req: u32 = 222;
    let c1: u32 = 71;
    let n2_req: u32 = 229;
    let c2: u32 = 74;

    let mut l1 = LFSR::new(l1_rec, L1_DEG);
    let mut l2 = LFSR::new(l2_rec, L2_DEG);
    let mut l3 = LFSR::new(l3_rec, L3_DEG);

    let mut l1_candidates: Vec<(u32, usize)> = Vec::new();
    {
        let cyclen = (1u64 << L1_DEG) + n1_req as u64;
        let mut curr_candidate = 1u32;
        let generated_seq = l1.generate_from_fill(curr_candidate, cyclen);
        for j in 0..(1u64 << L1_DEG) {
            let mut r = 0;
            for i in 0..(n1_req as u64) {
                r += (generated_seq[(j + i) as usize] ^ r_seq[i as usize]) as usize;
            }
            if r < c1 as usize {
                l1_candidates.push((curr_candidate, r));
            }
            curr_candidate = (curr_candidate >> 1)
                ^ (((generated_seq[L1_DEG as usize + j as usize] as u32) << (L1_DEG - 1)) as u32);
        }
    }
    println!("L1: {} кандидатів", l1_candidates.len());

    let mut l2_candidates: Vec<(u32, usize)> = Vec::new();
    {
        let cyclen = (1u64 << L2_DEG) + n2_req as u64;
        let mut curr_candidate = 1u32;
        let generated_seq = l2.generate_from_fill(curr_candidate, cyclen);
        for j in 0..(1u64 << L2_DEG) {
            let mut r = 0;
            for i in 0..(n2_req as u64) {
                r += (generated_seq[(j + i) as usize] ^ r_seq[i as usize]) as usize;
            }
            if r < c2 as usize {
                l2_candidates.push((curr_candidate, r));
            }
            curr_candidate = (curr_candidate >> 1)
                ^ (((generated_seq[L2_DEG as usize + j as usize] as u32) << (L2_DEG - 1)) as u32);
        }
    }
    println!("L2: {} кандидатів", l2_candidates.len());

    let mut l1_candidate = 0;
    let mut l2_candidate = 0;
    let mut l3_candidate = 0;
    {
        let mut best_candidate_l1 = l1_candidates[0].0;
        let mut min_deviation = l1_candidates[0].1 as f32 - 0.25 * n1_req as f32;
        for (b, r) in &l1_candidates {
            let deviation = *r as f32 - 0.25 * n1_req as f32;
            if deviation < min_deviation {
                best_candidate_l1 = *b;
                min_deviation = deviation;
            }
        }
        let mut best_candidate_l2 = l2_candidates[0].0;
        min_deviation = l2_candidates[0].1 as f32 - 0.25 * n2_req as f32;
        for (b, r) in &l2_candidates {
            let deviation = *r as f32 - 0.25 * n2_req as f32;
            if deviation < min_deviation {
                best_candidate_l2 = *b;
                min_deviation = deviation;
            }
        }

        let cyclen = (1u64 << L3_DEG) + n as u64;
        let mut curr_candidate = 1u32;
        let l3_seq = l3.generate_from_fill(curr_candidate, cyclen);
        let l1_seq = l1.generate_from_fill(best_candidate_l1, n as u64);
        let l2_seq = l2.generate_from_fill(best_candidate_l2, n as u64);
        for j in 0..(1u64 << L3_DEG) {
            let mut found = true;
            for i in 0..(n as u64) {
                if ((l3_seq[(j + i) as usize] & l1_seq[i as usize])
                    ^ ((1u8 ^ l3_seq[(j + i) as usize]) & l2_seq[i as usize]))
                    != r_seq[i as usize]
                {
                    found = false;
                    break;
                }
            }
            if found {
                l1_candidate = best_candidate_l1;
                l2_candidate = best_candidate_l2;
                l3_candidate = curr_candidate;
                break;
            }
            curr_candidate = (curr_candidate >> 1)
                ^ ((l3_seq[L3_DEG as usize + j as usize] as u32) << (L3_DEG - 1));
        }
    }
    println!("L3 всьо");
    println!(
        "L1 кандидат: {:>10} {:>32}",
        l1_candidate,
        format!("{:b}", l1_candidate)
    ); // виводити як бітсет довжини lx deg ?
    println!(
        "L2 кандидат: {:>10} {:>32}",
        l2_candidate,
        format!("{:b}", l2_candidate)
    );
    println!(
        "L3 кандидат: {:>10} {:>32}",
        l3_candidate,
        format!("{:b}", l3_candidate)
    );
    println!("\nПеревірка результатів");
    let mut generator = Geffe::new(l1, l2, l3);
    let test_gen = generator.generate(l1_candidate, l2_candidate, l3_candidate, STRING_LEN);
    let fin_gen = test_gen.clone();
    println!("Згенерована послідовність:");
    for c in test_gen {
        print!("{}", c);
    }
    println!("\nОчікувана послідовність:");
    let mut aaaa = true;
    for i in 0..n {
        print!("{}", r_seq[i]);
        if fin_gen[i] != r_seq[i] {
            aaaa = false;
        }
    }
    let stop = Instant::now();
    println!(
        "\nНа виконання витрачено: {:?}",
        stop.duration_since(start)
    );
    println!("Очікувана послідовність == згенерована: {aaaa}");
}
