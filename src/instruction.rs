use std::fmt;
use super::hangul::*;

// aheui action enum
#[derive(Debug, Clone, Copy)]
pub enum AheuiOperation {
    Null, //ㅇ
    Terminate, //ㅎ
    Add, //ㄷ
    Multiply, //ㄸ
    Divide, //ㄴ
    Subtract, //ㅌ
    Modulo, //ㄹ
    Pop, //ㅁ
    Push, //ㅂ
    Duplicate, //ㅃ
    Swap, //ㅍ
    StoreSelect, //ㅅ
    StoreTransfer, //ㅆ
    Compare, //ㅈ
    Fork, //ㅊ
}

impl AheuiOperation {
    // attempt an arithmetic operation based on enum
    pub fn arithmetic_operation(&self, a: isize, b: isize) -> Option<isize> {
        match *self {
            AheuiOperation::Add => a.checked_add(b),
            AheuiOperation::Multiply => a.checked_mul(b),
            AheuiOperation::Divide => a.checked_div(b),
            AheuiOperation::Subtract => a.checked_sub(b),
            AheuiOperation::Modulo => a.checked_rem(b),
            _ => None,
        }
    }
}

// aheui direction enum
#[derive(Debug, Clone, Copy)]
pub enum AheuiDirection {
    Null,
    Up(bool),
    Down(bool),
    Left(bool),
    Right(bool),
    ReflectX,
    ReflectY,
    ReflectXY,
}

impl AheuiDirection {
    // reflect direction along X axis
    pub fn reflect_x(&mut self) {
        match *self {
            AheuiDirection::Left(b) => *self = AheuiDirection::Right(b),
            AheuiDirection::Right(b) => *self = AheuiDirection::Left(b),
            _ => {},
        };
    }

    // reflect direction along Y axis
    pub fn reflect_y(&mut self) {
        match *self {
            AheuiDirection::Up(b) => *self = AheuiDirection::Down(b),
            AheuiDirection::Down(b) => *self = AheuiDirection::Up(b),
            _ => {},
        };
    }

    // reflect direction along both X and Y axes
    pub fn reflect_xy(&mut self) {
        match *self {
            AheuiDirection::Up(b) => *self = AheuiDirection::Down(b),
            AheuiDirection::Down(b) => *self = AheuiDirection::Up(b),
            AheuiDirection::Left(b) => *self = AheuiDirection::Right(b),
            AheuiDirection::Right(b) => *self = AheuiDirection::Left(b),
            _ => {},
        };
    }
}

// aheui argument enum
#[derive(Debug, Clone, Copy)]
pub enum AheuiArgument {
    Null,
    AsInt,
    AsChar,
    Storage(usize),
    Number(usize),
}

// representation of an aheui instruction
// each instruction contains an operation (onset),
// a direction (vowel), and an argument (coda)
pub struct AheuiInstruction {
    pub operation: AheuiOperation,
    pub direction: AheuiDirection,
    pub argument: AheuiArgument,
    pub character: char,
}

impl AheuiInstruction {
    pub fn from_char(cmd: char) -> Self {
        // create hangul syllable from character
        // and return a null instruction if invalid or non-hangul
        let syllable = match HangulSyllable::from_char(cmd) {
            Some(syllable) => syllable,
            None => return AheuiInstruction::null(),
        };
        
        // convert init const to AheuiOperation
        let operation = match syllable.onset {
            HangulOnset::Hieut => AheuiOperation::Terminate,
            HangulOnset::Digeut => AheuiOperation::Add,
            HangulOnset::SsangDigeut => AheuiOperation::Multiply,
            HangulOnset::Nieun => AheuiOperation::Divide,
            HangulOnset::Tigeut => AheuiOperation::Subtract,
            HangulOnset::Rieul => AheuiOperation::Modulo,
            HangulOnset::Mieum => AheuiOperation::Pop,
            HangulOnset::Bieup => AheuiOperation::Push,
            HangulOnset::SsangBieup => AheuiOperation::Duplicate,
            HangulOnset::Siot => AheuiOperation::StoreSelect,
            HangulOnset::SsangSiot => AheuiOperation::StoreTransfer,
            HangulOnset::Jieut => AheuiOperation::Compare,
            HangulOnset::Chieut => AheuiOperation::Fork,
            HangulOnset::Pieup => AheuiOperation::Swap,
            _ => AheuiOperation::Null,
        };

        // convert final vowel to AheuiArgument depending on command
        let argument = match operation {
            AheuiOperation::Pop => match syllable.coda {
                HangulCoda::Ieung => AheuiArgument::AsInt,
                HangulCoda::Hieut => AheuiArgument::AsChar,
                _ => AheuiArgument::Null, // should not happen
            },
            AheuiOperation::Push => match syllable.coda {
                HangulCoda::Null => AheuiArgument::Number(0),
                HangulCoda::Ieung => AheuiArgument::AsInt,
                HangulCoda::Hieut => AheuiArgument::AsChar,
                HangulCoda::Giyeok |
                HangulCoda::Nieun |
                HangulCoda::Siot => AheuiArgument::Number(2),
                HangulCoda::Digeut |
                HangulCoda::Jieut |
                HangulCoda::Kieuk => AheuiArgument::Number(3),
                HangulCoda::Mieum |
                HangulCoda::Bieup |
                HangulCoda::Pieup |
                HangulCoda::Chieut |
                HangulCoda::Tigeut |
                HangulCoda::SsangGiyeok |
                HangulCoda::GiyeokSiot |
                HangulCoda::SsangSiot => AheuiArgument::Number(4),
                HangulCoda::Rieul |
                HangulCoda::NieunJieut |
                HangulCoda::NieunHieut => AheuiArgument::Number(5),
                HangulCoda::BieupSiot => AheuiArgument::Number(6),
                HangulCoda::RieulGiyeok |
                HangulCoda::RieulSiot => AheuiArgument::Number(7),
                HangulCoda::RieulHieut => AheuiArgument::Number(8),
                HangulCoda::RieulMieum |
                HangulCoda::RieulBieup |
                HangulCoda::RieulTigeut |
                HangulCoda::RieulPieup => AheuiArgument::Number(9),
            },
            AheuiOperation::StoreSelect |
            AheuiOperation::StoreTransfer => AheuiArgument::Storage(syllable.coda as usize),
            _ => AheuiArgument::Null,
        };

        // convert vowel into direction of AheuiCommand
        let direction = match syllable.vowel {
            HangulVowel::A => AheuiDirection::Right(false),
            HangulVowel::Ya => AheuiDirection::Right(true),
            HangulVowel::Eo => AheuiDirection::Left(false),
            HangulVowel::Yeo => AheuiDirection::Left(true),
            HangulVowel::O => AheuiDirection::Up(false),
            HangulVowel::Yo => AheuiDirection::Up(true),
            HangulVowel::U => AheuiDirection::Down(false),
            HangulVowel::Yu => AheuiDirection::Down(true),
            HangulVowel::Eu => AheuiDirection::ReflectY,
            HangulVowel::I => AheuiDirection::ReflectX,
            HangulVowel::Ui => AheuiDirection::ReflectXY,
            _ => AheuiDirection::Null,
        };

        Self {
            operation,
            direction,
            argument,
            character: cmd,
        }
    }

    // return a null instruction
    pub fn null() -> Self {
        Self {
            operation: AheuiOperation::Null,
            direction: AheuiDirection::Null,
            argument: AheuiArgument::Null,
            character: ' ',
        }
    }
}

impl fmt::Debug for AheuiInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} with arg {:?} and dir {:?}", self.operation, self.argument, self.direction)
    }
}
