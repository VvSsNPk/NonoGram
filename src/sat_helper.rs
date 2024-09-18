use crate::nono::NonoGram;
use std::collections::VecDeque;
use varisat::Lit;
#[derive(Debug)]
pub enum LitStore {
    One(Lit),
    Two(Lit, Lit),
}

impl LitStore {
    pub fn is_negative(&self) -> bool {
        match self {
            LitStore::One(x) => x.is_negative(),
            LitStore::Two(y, z) => (y.is_negative() && z.is_negative()),
        }
    }

    pub fn return_char(&self) -> char {
        match self {
            LitStore::One(x) => 'a',
            LitStore::Two(x, y) => {
                if x.is_positive() {
                    'a'
                } else {
                    'b'
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct AnswerParser {
    nono_gram: NonoGram,
    pub answer: Vec<Vec<LitStore>>,
}

impl AnswerParser {
    pub fn new(nono_gram: NonoGram, answer: Vec<Vec<LitStore>>) -> Self {
        Self { nono_gram, answer }
    }

    pub fn display_solution(&self) -> Vec<String> {
        //assert_eq!(height,self.answer.len(),"Length not same");
        let mut ans = Vec::new();
        for i in 0..self.answer.len() {
            let sol = self.answer.get(i).unwrap();
            let mut printer = String::new();
            for j in sol {
                if j.is_negative() {
                    printer.push('-');
                } else {
                    printer.push(j.return_char());
                }
            }
            ans.push(printer)
        }
        ans
    }
}
