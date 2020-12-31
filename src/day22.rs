//use num_bigint::BigUint;
use std::collections::{HashSet, VecDeque};

pub fn run() {
    let input_bytes = include_bytes!("day22_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();
    let mut lines = input_string.lines();

    // Sweet: https://stackoverflow.com/questions/31374051/why-does-iteratortake-while-take-ownership-of-the-iterator
    let cards1: Vec<_> = lines
        .by_ref()
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let cards2: Vec<_> = lines.skip(1).map(|n| n.parse::<u32>().unwrap()).collect();

    part1(&cards1, &cards2);
    part2(&cards1, &cards2);
}

fn score_deck(deck: &VecDeque<u32>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, c)| *c as usize * (i + 1))
        .sum()
}

fn part1(cards1: &[u32], cards2: &[u32]) {
    let mut deck1: VecDeque<u32> = cards1.iter().copied().collect();
    let mut deck2: VecDeque<u32> = cards2.iter().copied().collect();

    while !deck1.is_empty() && !deck2.is_empty() {
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }

    let win_deck = if deck1.is_empty() { deck2 } else { deck1 };
    println!("Day 22, part 1: {}", score_deck(&win_deck));
}

//fn mult_pack_hash(deck: &VecDeque<u32>) -> BigUint {
//    deck.iter().fold(BigUint::from(0u32), |acc, c| {
//        acc * BigUint::from(50u32) + BigUint::from(c - 1)
//    })
//}

fn deck1_wins_rec(deck1: &mut VecDeque<u32>, deck2: &mut VecDeque<u32>) -> bool {
    let mut states: HashSet<(VecDeque<u32>, VecDeque<u32>)> = HashSet::new();
    //let mut states: HashSet<(BigUint, BigUint)> = HashSet::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        // Have we already encountered the same game state before?
        // I find the solution here quite elegant with the Hasher.
        // https://github.com/sheredom/AOC2020/blob/main/src/day22/mod.rs#L96
        // But I think it might be incorrect because there is a chance that it could fail
        // (very small chance though). And I think the check needs to happen at the
        // beginning of the loop.
        let hash = (deck1.clone(), deck2.clone());
        if states.contains(&hash) {
            return true;
        }
        states.insert(hash);
        //
        // I thought the following would be faster, but it isn't:
        // Instead of putting the entire VecDeques into a hashset let's compute some unique
        // "hashes" ourselves. It's based on:
        // http://number-none.com/product/Packing%20Integers/index.html
        //let hash = (mult_pack_hash(deck1), mult_pack_hash(deck2));
        //if states.contains(&hash) {
        //    return true;
        //}
        //states.insert(hash);

        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        let one_wins = if deck1.len() >= c1 as usize && deck2.len() >= c2 as usize {
            let mut deck1_copy: VecDeque<u32> = deck1.iter().copied().take(c1 as usize).collect();
            let mut deck2_copy: VecDeque<u32> = deck2.iter().copied().take(c2 as usize).collect();
            deck1_wins_rec(&mut deck1_copy, &mut deck2_copy)
        } else {
            c1 > c2
        };

        if one_wins {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }

    return deck2.is_empty();
}

fn part2(cards1: &[u32], cards2: &[u32]) {
    let mut deck1: VecDeque<u32> = cards1.iter().copied().collect();
    let mut deck2: VecDeque<u32> = cards2.iter().copied().collect();

    let win_deck = if deck1_wins_rec(&mut deck1, &mut deck2) {
        deck1
    } else {
        deck2
    };

    println!("Day 22, part 2: {}", score_deck(&win_deck));
}
