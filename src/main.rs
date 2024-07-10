use std::{fmt, str::FromStr};

use every_variant::EveryVariant;
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use structopt::StructOpt;

mod vowels;

use crate::vowels::*;

pub const VN_EDICT_RAW: &str = include_str!("vnedict_lowercase.txt");

#[derive(Debug, Default, Clone, StructOpt)]
#[structopt(
    name = "vphone",
    about = "Find minimal pairs and sets of vietnamese phonemes"
)]
struct Options {
    #[structopt(short = "v")]
    vowels: Vec<String>,

    #[structopt(short = "i")]
    initial_consonants: Vec<String>,

    #[structopt(short = "f")]
    final_consonants: Vec<String>,

    #[structopt(short = "t")]
    tones: Vec<Tone>,

    #[structopt(short = "m", default_value = "tone", possible_values(&["initial_consonant", "tone", "vowel", "final_consonant"]))]
    kind: DeltaKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Entry<'a> {
    raw: &'a str,
}

impl<'a> Entry<'a> {
    fn new(line: &'a str) -> Option<Entry<'a>> {
        let (word, _definition) = line
            .split_once(" : ")
            .unwrap_or_else(|| panic!("Couldn't separate line {:?}", line));
        let syllables = word
            .trim()
            .split(char::is_whitespace)
            .filter_map(Syllable::new)
            .count();

        if syllables == 0 {
            None
        } else {
            Some(Entry { raw: word })
        }
    }

    fn par_syllables(self) -> impl ParallelIterator<Item = Syllable<'a>> {
        self.raw.par_split_whitespace().filter_map(Syllable::new)
    }

    fn syllables(self) -> impl Iterator<Item = Syllable<'a>> {
        self.raw.split_whitespace().filter_map(Syllable::new)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Syllable<'a> {
    raw: &'a str,
    initial_consonant: Option<&'a str>,
    vowel: Vowel<'a>,
    final_consonant: Option<&'a str>,
}

impl<'a> Syllable<'a> {
    fn new(raw: &'a str) -> Option<Self> {
        raw.split(|c| !is_vowel(c))
            .find(|s| !s.is_empty())
            .map(Vowel::new)
            .map(|vowel| {
                let consonants: Vec<&str> = raw.split(is_vowel).filter(|s| !s.is_empty()).collect();
                let (initial_consonant, final_consonant) =
                    match (consonants.first().copied(), consonants.get(1).copied()) {
                        both @ (Some(_initial_consonant), Some(_final_consonant)) => both,
                        (Some(consonant), None) => {
                            if raw.starts_with(consonant) {
                                (Some(consonant), None)
                            } else {
                                (None, Some(consonant))
                            }
                        }
                        (None, _) => (None, None),
                    };
                Self {
                    raw,
                    initial_consonant,
                    vowel,
                    final_consonant,
                }
            })
    }
}

fn words() -> impl ParallelIterator<Item = Entry<'static>> {
    VN_EDICT_RAW.par_lines().filter_map(Entry::new)
}

lazy_static! {
    static ref DELTA_KINDS: Vec<DeltaKind> = DeltaKind::every_variant();
}

#[derive(serde::Serialize)]
struct Delta<'a> {
    kind: DeltaKind,
    isolated_right: &'a str,
    isolated_left: &'a str,
    right: &'a str,
    left: &'a str,
}

enum DeltasOrLengthDifference<T> {
    Deltas(T),
    LengthDifference,
}

impl<'a> Delta<'a> {
    fn from_word_pair(
        left: Entry<'a>,
        right: Entry<'a>,
    ) -> DeltasOrLengthDifference<impl Iterator<Item = (usize, Delta<'a>)>> {
        if left.syllables().count() != right.syllables().count() {
            return DeltasOrLengthDifference::LengthDifference;
        }

        DeltasOrLengthDifference::Deltas(
            left.syllables()
                .zip(right.syllables())
                .enumerate()
                .flat_map(|(i, (a, b))| {
                    Delta::from_syllable_pair(left.raw, right.raw, a, b).map(move |d| (i, d))
                }),
        )
    }

    fn from_syllable_pair(
        left_word: &'a str,
        right_word: &'a str,
        left: Syllable<'a>,
        right: Syllable<'a>,
    ) -> impl Iterator<Item = Delta<'a>> {
        let maybe_delta = move |kind| {
            let (isolated_left, isolated_right) = match kind {
                DeltaKind::InitialConsonant => (
                    left.initial_consonant.unwrap_or(""),
                    right.initial_consonant.unwrap_or(""),
                ),
                DeltaKind::FinalConsonant => (
                    left.final_consonant.unwrap_or(""),
                    right.final_consonant.unwrap_or(""),
                ),
                DeltaKind::Tone => (left.vowel.tone().name(), right.vowel.tone().name()),
                DeltaKind::Vowel => (left.vowel.normal(), right.vowel.normal()),
            };

            if isolated_left != isolated_right {
                Some(Delta {
                    kind,
                    isolated_left,
                    isolated_right,
                    left: left_word,
                    right: right_word,
                })
            } else {
                None
            }
        };

        DELTA_KINDS.iter().copied().filter_map(maybe_delta)
    }
}

#[derive(EveryVariant, Debug, Clone, Eq, PartialEq, Copy, serde::Serialize)]
enum DeltaKind {
    InitialConsonant,
    Tone,
    Vowel,
    FinalConsonant,
}

impl Default for DeltaKind {
    fn default() -> Self {
        DeltaKind::Tone
    }
}

impl fmt::Display for DeltaKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                DeltaKind::InitialConsonant => "initial_consonant",
                DeltaKind::Tone => "tone",
                DeltaKind::Vowel => "vowel",
                DeltaKind::FinalConsonant => "final_consonant",
            }
        )
    }
}

impl FromStr for DeltaKind {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "initial_consonant" => Ok(DeltaKind::InitialConsonant),
            "tone" => Ok(DeltaKind::Tone),
            "vowel" => Ok(DeltaKind::Vowel),
            "final_consonant" => Ok(DeltaKind::FinalConsonant),
            _ => Err(format!(
                "Invalid delta kind. Available kinds: {:#?}",
                DELTA_KINDS.as_slice()
            )),
        }
    }
}

fn filter<'a>(options: &Options, words: impl ParallelIterator<Item = Entry<'a>>) -> Vec<Entry<'a>> {
    words
        .filter_map(|w| {
            let passing_vowel = w
                .syllables()
                .find_position(|s| options.vowels.iter().any(|v| s.vowel.normal() == v));
            let passing_initial_consonant = w.syllables().find_position(|s| {
                options
                    .initial_consonants
                    .iter()
                    .any(|c| s.initial_consonant == Some(c.as_str()))
            });

            let passing_final_consonant = w.syllables().find_position(|s| {
                options
                    .final_consonants
                    .iter()
                    .any(|c| s.final_consonant == Some(c.as_str()))
            });

            let passing_tone = w
                .syllables()
                .find_position(|s| options.tones.iter().copied().any(|t| s.vowel.tone() == t));

            if passing_tone.is_none() && !options.tones.is_empty() {
                return None;
            }

            if passing_vowel.is_none() && !options.vowels.is_empty() {
                return None;
            }

            if passing_initial_consonant.is_none() && !options.initial_consonants.is_empty() {
                return None;
            }

            if passing_final_consonant.is_none() && !options.final_consonants.is_empty() {
                return None;
            }

            if ![
                passing_tone,
                passing_vowel,
                passing_initial_consonant,
                passing_final_consonant,
            ]
            .into_iter()
            .filter_map(std::convert::identity)
            .all_equal()
            {
                return None;
            }

            Some(w)
        })
        .collect::<Vec<_>>()
}

fn main() {
    let options = Options::from_args();
    let mut writer = csv::Writer::from_writer(std::io::stdout());

    let filtered_words = filter(&options, words());

    let which_syllable_passed_predicate = |w: Entry<'_>| {
        w.syllables()
            .map(|s| Entry { raw: s.raw })
            .find_position(|w| !filter(&options, rayon::iter::once(*w)).is_empty())
            .expect("couldn't identify passing syllable index")
            .0
    };

    let pairs = filtered_words
        .into_iter()
        .permutations(2)
        .par_bridge()
        .filter_map(|v| {
            let (a, b) = (v[0], v[1]);
            let mut deltas = match Delta::from_word_pair(a, b) {
                DeltasOrLengthDifference::LengthDifference => return None,
                DeltasOrLengthDifference::Deltas(deltas) => deltas,
            };

            let (delta_syllable_index, delta) = deltas.next()?;
            if deltas.next().is_some() {
                // pair is not minimal
                return None;
            }

            if delta.kind != options.kind {
                return None;
            }

            let (a_syllable_index, b_syllable_index) = (
                which_syllable_passed_predicate(a),
                which_syllable_passed_predicate(b),
            );
            if a_syllable_index != delta_syllable_index || b_syllable_index != delta_syllable_index
            {
                return None;
            }

            Some(delta)
        })
        .collect::<Vec<_>>();

    #[derive(serde::Serialize)]
    struct Row<'a> {
        right: &'a str,
        left: &'a str,
    }


    for pair in pairs {
        writer.serialize(Row { left: pair.left, right: pair.right }).expect("failed to write row");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_entry() {
        let entry = Entry::new("phức tạp : complicated").unwrap();
        let syllables: Vec<Syllable> = entry.syllables().collect();

        assert_eq!(entry.raw, "phức tạp");
        assert_eq!(syllables[0].raw, "phức");
        assert_eq!(syllables[1].raw, "tạp");
    }

    #[test]
    fn filter_works() {
        let entry = Entry::new("phức tạp : complicated").unwrap();
        let words = rayon::iter::once(entry);

        assert_eq!(
            filter(
                &Options {
                    vowels: vec!["e".to_owned()],
                    ..Default::default()
                },
                words
            ),
            vec![]
        );
    }

    #[test]
    fn filter_preserves() {
        let entry = Entry::new("phức tạp : complicated").unwrap();
        let words = rayon::iter::once(entry);

        assert_eq!(
            filter(
                &Options {
                    vowels: vec!["a".to_owned()],
                    ..Default::default()
                },
                words
            ),
            vec![Entry {
                raw: "phức tạp"
            }]
        );
    }
}
