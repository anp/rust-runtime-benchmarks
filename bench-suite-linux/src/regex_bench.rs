// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// NOTE: modified and munged together from multiple files.

use std::collections::BTreeMap;
use regex::internal::ProgramBuilder;
use regex_syntax::Expr;
use regex::Regex;

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("compile_simple".to_string()).or_insert(vec![]).push(compile_simple());
        results.entry("compile_simple_bytes".to_string())
               .or_insert(vec![])
               .push(compile_simple_bytes());
        results.entry("compile_small".to_string()).or_insert(vec![]).push(compile_small());
        results.entry("compile_small_bytes".to_string())
               .or_insert(vec![])
               .push(compile_small_bytes());
        results.entry("compile_huge".to_string()).or_insert(vec![]).push(compile_huge());
        results.entry("compile_huge_bytes".to_string())
               .or_insert(vec![])
               .push(compile_huge_bytes());
        results.entry("parse_simple".to_string()).or_insert(vec![]).push(parse_simple());
        results.entry("parse_small".to_string()).or_insert(vec![]).push(parse_small());
        results.entry("parse_huge".to_string()).or_insert(vec![]).push(parse_huge());
        results.entry("name_sherlock".to_string()).or_insert(vec![]).push(name_sherlock());
        results.entry("name_holmes".to_string()).or_insert(vec![]).push(name_holmes());
        results.entry("name_sherlock_holmes".to_string())
               .or_insert(vec![])
               .push(name_sherlock_holmes());
        results.entry("name_sherlock_nocase".to_string())
               .or_insert(vec![])
               .push(name_sherlock_nocase());
        results.entry("name_holmes_nocase".to_string())
               .or_insert(vec![])
               .push(name_holmes_nocase());
        results.entry("name_sherlock_holmes_nocase".to_string())
               .or_insert(vec![])
               .push(name_sherlock_holmes_nocase());
        results.entry("name_whitespace".to_string()).or_insert(vec![]).push(name_whitespace());
        results.entry("name_alt1".to_string()).or_insert(vec![]).push(name_alt1());
        results.entry("name_alt2".to_string()).or_insert(vec![]).push(name_alt2());
        results.entry("name_alt3".to_string()).or_insert(vec![]).push(name_alt3());
        results.entry("name_alt3_nocase".to_string()).or_insert(vec![]).push(name_alt3_nocase());
        results.entry("name_alt4".to_string()).or_insert(vec![]).push(name_alt4());
        results.entry("name_alt4_nocase".to_string()).or_insert(vec![]).push(name_alt4_nocase());
        results.entry("no_match_uncommon".to_string()).or_insert(vec![]).push(no_match_uncommon());
        results.entry("no_match_common".to_string()).or_insert(vec![]).push(no_match_common());
        results.entry("the_lower".to_string()).or_insert(vec![]).push(the_lower());
        results.entry("the_upper".to_string()).or_insert(vec![]).push(the_upper());
        results.entry("the_nocase".to_string()).or_insert(vec![]).push(the_nocase());
        results.entry("everything_greedy".to_string()).or_insert(vec![]).push(everything_greedy());
        results.entry("everything_greedy_nl".to_string())
               .or_insert(vec![])
               .push(everything_greedy_nl());
        results.entry("letters".to_string()).or_insert(vec![]).push(letters());
        results.entry("letters_upper".to_string()).or_insert(vec![]).push(letters_upper());
        results.entry("letters_lower".to_string()).or_insert(vec![]).push(letters_lower());
        results.entry("words".to_string()).or_insert(vec![]).push(words());
        results.entry("the_whitespace".to_string()).or_insert(vec![]).push(the_whitespace());
        results.entry("before_holmes".to_string()).or_insert(vec![]).push(before_holmes());
        results.entry("holmes_cochar_watson".to_string())
               .or_insert(vec![])
               .push(holmes_cochar_watson());
        results.entry("holmes_coword_watson".to_string())
               .or_insert(vec![])
               .push(holmes_coword_watson());
        results.entry("quotes".to_string()).or_insert(vec![]).push(quotes());
        results.entry("line_boundary_sherlock_holmes".to_string())
               .or_insert(vec![])
               .push(line_boundary_sherlock_holmes());
        results.entry("word_ending_n".to_string()).or_insert(vec![]).push(word_ending_n());
    }

    results
}

// Due to macro scoping rules, this definition only applies for the modules
// defined below. Effectively, it allows us to use the same tests for both
// native and dynamic regexes.
macro_rules! regex(
    ($re:expr) => {{
        use regex::internal::ExecBuilder;
        ExecBuilder::new($re).nfa().build().unwrap().into_regex()
    }}
);

fn compile_simple() -> u64 {
    bench!(1_000_000, {
        let re = r"^bc(d|e)*$";
        ProgramBuilder::new(&re).compile().unwrap()
    })
}

fn compile_simple_bytes() -> u64 {
    bench!(1_000_000, {
        let re = r"^bc(d|e)*$";
        ProgramBuilder::new(&re).bytes(true).compile().unwrap()
    })
}

fn compile_small() -> u64 {
    bench!(1_000_000, {
        let re = r"\p{L}|\p{N}|\s|.|\d";
        ProgramBuilder::new(&re).compile().unwrap()
    })
}

fn compile_small_bytes() -> u64 {
    bench!(10_000, {
        let re = r"\p{L}|\p{N}|\s|.|\d";
        ProgramBuilder::new(&re).bytes(true).compile().unwrap()
    })
}

fn compile_huge() -> u64 {
    bench!(100_000, {
        let re = r"\p{L}{100}";
        ProgramBuilder::new(&re).compile().unwrap()
    })
}

fn compile_huge_bytes() -> u64 {
    bench!(100, {
        let re = r"\p{L}{100}";
        ProgramBuilder::new(&re).bytes(true).compile().unwrap()
    })
}

fn parse_simple() -> u64 {
    bench!(100_000, {
        let re = r"^bc(d|e)*$";
        Expr::parse(re).unwrap()
    })
}

fn parse_small() -> u64 {
    bench!(100_000, {
        let re = r"\p{L}|\p{N}|\s|.|\d";
        Expr::parse(re).unwrap()
    })
}

fn parse_huge() -> u64 {
    bench!(1_000_000, {
        let re = r"\p{L}{100}";
        Expr::parse(re).unwrap()
    })
}

lazy_static! {
    static ref SHERLOCK: String = {
        include_str!("the-adventures-of-sherlock-holmes.txt").to_owned()
    };
}

macro_rules! bench_find {
    ($name:ident, $re:expr, $iters:expr) => {
        fn $name() -> u64 {
            lazy_static! {
                static ref RE: Regex = $re;
            };
            bench!($iters, {
                let count = RE.find_iter(&SHERLOCK).count();
            })
        }
    }
}

// These patterns are all single string literals that compile down to a variant
// of Boyer-Moore w/ memchr. This also demonstrates the impact that the
// frequency of a match has on performance.
bench_find!(name_sherlock, regex!("Sherlock"), 10_000);
bench_find!(name_holmes, regex!("Holmes"), 10_000);
bench_find!(name_sherlock_holmes, regex!("Sherlock Holmes"), 10_000);

// Like the above, except case insensitively.
// The prefix finder is broken at the moment, so these are probably a touch
// slower than they should be.
bench_find!(name_sherlock_nocase, regex!("(?i)Sherlock"), 10_000);
bench_find!(name_holmes_nocase, regex!("(?i)Holmes"), 10_000);
bench_find!(name_sherlock_holmes_nocase, regex!("(?i)Sherlock Holmes"), 10_000);

// Will quickly find instances of 'Sherlock', but then needs to fall back to
// the lazy DFA to process the Unicode aware `\s`.
bench_find!(name_whitespace, regex!(r"Sherlock\s+Holmes"), 10_000);

// Now try more variations on name matching.
// This one has two alternates that both start with 'S'. This should compile
// to an Aho-Corasick automaton that uses memchr. Never enters lazy DFA.
bench_find!(name_alt1, regex!("Sherlock|Street"), 100_000);
// This one doesn't have a common byte, but should still use Aho-Corasick.
// Never enters lazy DFA.
bench_find!(name_alt2, regex!("Sherlock|Holmes"), 10_000);
// OK, still using Aho-Corasick, but more patterns. Never enters lazy DFA.
bench_find!(
    name_alt3,
    regex!("Sherlock|Holmes|Watson|Irene|Adler|John|Baker"), 10_000);
// Still using Aho-Corasick, but needs the lazy DFA.
bench_find!(
    name_alt3_nocase,
    regex!("(?i)Sherlock|Holmes|Watson|Irene|Adler|John|Baker"), 1_000);
// Should still use Aho-Corasick for the prefixes in each alternate, but
// we need to use the lazy DFA to complete it.
bench_find!(name_alt4, regex!("Sher[a-z]+|Hol[a-z]+"), 10_000);
bench_find!(name_alt4_nocase, regex!("(?i)Sher[a-z]+|Hol[a-z]+"), 10_000);

// How long does it take to discover that there's no match?
// If it starts with an uncommon character, then not long at all.
// A common character? It might be 25x slower!
bench_find!(no_match_uncommon, regex!("zyx"), 10_000);
bench_find!(no_match_common, regex!("ayx"), 10_000);

// Various twiddling on very common words.
bench_find!(the_lower, regex!("the"), 100);
bench_find!(the_upper, regex!("The"), 1_000);
bench_find!(the_nocase, regex!("(?i)the"), 100);

// How fast can we match everything? This essentially defeats any clever prefix
// tricks and just executes the DFA across the entire input.
bench_find!(everything_greedy, regex!(".*"), 10);
bench_find!(everything_greedy_nl, regex!("(?s).*"), 10);

// How fast can we match every letter? This also defeats any clever prefix
// tricks.
bench_find!(letters, regex!(r"\pL"), 10);
bench_find!(letters_upper, regex!(r"\p{Lu}"), 100);
bench_find!(letters_lower, regex!(r"\p{Ll}"), 10);

// Similarly, for words.
bench_find!(words, regex!(r"\w+"), 10);

// Process whitespace after a very common word.
// Uses Boyer-Moore to find `the` and the lazy DFA for the rest.
bench_find!(the_whitespace, regex!(r"the\s+\w+"), 100);

// Find complete words before Holmes. The `\w` defeats any prefix
// optimizations, so it's the lazy DFA the entire way.
bench_find!(before_holmes, regex!(r"\w+\s+Holmes"), 10);

// Find Holmes co-occuring with Watson in a particular window of characters.
// This uses Aho-Corasick for the Holmes|Watson prefix, but the lazy DFA for
// the rest.
bench_find!(
    holmes_cochar_watson,
    regex!(r"Holmes.{0,25}Watson|Watson.{0,25}Holmes"), 1_000);

// Find Holmes co-occuring with Watson in a particular window of words.
// This uses Aho-Corasick for the Holmes|Watson prefix, but the lazy DFA for
// the rest.
bench_find!(
    holmes_coword_watson,
    regex!(r"Holmes(?:\s*.+\s*){0,10}Watson|Watson(?:\s*.+\s*){0,10}Holmes"),
    10);

// Find some subset of quotes in the text.
// This does detect the `"` or `'` prefix literal and does a quick scan for
// either byte before starting the lazy DFA.
bench_find!(quotes, regex!(r#"["'][^"']{0,30}[?!.]["']"#), 100);

// Finds all occurrences of Sherlock Holmes at the beginning or end of a line.
// The empty assertions defeat any detection of prefix literals, so it's the
// lazy DFA the entire way.
bench_find!(
    line_boundary_sherlock_holmes,
    regex!(r"(?m)^Sherlock Holmes|Sherlock Holmes$"), 10);

// All words ending in `n`.
// This uses word boundaries, which the lazy DFA cannot handle. Since the word
// boundary also defeats finding any literal prefixes, we have to use the
// NFA algorithm the whole way.
bench_find!(word_ending_n, regex!(r"\b\w+n\b"), 10);
