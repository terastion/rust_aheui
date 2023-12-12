use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const HANGUL_START: u32 = 0xAC00; // start of hangul code block
const HANGUL_END: u32 = 0xD7A3; // end of hangul code block
const HANGUL_ONSET_MULT: u32 = 0x24C; // offset multiplier for initial consonant
const HANGUL_VOWEL_MULT: u32 = 0x1C; // offset multiplier for vowel

// initial hangul consonant enum
#[derive(Debug, FromPrimitive, PartialEq, Eq)]
pub enum HangulOnset {
    Giyeok, //ㄱ
    SsangGiyeok,    //ㄲ
    Nieun,  //ㄴ
    Digeut, //ㄷ
    SsangDigeut,    //ㄸ
    Rieul,  //ㄹ
    Mieum,  //ㅁ
    Bieup,  //ㅂ
    SsangBieup, //ㅃ
    Siot,   //ㅅ
    SsangSiot,  //ㅆ
    Ieung,  //ㅇ
    Jieut,  //ㅈ
    SsangJieut, //ㅉ
    Chieut, //ㅊ
    Kieuk,  //ㅋ
    Tigeut, //ㅌ
    Pieup,  //ㅍ
    Hieut,  //ㅎ
}


// hangul vowel enum
#[derive(Debug, FromPrimitive, PartialEq, Eq)]
pub enum HangulVowel {
    A,  //ㅏ
    Ae, //ㅐ
    Ya, //ㅑ
    Yae,    //ㅒ
    Eo, //ㅓ
    E,  //ㅔ
    Yeo,    //ㅕ
    Ye, //ㅖ
    O,  //ㅗ
    Wa, //ㅘ
    Wae,    //ㅙ
    Oe, //ㅚ
    Yo, //ㅛ
    U,  //ㅜ
    Wo, //ㅝ
    We, //ㅞ
    Wi, //ㅟ
    Yu, //ㅠ
    Eu, //ㅡ
    Ui, //ㅢ
    I,  //ㅣ
}

// final consonant enum
#[derive(Debug, FromPrimitive, PartialEq, Eq)]
pub enum HangulCoda {
    Null,   // empty
    Giyeok, //ㄱ
    SsangGiyeok,    //ㄲ
    GiyeokSiot, //ㄳ
    Nieun,  //ㄴ
    NieunJieut, //ㄵ
    NieunHieut, //ㄶ
    Digeut, //ㄷ
    Rieul,  //ㄹ
    RieulGiyeok,    //ㄺ
    RieulMieum, //ㄻ
    RieulBieup, //ㄼ
    RieulSiot,  //ㄽ
    RieulTigeut,    //ㄾ
    RieulPieup, //ㄿ
    RieulHieut, //ㅀ
    Mieum,  //ㅁ
    Bieup,  //ㅂ
    BieupSiot,  //ㅄ
    Siot,   //ㅅ
    SsangSiot,  //ㅆ
    Ieung,  //ㅇ
    Jieut,  //ㅈ
    Chieut, //ㅊ
    Kieuk,  //ㅋ
    Tigeut, //ㅌ
    Pieup,  //ㅍ
    Hieut,  //ㅎ
}

// a representation of a hangul syllable
// containing an onset, vowel, and coda
#[derive(Debug, PartialEq, Eq)]
pub struct HangulSyllable {
    pub onset: HangulOnset,
    pub vowel: HangulVowel,
    pub coda: HangulCoda,
}

impl HangulSyllable { 
    // create a HangulSyllable from a hangul character
    // if invalid, return None
    pub fn from_char(hangul_char: char) -> Option<Self> {
        let uni = u32::from(hangul_char);

        // return None if character is not hangul
        if uni < HANGUL_START || uni > HANGUL_END {
            return None;
        }

        // subtract hangul offset from cmd and get hangul char num
        let han_offset = uni - HANGUL_START;

        // extract initial const, medial vowel, and final const nums
        let onset_num = han_offset / HANGUL_ONSET_MULT;
        let onset_rem = han_offset % HANGUL_ONSET_MULT;
        let vowel_num = onset_rem / HANGUL_VOWEL_MULT;
        let coda_num = onset_rem % HANGUL_VOWEL_MULT;

        // convert numbers into hangul component enums
        let onset: HangulOnset = FromPrimitive::from_u32(onset_num)?;
        let vowel: HangulVowel = FromPrimitive::from_u32(vowel_num)?;
        let coda: HangulCoda = FromPrimitive::from_u32(coda_num)?;

        Some(Self {
            onset,
            vowel,
            coda,
        })
    }
}
