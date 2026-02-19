use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Carrega palavras de um arquivo (uma por linha).
pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let word = line.trim();
        if !word.is_empty() {
            words.push(word.to_string());
        }
    }

    Ok(words)
}

/// Carrega arquivo de frequência (formato: "palavra contagem" por linha).
/// Retorna vetor de (palavra, frequência_normalizada).
/// A frequência é normalizada para um rank: posição 1 = mais comum.
pub fn load_frequency_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<(String, u32)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    let mut rank: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Formato: "palavra contagem" (separados por espaço)
        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        if let Some(&word) = parts.first() {
            if word.len() >= 2 {
                rank += 1;
                // Inverter rank para que palavras mais comuns tenham valor MAIOR
                // Max rank = 50000, então freq = 50001 - rank
                let freq = 50001u32.saturating_sub(rank);
                entries.push((word.to_lowercase(), freq));
            }
        }
    }

    Ok(entries)
}
