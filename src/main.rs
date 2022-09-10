use std::collections::BTreeMap;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::scope;

/// Iterates a B-Tree with `u32` keys and yields the keys which are greater than or equal to the
/// given `start`, and which have no bits in common with the given `start`.
struct Iter<'a> {
    word_list: &'a BTreeMap<u32, Vec<String>>,
    filter: u32,
    last_yielded: u32,
}

impl<'a> Iter<'a> {
    fn new(word_list: &'a BTreeMap<u32, Vec<String>>, start: u32) -> Iter<'a> {
        Iter {
            word_list,
            filter: start,
            last_yielded: start,
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut next = next_with_none_in_common(self.last_yielded, self.filter);

        while let Some((&k, _)) = self.word_list.range(next..).next() {
            if (k & self.filter) == 0 {
                self.last_yielded = k;
                return Some(k);
            }

            next = next_with_none_in_common(k, self.filter);
        }

        None
    }
}

/// Given a u32 value, finds the next largest value which contains none of the bits set in the given
/// `mask`.
#[inline]
const fn next_with_none_in_common(val: u32, mask: u32) -> u32 {
    let common = val & mask;
    let eligible_zeroes = !val & !((common + 1).next_power_of_two() - 1);
    let eligible_mask = eligible_zeroes ^ (eligible_zeroes - 1);
    (val | eligible_mask) & !(eligible_mask >> 1)
}

fn main() {
    let mut args = args();
    let _exec_name = args.next();
    let file = args.next().expect("No file name given");
    if args.next().is_some() {
        panic!("Too many arguments");
    }

    let mut word_list = BTreeMap::new();

    for line in BufReader::new(File::open(file).unwrap())
        .lines()
        .map(Result::unwrap)
        .filter(|x| x.len() == 5)
    {
        let key = key(&line);
        if key.count_ones() != 5 {
            continue;
        }
        word_list.entry(key).or_insert_with(Vec::new).push(line);
    }

    let found = AtomicUsize::new(0);
    let found_anagrams = AtomicUsize::new(0);

    scope(|s| {
        let cpus = num_cpus::get();
        let word_list = &word_list;
        let found = &found;
        let found_anagrams = &found_anagrams;
        for cpu in 0..cpus {
            s.spawn(move || {
                for result in word_list
                    .keys()
                    .skip(cpu)
                    .step_by(cpus)
                    .copied()
                    .flat_map(|a| Iter::new(word_list, a).map(move |b| (vec![a, b], a | b)))
                    .flat_map(|(accum, mask)| {
                        Iter::new(word_list, mask).map(move |new| {
                            let mut accum = accum.clone();
                            accum.push(new);
                            (accum, mask | new)
                        })
                    })
                    .flat_map(|(accum, mask)| {
                        Iter::new(word_list, mask).map(move |new| {
                            let mut accum = accum.clone();
                            accum.push(new);
                            (accum, mask | new)
                        })
                    })
                    .flat_map(|(accum, mask)| {
                        Iter::new(word_list, mask).map(move |new| {
                            let mut accum = accum.clone();
                            accum.push(new);
                            accum
                        })
                    })
                {
                    let mut total = 1;
                    let mut got = Vec::with_capacity(result.len());
                    for (word, count) in result.into_iter().map(|x| {
                        let words = word_list.get(&x).unwrap();
                        (words[0].clone(), words.len())
                    }) {
                        total *= count;
                        got.push(word);
                    }

                    let got = got.join(", ");

                    if total == 1 {
                        println!("{got}");
                    } else {
                        println!("{got} (plus {} others with anagrams)", total - 1);
                    }
                    found.fetch_add(1, Ordering::Relaxed);
                    found_anagrams.fetch_add(total, Ordering::Relaxed);
                }
            });
        }
    });

    let found = found.load(Ordering::Relaxed);
    let found_anagrams = found_anagrams.load(Ordering::Relaxed);
    println!();
    println!("Total: {found} ({found_anagrams} with anagrams)");
}

fn key(s: &str) -> u32 {
    s.chars()
        .map(|x| 1 << (x as u32 - 'a' as u32))
        .fold(0, |x, y| x | y)
}
