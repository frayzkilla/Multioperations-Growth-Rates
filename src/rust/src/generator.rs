use std::collections::HashSet;
use crate::utils::{combinations, product};

pub fn generate_subset_strings(k: u32) -> Vec<String> {
    let elements: Vec<u32> = (0..k).collect();
    let cap = if k <= 20 { 1 << k } else { 0 };
    let mut subsets = Vec::with_capacity(cap as usize);
    
    subsets.push("_".to_string());
    
    for len in 1..=elements.len() {
        for combo in combinations(&elements, len) {
            let s = combo.iter().map(u32::to_string).collect();
            subsets.push(s);
        }
    }
    
    subsets
}

pub fn generate_combinations(k: u32, m: usize) -> Vec<String> {
    let subsets = generate_subset_strings(k);
    product(&subsets, m)
        .into_iter()
        .map(|combo| combo.join(" "))
        .collect()
}

pub fn generate_sequences(k: usize, n: usize) -> Vec<Vec<Vec<usize>>> {
    let values: Vec<usize> = (0..k).collect();
    let base: Vec<Vec<usize>> = product(&values, n).collect();
    let mut seen = HashSet::new();
    let mut result = Vec::with_capacity(base.len());

    for length in 1..=base.len() {
        for combo in combinations(&base, length) {
            let mut sorted = combo.clone();
            sorted.sort_unstable();
            if seen.insert(sorted) {
                result.push(combo);
            }
        }
    }
    result
}