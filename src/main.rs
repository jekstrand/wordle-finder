// Copyright (c) Jason Ekstrand 2022.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice,
//     this list of conditions and the following disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright
//     notice, this list of conditions and the following disclaimer in the
//     documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its
//     contributors may be used to endorse or promote products derived from
//     this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Word {
    word: String,
    letters: u32,
}

fn find_sets_of_5_dumb_loop(words: &Vec<Word>) {
    let num_words = words.len();

    let mut num_sets_2: u32 = 0;
    let mut num_sets_3: u32 = 0;
    let mut num_sets_4: u32 = 0;
    let mut num_sets_5: u32 = 0;

    for a in 0..num_words {
        let wa = &words[a];
        let letters1 = wa.letters;
        for b in (a+1)..num_words {
            let wb = &words[b];

            if (letters1 & wb.letters) != 0 {
                continue;
            }
            num_sets_2 += 1;
            let letters2 = letters1 | wb.letters;

            for c in (b+1)..num_words {
                let wc = &words[c];

                if (letters2 & wc.letters) != 0 {
                    continue;
                }
                num_sets_3 += 1;
                let letters3 = letters2 | wc.letters;

                for d in (c+1)..num_words {
                    let wd = &words[d];

                    if (letters3 & wd.letters) != 0 {
                        continue;
                    }
                    num_sets_4 += 1;
                    let letters4 = letters3 | wd.letters;

                    for e in (d+1)..num_words {
                        let we = &words[e];

                        if (letters4 & we.letters) != 0 {
                            continue;
                        }
                        num_sets_5 += 1;

                        assert!((letters4 | we.letters).count_ones() == 25);
                        println!("{}, {}, {}, {}, {}",
                                 wa.word, wb.word, wc.word, wd.word, we.word);
                    }
                }
            }
        }
    }
    println!("Found {} words with unique letters", num_words);
    println!("Found {} pairs of words with unique letters", num_sets_2);
    println!("Found {} sets of three words with unique letters", num_sets_3);
    println!("Found {} sets of four words with unique letters", num_sets_4);
    println!("Found {} sets of five words with unique letters", num_sets_5);
}

#[derive(Clone)]
struct Range {
    start: u32,
    end: u32,
}

struct WordPair {
    words: [u16; 2],
}

fn find_sets_of_5_pair_graph(words: &Vec<Word>) {
    // We assume word indices fit in a u16
    let num_words: u16 = words.len().try_into().unwrap();

    let mut pairs: Vec<WordPair> = Vec::new();
    let mut ranges: Vec<Range> = Vec::new();
    for a in 0..num_words {
        let range_start = pairs.len();
        let wa = &words[a as usize];
        for b in (a+1)..num_words {
            let wb = &words[b as usize];
            if (wa.letters & wb.letters) == 0 {
                pairs.push(WordPair {
                    words: [a, b],
                });
            }
        }

        assert!(ranges.len() == a.into());
        ranges.push(Range {
            start: range_start.try_into().unwrap(),
            end: pairs.len().try_into().unwrap(),
        });
    }
    assert!(ranges.len() == words.len());

    let mut num_sets_3: u32 = 0;
    let mut num_sets_4: u32 = 0;
    let mut num_sets_5: u32 = 0;

    for p in &pairs {
        let a = p.words[0] as usize;
        let b = p.words[1] as usize;
        let wa = &words[a];
        let wb = &words[b];
        let letters2 = wa.letters | wb.letters;

        for pb in &pairs[(ranges[b].start as usize)..(ranges[b].end as usize)] {
            let c = pb.words[1] as usize;
            let wc = &words[c];

            if (letters2 & wc.letters) != 0 {
                continue;
            }
            num_sets_3 += 1;
            let letters3 = letters2 | wc.letters;

            for pc in &pairs[(ranges[c].start as usize)..(ranges[c].end as usize)] {
                let d = pc.words[1] as usize;
                let wd = &words[d];

                if (letters3 & wd.letters) != 0 {
                    continue;
                }
                num_sets_4 += 1;
                let letters4 = letters3 | wd.letters;

                for pd in &pairs[(ranges[d].start as usize)..(ranges[d].end as usize)] {
                    let e = pd.words[1] as usize;
                    let we = &words[e];

                    if (letters4 & we.letters) != 0 {
                        continue;
                    }
                    num_sets_5 += 1;

                    assert!((letters4 | we.letters).count_ones() == 25);
                    println!("{}, {}, {}, {}, {}",
                             wa.word, wb.word, wc.word, wd.word, we.word);
                }
            }
        }
    }

    println!("Found {} words with unique letters", num_words);
    println!("Found {} pairs of words with unique letters", pairs.len());
    println!("Found {} sets of three words with unique letters", num_sets_3);
    println!("Found {} sets of four words with unique letters", num_sets_4);
    println!("Found {} sets of five words with unique letters", num_sets_5);
}

#[derive(Clone)]
struct WordSet {
    parent: u32,
    new_word: u16,
    letters: u32,
}

fn find_sets_of_5_worklist(words: &Vec<Word>) {
    // We assume word indices fit in a u16
    let num_words: u16 = words.len().try_into().unwrap();

    let mut sets: Vec<WordSet> = Vec::new();
    let mut ranges: Vec<Range> = Vec::new();
    ranges.resize(1 << 26, Range { start: 0, end: 0 });

    // Initial empty word set
    sets.push(WordSet {
        parent: u32::MAX,
        new_word: u16::MAX,
        letters: 0,
    });

    for i in 0..num_words {
        sets.push(WordSet {
            parent: 0,
            new_word: i,
            letters: words[i as usize].letters
        });
    }
    ranges[0] = Range {
        start: 1,
        end: sets.len().try_into().unwrap(),
    };

    let mut num_sets: [u32; 5] = [ num_words.into(), 0, 0, 0, 0 ];

    // Skip the empty set at the start
    let mut item: usize = 1;
    loop {
        if item == sets.len() {
            break;
        }

        let set = sets[item].clone();
        let set_idx: u32 = item.try_into().unwrap();

        let set_words = set.letters.count_ones() / 5;
        if set_words == 5 {
            break;
        }
        item += 1;

        let range = &ranges[sets[set.parent as usize].letters as usize];

        let start: u32 = sets.len().try_into().unwrap();
        for i in range.start..range.end {
            let other = sets[i as usize].clone();
            if other.new_word <= set.new_word {
                continue;
            }

            let new_word = &words[other.new_word as usize];
            if (set.letters & new_word.letters) == 0 {
                sets.push(WordSet {
                    parent: set_idx,
                    new_word: other.new_word,
                    letters: set.letters | new_word.letters,
                });
            }
        }
        let end: u32 = sets.len().try_into().unwrap();
        ranges[set.letters as usize] = Range {
            start: start,
            end: end,
        };
        num_sets[set_words as usize] += end - start;
    }

    for i in item..sets.len() {
        let mut set = &sets[i];
        assert!(set.letters.count_ones() == 25);
        let mut set_words = Vec::new();
        loop {
            set_words.push(set.new_word);
            set = &sets[set.parent as usize];
            if set.letters == 0 {
                break;
            }
        }
        assert!(set_words.len() == 5);
        println!("{}, {}, {}, {}, {}",
                 words[set_words[4] as usize].word,
                 words[set_words[3] as usize].word,
                 words[set_words[2] as usize].word,
                 words[set_words[1] as usize].word,
                 words[set_words[0] as usize].word);
    }

    println!("Found {} words with unique letters", num_sets[0]);
    println!("Found {} pairs of words with unique letters", num_sets[1]);
    println!("Found {} sets of three words with unique letters", num_sets[2]);
    println!("Found {} sets of four words with unique letters", num_sets[3]);
    println!("Found {} sets of five words with unique letters", num_sets[4]);
}

fn main() {
    // Open the file name given as a command line argument
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap();

    let mut words: Vec<Word> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let word = line.unwrap().trim().to_string();
        if word.len() != 5 {
            continue;
        }

        let mut letters = 0u32;
        for c in word.to_uppercase().as_bytes() {
            assert!(b'A' <= *c && *c <= b'Z');
            letters |= 1u32 << (*c - b'A');
        }
        if letters.count_ones() != 5 {
            continue;
        }

        words.push(Word {
            word: word,
            letters: letters,
        });
    }

    // Get rid of any annograms.  This isn't necessary but cuts down on
    // runtime and lets us verify results against Matt's algorithm
    words.sort_by_key(|w| w.letters.reverse_bits());
    words.dedup_by_key(|w| w.letters);

//    find_sets_of_5_dumb_loop(&words);
//    find_sets_of_5_pair_graph(&words);
    find_sets_of_5_worklist(&words);
}
