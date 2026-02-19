pub mod dict_loader;
pub mod phonetic;
pub mod stage_a;
pub mod stage_b;
pub mod stage_c;
pub mod trie;
pub mod typo_model;

#[cfg(test)]
mod validation;

pub fn init() {
    println!("Correction Engine initialized");
}
