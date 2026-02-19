use crate::phonetic::PhoneticIndex;
use crate::trie::Trie;
use crate::typo_model::TypoModel;
use std::collections::HashMap;

/// Limiar de frequência para "upgrade" de palavra válida.
/// Se a alternativa é N vezes mais frequente, corrige para ela.
const FREQ_UPGRADE_RATIO: u32 = 15;

pub struct StageA {
    trie: Trie,
    typo_model: TypoModel,
    phonetic_index: PhoneticIndex,
    /// Mapa de frequência: palavra → score (maior = mais comum)
    frequency: HashMap<String, u32>,
}

impl StageA {
    pub fn new() -> Self {
        Self {
            trie: Trie::new(),
            typo_model: TypoModel::new(),
            phonetic_index: PhoneticIndex::new(),
            frequency: HashMap::new(),
        }
    }

    /// Carrega dados de frequência.
    pub fn load_frequency_data(&mut self, entries: &[(String, u32)]) {
        for (word, freq) in entries {
            let lower = word.to_lowercase();
            self.frequency.insert(lower.clone(), *freq);
            self.trie.insert_with_frequency(&lower, *freq);
        }
    }

    fn get_frequency(&self, word: &str) -> u32 {
        self.frequency.get(word).copied().unwrap_or(0)
    }

    /// Pipeline de correção completo.
    pub fn correct(&self, word: &str, aggressiveness: u32) -> String {
        if word.is_empty() {
            return word.to_string();
        }

        let first_char_upper = word
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false);
        let all_upper = word.chars().all(|c| c.is_uppercase() || !c.is_alphabetic());
        let word_lower = word.to_lowercase();

        // 1. TypoModel PRIMEIRO — pares explícitos de confusão (par→para, etc.)
        //    Checa ANTES do dicionário para capturar palavras válidas-mas-erradas
        if let Some(correction) = self.typo_model.get_correction(&word_lower) {
            if correction != word_lower {
                return Self::restore_case(&correction, first_char_upper, all_upper);
            }
        }

        // 2. Palavra no dicionário? Verificar se faz "upgrade" de frequência
        if self.trie.contains(&word_lower) {
            // Tentar upgrade: se existe palavra MUITO mais comum à distância 1
            if word_lower.len() >= 2 && word_lower.len() <= 6 {
                if let Some(upgrade) = self.try_frequency_upgrade(&word_lower) {
                    return Self::restore_case(&upgrade, first_char_upper, all_upper);
                }
            }
            return word.to_string(); // Palavra está OK
        }

        // 3. Detecção de TRANSPOSIÇÃO (teh→the, tabalho→trabalho)
        if let Some(transposed) = self.try_transpositions(&word_lower) {
            return Self::restore_case(&transposed, first_char_upper, all_upper);
        }

        // 4. Busca Fonética (S/SS/Ç/Z/SC, X/CH, G/J)
        let phonetic_matches = self.phonetic_index.find_matches(&word_lower);
        if !phonetic_matches.is_empty() {
            let best = phonetic_matches
                .iter()
                .max_by_key(|w| self.get_frequency(w))
                .unwrap();
            return Self::restore_case(best, first_char_upper, all_upper);
        }

        // 5. Busca Fuzzy (Distância 1)
        if word_lower.len() >= 3 {
            let suggestions = self.trie.get_suggestions(&word_lower, 1);
            let best = if word_lower.len() <= 3 {
                suggestions.into_iter().find(|(_, _, f)| *f > 40000)
            } else {
                suggestions.into_iter().next()
            };
            if let Some((best_word, _, _)) = best {
                return Self::restore_case(&best_word, first_char_upper, all_upper);
            }
        }

        // 6. Busca Fuzzy (Distância 2) — modo agressivo
        if aggressiveness > 0 && word_lower.len() >= 4 {
            let suggestions = self.trie.get_suggestions(&word_lower, 2);
            let best = suggestions
                .into_iter()
                .filter(|(candidate, _, freq)| {
                    *freq > 0
                        && (candidate.len() as i32 - word_lower.len() as i32).unsigned_abs() <= 2
                })
                .next();
            if let Some((best_word, _, _)) = best {
                return Self::restore_case(&best_word, first_char_upper, all_upper);
            }
        }

        // Nenhuma correção encontrada
        word.to_string()
    }

    /// Tenta "upgrade de frequência": se existe uma palavra
    /// MUITO mais comum (>15x) à distância de edição 1,
    /// corrige para ela. Exemplo: "par"(rara) → "para"(muito comum)
    fn try_frequency_upgrade(&self, word: &str) -> Option<String> {
        let my_freq = self.get_frequency(word);
        if my_freq == 0 {
            return None; // Sem dados de frequência, não fazer upgrade
        }

        let suggestions = self.trie.get_suggestions(word, 1);

        for (candidate, _dist, cand_freq) in &suggestions {
            if candidate == word {
                continue;
            }
            // Candidato deve ser significativamente mais frequente
            if *cand_freq > my_freq.saturating_mul(FREQ_UPGRADE_RATIO) {
                return Some(candidate.clone());
            }
        }

        None
    }

    /// Tenta encontrar palavra válida trocando pares de letras adjacentes.
    fn try_transpositions(&self, word: &str) -> Option<String> {
        let chars: Vec<char> = word.chars().collect();
        let mut best: Option<(String, u32)> = None;

        for i in 0..chars.len().saturating_sub(1) {
            let mut swapped = chars.clone();
            swapped.swap(i, i + 1);
            let candidate: String = swapped.into_iter().collect();

            if self.trie.contains(&candidate) {
                let freq = self.get_frequency(&candidate);
                if best.as_ref().map_or(true, |(_, f)| freq > *f) {
                    best = Some((candidate, freq));
                }
            }
        }

        best.map(|(word, _)| word)
    }

    fn restore_case(corrected: &str, first_upper: bool, all_upper: bool) -> String {
        if all_upper && corrected.chars().count() > 1 {
            corrected.to_uppercase()
        } else if first_upper {
            let mut chars = corrected.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    upper + chars.as_str()
                }
            }
        } else {
            corrected.to_string()
        }
    }

    pub fn load_dictionary(&mut self, words: &[&str]) {
        for word in words {
            let lower = word.to_lowercase();
            let freq = self.get_frequency(&lower);
            self.trie.insert_with_frequency(&lower, freq);
            self.phonetic_index.insert(&lower);
        }
    }

    pub fn load_dictionary_strings(&mut self, words: &[String]) {
        for word in words {
            let lower = word.to_lowercase();
            let freq = self.get_frequency(&lower);
            self.trie.insert_with_frequency(&lower, freq);
            self.phonetic_index.insert(&lower);
        }
    }
}
