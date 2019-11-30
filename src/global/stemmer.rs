extern crate utils;

use utils::substring;

pub struct PortarsStemmer<'a> {
    vowels: Vec<char>,
    perfective_gerund: [Vec<&'a str>; 2],
    adjective: [Vec<&'a str>; 2],
    participle: [Vec<&'a str>; 2],
    reflexive: [Vec<&'a str>; 2],
    verb: [Vec<&'a str>; 2],
    noun: [Vec<&'a str>; 2],
    superlative: [Vec<&'a str>; 2],
    derivational: [Vec<&'a str>; 2],
    adjectival: [Vec<&'a str>; 2],
    suffix: [Vec<&'a str>; 2],

    ends: Vec<[Vec<&'a str>; 2]>,
}

impl<'a> PortarsStemmer<'a> {
    pub fn new() -> PortarsStemmer<'a> {
        let mut stemmer = PortarsStemmer {
            vowels: vec!['а', 'е', 'и', 'о', 'у', 'ы', 'э', 'ю', 'я', 'ё'],
            perfective_gerund: [
                vec!["вшись", "вши", "в"],
                vec![
                    "ывшись",
                    "ившись",
                    "ывши",
                    "ивши",
                    "ыв",
                    "ив",
                ],
            ],
            adjective: [
                vec![],
                vec![
                    "ими", "ыми", "его", "ого", "ему", "ому", "ее", "ие",
                    "ые", "ое", "ей", "ий", "ый", "ой", "ем", "им", "ым", "ом",
                    "их", "ых", "ую", "юю", "ая", "яя", "ою", "ею",
                ],
            ],
            participle: [
                vec!["ем", "нн", "вш", "ющ", "щ"],
                vec!["ивш", "ывш", "ующ"],
            ],
            reflexive: [vec![], vec!["ся", "сь"]],
            verb: [
                vec![
                    "ешь", "нно", "ете", "йте", "ла", "на", "ли", "ем", "ло",
                    "но", "ет", "ют", "ны", "ть", "й", "л", "н",
                ],
                vec![
                    "ейте", "уйте", "ила", "ыла", "ена", "ите", "или",
                    "ыли", "ило", "ыло", "ено", "ует", "уют", "ены", "ить",
                    "ыть", "ишь", "ей", "уй", "ил", "ыл", "им", "ым", "ен",
                    "ят", "ит", "ыт", "ую", "ю",
                ],
            ],
            noun: [
                vec![],
                vec![
                    "иями", "ями", "ами", "ией", "иям", "ием", "иях", "ев",
                    "ов", "ие", "ье", "еи", "ии", "ей", "ой", "ий", "ям", "ем",
                    "ам", "ом", "ия", "ья", "ах", "ях", "ию", "ью", "ю", "о",
                    "у", "а", "е", "и", "я", "ы", "ь", "й",
                ],
            ],
            superlative: [vec![], vec!["ейше", "ейш"]],
            derivational: [vec![], vec!["ость", "ост"]],
            adjectival: [vec![], vec![]],
            suffix: [
                vec![],
                vec!["ическ", "чик", "ок", "ик", "ек", "к"],
            ],
            ends: vec![],
        };

        stemmer.ends = vec![
            stemmer.perfective_gerund.clone(),
            stemmer.adjective.clone(),
            stemmer.verb.clone(),
            stemmer.noun.clone(),
        ];

        stemmer
    }
}

pub fn clip(word: &str, stemmer: &PortarsStemmer) -> String {

    //Выделяем области
    let mut RV_pos = 0;
    let mut R1_pos = 0;
    let mut R2_pos = 0;

    let mut prev_vowel = false;

    // println!("Определение областей");

    for (index, ch) in word.chars().enumerate() {
        // println!("Индекс: {3}, RV: {0}, R1: {1}, R2: {2}, ch: {4}",
        //     RV_pos.to_string(), R1_pos.to_string(), R2_pos.to_string(), index.to_string(), ch);

        if RV_pos == 0 {
            if stemmer.vowels.contains(&ch) {
                RV_pos = index/2 + 1;
                prev_vowel = true;
            }
        } else if R1_pos == 0 {
            if prev_vowel {
                if !stemmer.vowels.contains(&ch) && ch.is_alphabetic() {
                    R1_pos = index/2 + 1;
                }

                prev_vowel = false;
            } else {
                if stemmer.vowels.contains(&ch) {
                    prev_vowel = true;
                }
            }
        } else if R2_pos == 0 {
            if prev_vowel {
                if !stemmer.vowels.contains(&ch) && ch.is_alphabetic() {
                    R2_pos = index/2 + 1;
                }

                prev_vowel = false;
            } else {
                if stemmer.vowels.contains(&ch) {
                    prev_vowel = true;
                }
            }
        } else {
            break;
        }
    }
    // println!("Области - RV: {0}, R1: {1}, R2: {2}",
    //         RV_pos.to_string(), R1_pos.to_string(), R2_pos.to_string());
    //Области определены

    if RV_pos > word.chars().count() || word.chars().count() == 1 {
        return String::new();
    }

    let mut word_end_pos = word.chars().count();

    let mut RV = substring(word, RV_pos, None);

    // println!("RV: {}, end pos: {}", RV,word_end_pos.to_string());

    let mut end_found = false;

    //шаг 1
    for ends in &stemmer.ends {
        for end in &ends[0] {
            if RV.len() > end.len() {
                if RV.ends_with(end) && RV.chars().count() > end.chars().count() {
                    // println!("совпало окончание {} первой группы",end);
                    let pos = RV.chars().count() - 1 - end.chars().count();
                    // let prev_ch = RV.get((pos - 1) * 2..pos * 2).unwrap();
                    let prev_ch:char = RV.chars().nth(pos).unwrap();
                    
                    // println!("совпало окончание {1} первой группы, символ перед ним: {0}", 
                    //             prev_ch, end);

                    if prev_ch == 'а' || prev_ch == 'я' {
                        word_end_pos = word_end_pos - end.chars().count();
                        end_found = true;
                        break;
                    }
                }
            }
        }

        if end_found {
            break;
        }

        for end in &ends[1] {
            if RV.len() > end.len() {
                if RV.ends_with(end) {
                    // println!("совпало окончание {} второй группы", end);
                    word_end_pos = word_end_pos - end.chars().count();
                    end_found = true;
                    break;
                }
            }
        }

        if end_found {
            break;
        }
    }

    // let rv_2 = substring(word, RV_pos, Some(word_end_pos-RV_pos));
    // println!("Слово: {0}, начало: {1}, длина: {2}", word, RV_pos.to_string(),word_end_pos.to_string());

    RV = substring(word, RV_pos, Some(word_end_pos-RV_pos));
    
    // println!("Новая end pos: {0}, новая RV: {1}", word_end_pos.to_string(),rv_2);

    //Шаг 2
    if RV.ends_with('и') {
        word_end_pos = word_end_pos - 1;
        RV = substring(word, RV_pos, Some(word_end_pos-RV_pos));
    }

    //Шаг 3
    if R2_pos < word_end_pos && R2_pos != 0 {
        let R2 = substring(word, R2_pos, Some(word_end_pos-R2_pos));

        for der_end in &stemmer.derivational[1] {
            if R2.ends_with(der_end) {
                // println!("word_end_pos: {}, der_end_count: {}, RV_pos: {}, R2: {}",
                //         word_end_pos.to_string(), der_end.chars().count().to_string(),RV_pos.to_string(), R2);
                word_end_pos = word_end_pos - der_end.chars().count();
                RV = substring(word, RV_pos, Some(word_end_pos-(RV_pos)));
            }
        }
    }

    //Шаг 4

    let mut step_four_complete = false;
    if RV.ends_with("нн") {
        word_end_pos = word_end_pos - 1;
        RV = substring(word, RV_pos, Some(word_end_pos-RV_pos));
        step_four_complete = true;
    } else {
        for sup_end in &stemmer.superlative[1] {
            if RV.ends_with(sup_end) {
                word_end_pos = word_end_pos - sup_end.chars().count();
                RV = substring(word, RV_pos, Some(word_end_pos-RV_pos));
                step_four_complete = true;
                break;
            }
        }

        if step_four_complete {
            if RV.ends_with("нн") {
                word_end_pos = word_end_pos - 1;
                RV = substring(word, RV_pos, Some(word_end_pos-RV_pos));
            }
        }
    }

    if !step_four_complete {
        if RV.ends_with("ь") {
            word_end_pos = word_end_pos - 1;
            RV = substring(word, RV_pos, Some(word_end_pos-RV_pos));
            step_four_complete = true;
        }
    }

    if !step_four_complete {
        for suff in &stemmer.suffix[1] {
            if RV.ends_with(suff) && RV.chars().count() > suff.chars().count() {
                let pos = RV.chars().count() - 1 - suff.chars().count();
                let prev_ch:char = RV.chars().nth(pos).unwrap();

                if !stemmer.vowels.contains(&prev_ch) {
                    word_end_pos = word_end_pos - suff.chars().count();
                    break;
                }
            }
        }
    }

    substring(word, 0, Some(word_end_pos))
}
