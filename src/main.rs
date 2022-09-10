use std::collections::BTreeMap;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

struct Node {
    key: u32,
    children: Vec<Node>,
}

impl Node {
    fn insert(&mut self, key: u32) {
        for child in self.children.iter_mut() {
            if child.key & key == 0 {
                child.insert(key);
            }
        }
        self.children.push(Node {
            key,
            children: Vec::new(),
        });
    }
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

    let mut root = Node {
        key: 0,
        children: Vec::with_capacity(word_list.len()),
    };

    for key in word_list.keys().copied() {
        root.insert(key);
    }

    let mut found = 0;
    let mut found_anagrams = 0;

    for item in root
        .children
        .into_iter()
        .flat_map(|x| x.children.into_iter().map(move |y| (y, x.key)))
        .flat_map(|(x, a)| x.children.into_iter().map(move |y| (y, a, x.key)))
        .flat_map(|(x, a, b)| x.children.into_iter().map(move |y| (y, a, b, x.key)))
        .flat_map(|(x, a, b, c)| x.children.into_iter().map(move |y| [a, b, c, x.key, y.key]))
    {
        let mut anagrams = 1;
        let got = item
            .iter()
            .map(|f| {
                let list = word_list.get(f).unwrap();
                anagrams *= list.len();
                list[0].clone()
            })
            .collect::<Vec<_>>()
            .join(", ");

        if anagrams == 1 {
            println!("{got}")
        } else if anagrams == 2 {
            println!("{got} (plus 1 other with anagrams)")
        } else {
            println!("{got} (plus {} others with anagrams)", anagrams - 1)
        }
        found += 1;
        found_anagrams += anagrams;
    }

    println!();
    println!("Total: {found} ({found_anagrams} with anagrams)");
}

fn key(s: &str) -> u32 {
    s.chars()
        .map(|x| 1 << (x as u32 - 'a' as u32))
        .fold(0, |x, y| x | y)
}
