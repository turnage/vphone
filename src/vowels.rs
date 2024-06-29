use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

const NORMALIZED_VOWELS: [&'static str; 12] =
    ["a", "ă", "â", "e", "ê", "i", "o", "ô", "ơ", "u", "ư", "y"];

lazy_static! {
    static ref VOWELS: HashMap<char, (Tone, char)> = [
        // a
        ('a', (Tone::Flat, 'a')), ('á', (Tone::Rising, 'a')), ('à', (Tone::Falling, 'a')), ('ả', (Tone::Question, 'a')), ('ã', (Tone::Broken, 'a')), ('ạ', (Tone::LowBroken, 'a')),
        ('A', (Tone::Flat, 'a')), ('Á', (Tone::Rising, 'a')), ('À', (Tone::Falling, 'a')), ('Ả', (Tone::Question, 'a')), ('Ã', (Tone::Broken, 'a')), ('Ạ', (Tone::LowBroken, 'a')),
        // ă
        ('ă', (Tone::Flat, 'ă')), ('ắ', (Tone::Rising, 'ă')), ('ằ', (Tone::Falling, 'ă')), ('ẳ', (Tone::Question, 'ă')), ('ẵ', (Tone::Broken, 'ă')), ('ặ', (Tone::LowBroken, 'ă')),
        ('Ă', (Tone::Flat, 'ă')), ('Ắ', (Tone::Rising, 'ă')), ('Ằ', (Tone::Falling, 'ă')), ('Ẳ', (Tone::Question, 'ă')), ('Ẵ', (Tone::Broken, 'ă')), ('Ặ', (Tone::LowBroken, 'ă')),
        // â
        ('â', (Tone::Flat, 'â')), ('ấ', (Tone::Rising, 'â')), ('ầ', (Tone::Falling, 'â')), ('ẩ', (Tone::Question, 'â')), ('ẫ', (Tone::Broken, 'â')), ('ậ', (Tone::LowBroken, 'â')),
        ('Â', (Tone::Flat, 'â')), ('Ấ', (Tone::Rising, 'â')), ('Ầ', (Tone::Falling, 'â')), ('Ẩ', (Tone::Question, 'â')), ('Ẫ', (Tone::Broken, 'â')), ('Ậ', (Tone::LowBroken, 'â')),
        // e
        ('e', (Tone::Flat, 'e')), ('é', (Tone::Rising, 'e')), ('è', (Tone::Falling, 'e')), ('ẻ', (Tone::Question, 'e')), ('ẽ', (Tone::Broken, 'e')), ('ẹ', (Tone::LowBroken, 'e')),
        ('E', (Tone::Flat, 'e')), ('É', (Tone::Rising, 'e')), ('È', (Tone::Falling, 'e')), ('Ẻ', (Tone::Question, 'e')), ('Ẽ', (Tone::Broken, 'e')), ('Ẹ', (Tone::LowBroken, 'e')),
        // ê
        ('ê', (Tone::Flat, 'ê')), ('ế', (Tone::Rising, 'ê')), ('ề', (Tone::Falling, 'ê')), ('ể', (Tone::Question, 'ê')), ('ễ', (Tone::Broken, 'ê')), ('ệ', (Tone::LowBroken, 'ê')),
        ('Ê', (Tone::Flat, 'ê')), ('Ế', (Tone::Rising, 'ê')), ('Ề', (Tone::Falling, 'ê')), ('Ể', (Tone::Question, 'ê')), ('Ễ', (Tone::Broken, 'ê')), ('Ệ', (Tone::LowBroken, 'ê')),
        // i
        ('i', (Tone::Flat, 'i')), ('í', (Tone::Rising, 'i')), ('ì', (Tone::Falling, 'i')), ('ỉ', (Tone::Question, 'i')), ('ĩ', (Tone::Broken, 'i')), ('ị', (Tone::LowBroken, 'i')),
        ('I', (Tone::Flat, 'i')), ('Í', (Tone::Rising, 'i')), ('Ì', (Tone::Falling, 'i')), ('Ỉ', (Tone::Question, 'i')), ('Ĩ', (Tone::Broken, 'i')), ('Ị', (Tone::LowBroken, 'i')),
        // o
        ('o', (Tone::Flat, 'o')), ('ó', (Tone::Rising, 'o')), ('ò', (Tone::Falling, 'o')), ('ỏ', (Tone::Question, 'o')), ('õ', (Tone::Broken, 'o')), ('ọ', (Tone::LowBroken, 'o')),
        ('O', (Tone::Flat, 'o')), ('Ó', (Tone::Rising, 'o')), ('Ò', (Tone::Falling, 'o')), ('Ỏ', (Tone::Question, 'o')), ('Õ', (Tone::Broken, 'o')), ('Ọ', (Tone::LowBroken, 'o')),
        // ô
        ('ô', (Tone::Flat, 'ô')), ('ố', (Tone::Rising, 'ô')), ('ồ', (Tone::Falling, 'ô')), ('ổ', (Tone::Question, 'ô')), ('ỗ', (Tone::Broken, 'ô')), ('ộ', (Tone::LowBroken, 'ô')),
        ('Ô', (Tone::Flat, 'ô')), ('Ố', (Tone::Rising, 'ô')), ('Ồ', (Tone::Falling, 'ô')), ('Ổ', (Tone::Question, 'ô')), ('Ỗ', (Tone::Broken, 'ô')), ('Ộ', (Tone::LowBroken, 'ô')),
        // ơ
        ('ơ', (Tone::Flat, 'ơ')), ('ớ', (Tone::Rising, 'ơ')), ('ờ', (Tone::Falling, 'ơ')), ('ở', (Tone::Question, 'ơ')), ('ỡ', (Tone::Broken, 'ơ')), ('ợ', (Tone::LowBroken, 'ơ')),
        ('Ơ', (Tone::Flat, 'ơ')), ('Ớ', (Tone::Rising, 'ơ')), ('Ờ', (Tone::Falling, 'ơ')), ('Ở', (Tone::Question, 'ơ')), ('Ỡ', (Tone::Broken, 'ơ')), ('Ợ', (Tone::LowBroken, 'ơ')),
        // u
        ('u', (Tone::Flat, 'u')), ('ú', (Tone::Rising, 'u')), ('ù', (Tone::Falling, 'u')), ('ủ', (Tone::Question, 'u')), ('ũ', (Tone::Broken, 'u')), ('ụ', (Tone::LowBroken, 'u')),
        ('U', (Tone::Flat, 'u')), ('Ú', (Tone::Rising, 'u')), ('Ù', (Tone::Falling, 'u')), ('Ủ', (Tone::Question, 'u')), ('Ũ', (Tone::Broken, 'u')), ('Ụ', (Tone::LowBroken, 'u')),
        // ư
        ('ư', (Tone::Flat, 'ư')), ('ứ', (Tone::Rising, 'ư')), ('ừ', (Tone::Falling, 'ư')), ('ử', (Tone::Question, 'ư')), ('ữ', (Tone::Broken, 'ư')), ('ự', (Tone::LowBroken, 'ư')),
        ('Ư', (Tone::Flat, 'ư')), ('Ứ', (Tone::Rising, 'ư')), ('Ừ', (Tone::Falling, 'ư')), ('Ử', (Tone::Question, 'ư')), ('Ữ', (Tone::Broken, 'ư')), ('Ự', (Tone::LowBroken, 'ư')),
        // y
        ('y', (Tone::Flat, 'y')), ('ý', (Tone::Rising, 'y')), ('ỳ', (Tone::Falling, 'y')), ('ỷ', (Tone::Question, 'y')), ('ỹ', (Tone::Broken, 'y')), ('ỵ', (Tone::LowBroken, 'y')),
        ('Y', (Tone::Flat, 'y')), ('Ý', (Tone::Rising, 'y')), ('Ỳ', (Tone::Falling, 'y')), ('Ỷ', (Tone::Question, 'y')), ('Ỹ', (Tone::Broken, 'y')), ('Ỵ', (Tone::LowBroken, 'y')),
    ].into_iter().collect();

    static ref NORMALIZED_CLUSTERS: Vec<String> = NORMALIZED_VOWELS
        .into_iter()
        .combinations(3)
        .map(|v| format!("{}{}{}", v[0], v[1], v[2]))
        .chain(NORMALIZED_VOWELS.into_iter().combinations(2).map(|v| format!("{}{}", v[0], v[1])))
        .chain(NORMALIZED_VOWELS.into_iter().map(String::from))
        .collect();
}

fn normalize_vowel(c: char) -> char {
    VOWELS.get(&c).copied().expect("couldn't normalize vowel").1
}

pub fn normalize_cluster(raw: &str) -> &'static str {
    let normalized = || raw.chars().map(normalize_vowel);

    NORMALIZED_VOWELS
        .iter()
        .find(|v| itertools::equal(v.chars(), normalized()))
        .expect("couldn't normalize vowel cluster")
}

pub fn is_vowel(c: char) -> bool {
    VOWELS.get(&c).is_some()
}

#[derive(serde::Serialize, Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum Tone {
    Flat,
    Rising,
    Falling,
    Question,
    Broken,
    LowBroken,
}

impl Tone {
    pub fn name(self) -> &'static str {
        match self {
            Tone::Flat => "flat",
            Tone::Rising => "rising",
            Tone::Falling => "falling",
            Tone::Question => "question",
            Tone::Broken => "broken",
            Tone::LowBroken => "low_broken",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vowel<'a> {
    pub raw: &'a str,
    pub normal: &'static str,
    pub tone: Tone,
}

impl<'a> Vowel<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self {
            raw,
            normal: normalize_cluster(raw),
            tone: raw
                .chars()
                .map(|c| VOWELS.get(&c).expect("couldn't find vowel tone").0)
                .max()
                .expect("couldn't compare vowel candidate tones"),
        }
    }

    pub fn raw(self) -> &'a str {
        self.raw
    }

    pub fn normal(self) -> &'static str {
        self.normal
    }

    pub fn tone(self) -> Tone {
        self.tone
    }
}
