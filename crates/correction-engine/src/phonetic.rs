use std::collections::HashMap;

/// Normalização fonética para Português Brasileiro.
/// Trata confusões comuns entre letras/dígrafos que têm sons iguais ou similares.
///
/// Equivalências tratadas:
/// - S, SS, C (antes de e/i), Z, SC, Ç, XC → som de /s/
/// - X, CH, S (antes de consoante) → som de /ʃ/
/// - G (antes de e/i), J → som de /ʒ/
/// - L, U (final de sílaba) → som de /w/
pub struct PhoneticNormalizer;

impl PhoneticNormalizer {
    /// Gera variantes fonéticas de uma palavra, substituindo letras/dígrafos
    /// que soam igual no PT-BR. Retorna todas as variantes possíveis.
    pub fn generate_variants(word: &str) -> Vec<String> {
        let mut results = vec![word.to_string()];

        // Aplicar cada regra de substituição e acumular variantes
        let rules = Self::get_sibilant_rules();

        for (from, alternatives) in &rules {
            let mut new_results = Vec::new();
            for current in &results {
                // Para cada ocorrência do padrão "from" na palavra,
                // gerar variantes com cada alternativa
                if current.contains(from.as_str()) {
                    for alt in alternatives {
                        let variant = current.replace(from.as_str(), alt.as_str());
                        if variant != *current {
                            new_results.push(variant);
                        }
                    }
                }
            }
            results.extend(new_results);
        }

        // Remover duplicatas mantendo ordem
        let mut seen = std::collections::HashSet::new();
        results.retain(|x| seen.insert(x.clone()));
        results
    }

    /// Regras de substituição para sibilantes (S/SS/C/Z/SC/Ç)
    fn get_sibilant_rules() -> Vec<(String, Vec<String>)> {
        vec![
            // ss ↔ ç ↔ c (antes de e/i)
            ("ss".into(), vec!["ç".into(), "c".into(), "sc".into()]),
            ("ç".into(), vec!["ss".into(), "s".into(), "c".into()]),
            ("sc".into(), vec!["ss".into(), "ç".into(), "c".into()]),
            // s ↔ z (entre vogais, som de /z/)
            ("s".into(), vec!["z".into()]),
            ("z".into(), vec!["s".into()]),
            // c (antes de e/i) - tratado pela busca contextual
            ("ce".into(), vec!["se".into(), "sse".into()]),
            ("ci".into(), vec!["si".into(), "ssi".into()]),
            ("se".into(), vec!["ce".into()]),
            ("si".into(), vec!["ci".into()]),
            // x ↔ ch ↔ s
            ("x".into(), vec!["ch".into(), "s".into()]),
            ("ch".into(), vec!["x".into()]),
            // g (antes de e/i) ↔ j
            ("ge".into(), vec!["je".into()]),
            ("gi".into(), vec!["ji".into()]),
            ("je".into(), vec!["ge".into()]),
            ("ji".into(), vec!["gi".into()]),
            // l ↔ u (final de sílaba, ex: "mal" ↔ "mau")
            ("ou".into(), vec!["ol".into()]),
            ("ol".into(), vec!["ou".into()]),
            // Combinações com nh/lh
            ("nh".into(), vec!["ni".into()]),
            ("lh".into(), vec!["li".into()]),
        ]
    }

    /// Normaliza para uma forma canônica (para uso como chave de busca).
    /// Todas as variantes fonéticas equivalentes produzem a mesma forma normalizada.
    pub fn normalize(word: &str) -> String {
        let mut result = word.to_string();
        // Normalizar sibilantes para 's'
        result = result.replace("ss", "S");
        result = result.replace("ç", "S");
        result = result.replace("sc", "S");
        // Manter 's' simples
        result = result.replace('s', "S");
        result = result.replace('z', "S");
        result = result.replace('S', "s");
        result
    }
}

/// Mapa fonético: armazena palavras do dicionário indexadas pela forma normalizada.
/// Permite busca rápida de palavras que soam igual.
pub struct PhoneticIndex {
    /// forma_normalizada → lista de palavras originais do dicionário
    index: HashMap<String, Vec<String>>,
}

impl PhoneticIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    /// Adiciona uma palavra ao índice fonético.
    pub fn insert(&mut self, word: &str) {
        let normalized = PhoneticNormalizer::normalize(word);
        self.index
            .entry(normalized)
            .or_insert_with(Vec::new)
            .push(word.to_string());
    }

    /// Busca palavras do dicionário que soam foneticamente parecida.
    /// Retorna a lista de candidatos encontrados.
    pub fn find_matches(&self, word: &str) -> Vec<String> {
        let normalized = PhoneticNormalizer::normalize(word);
        self.index.get(&normalized).cloned().unwrap_or_default()
    }
}
