use every_variant::EveryVariant;
use itertools::Itertools;
use lazy_static::lazy_static;

mod vowels;

use crate::vowels::*;

const VN_EDICT_RAW: &str = include_str!("vnedict_lowercase.txt");

#[derive(Debug, Clone, Copy)]
struct Entry<'a> {
    raw: &'a str,
}

impl<'a> Entry<'a> {
    fn new(line: &'a str) -> Entry<'a> {
        let (word, _definition) = line
            .split_once(" : ")
            .unwrap_or_else(|| panic!("Couldn't separate line {:?}", line));
        let syllables = word
            .trim()
            .split(char::is_whitespace)
            .filter_map(Syllable::new)
            .count();

        assert_ne!(syllables, 0);
        Entry { raw: word }
    }

    fn syllables(self) -> impl Iterator<Item = Syllable<'a>> {
        self.raw.split_whitespace().filter_map(Syllable::new)
    }
}

#[derive(Debug, Clone)]
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

fn words() -> impl Iterator<Item = Entry<'static>> {
    VN_EDICT_RAW.lines().skip(1).map(Entry::new)
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

impl<'a> Delta<'a> {
    fn from_pair(left: Syllable<'a>, right: Syllable<'a>) -> impl Iterator<Item = Delta<'a>> {
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
                    left: left.raw,
                    right: right.raw,
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

fn main() {
    let mut writer = csv::Writer::from_writer(
        std::fs::File::create("minimal_pairs.csv").expect("couldn't open output csv"),
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_entry() {
        let entry = Entry::new("phức tạp : complicated");
        let syllables: Vec<Syllable> = entry.syllables().collect();

        assert_eq!(entry.raw, "phức tạp");
        assert_eq!(syllables[0].raw, "phức");
        assert_eq!(syllables[1].raw, "tạp");
    }
}
