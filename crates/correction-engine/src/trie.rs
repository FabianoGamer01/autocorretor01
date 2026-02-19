use std::collections::HashMap;

#[derive(Default)]
pub struct TrieNode {
    pub is_end_of_word: bool,
    pub frequency: u32, // 0 = sem frequência conhecida, maior = mais comum
    pub children: HashMap<char, TrieNode>,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        self.insert_with_frequency(word, 0);
    }

    pub fn insert_with_frequency(&mut self, word: &str, frequency: u32) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::default);
        }
        node.is_end_of_word = true;
        if frequency > node.frequency {
            node.frequency = frequency;
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(n) = node.children.get(&c) {
                node = n;
            } else {
                return false;
            }
        }
        node.is_end_of_word
    }

    /// Retorna sugestões de palavras com distância de Levenshtein <= max_distance.
    /// Ordenadas por: (distância crescente, frequência decrescente).
    /// Isso garante que palavras COMUNS sejam preferidas quando há empate de distância.
    pub fn get_suggestions(&self, word: &str, max_distance: usize) -> Vec<(String, usize, u32)> {
        let mut suggestions = Vec::new();
        let word_chars: Vec<char> = word.chars().collect();
        let current_row: Vec<usize> = (0..=word_chars.len()).collect();

        for (&c, child) in &self.root.children {
            self.search_recursive(
                child,
                c,
                &word_chars,
                &current_row,
                &mut String::new(),
                max_distance,
                &mut suggestions,
            );
        }

        // Ordenar por: distância crescente, depois frequência decrescente
        suggestions.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| b.2.cmp(&a.2)));
        suggestions
    }

    fn search_recursive(
        &self,
        node: &TrieNode,
        letter: char,
        word_chars: &[char],
        previous_row: &[usize],
        current_word: &mut String,
        max_distance: usize,
        suggestions: &mut Vec<(String, usize, u32)>,
    ) {
        current_word.push(letter);
        let columns = word_chars.len() + 1;
        let mut current_row = vec![0usize; columns];
        current_row[0] = previous_row[0] + 1;

        for i in 1..columns {
            let insert_cost = current_row[i - 1] + 1;
            let delete_cost = previous_row[i] + 1;
            let replace_cost = if word_chars[i - 1] == letter {
                previous_row[i - 1]
            } else {
                previous_row[i - 1] + 1
            };

            current_row[i] = insert_cost.min(delete_cost).min(replace_cost);
        }

        if current_row[columns - 1] <= max_distance && node.is_end_of_word {
            suggestions.push((
                current_word.clone(),
                current_row[columns - 1],
                node.frequency,
            ));
        }

        // Poda: só continua se ainda há chance de encontrar uma palavra dentro do limite
        if *current_row.iter().min().unwrap() <= max_distance {
            for (&c, child) in &node.children {
                self.search_recursive(
                    child,
                    c,
                    word_chars,
                    &current_row,
                    current_word,
                    max_distance,
                    suggestions,
                );
            }
        }

        current_word.pop();
    }
}
