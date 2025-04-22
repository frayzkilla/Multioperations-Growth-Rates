use crate::utils::product;
use std::collections::{BTreeSet, HashMap, HashSet};
use itertools::Itertools;
use crate::generator::{generate_combinations, generate_sequences};


fn determine_arities(k: usize, operations: Vec<Vec<HashSet<u32>>>) -> Vec<u32> {
    let mut arities = Vec::new();
    for op in operations {
        let mut m = 0;
        let mut size = 1;
        while size < op.len() {
            size *= k;
            m += 1;
        }
        if size != op.len() {
            panic!("ValueError: Invalid operation length: {}", op.len());
        }
        arities.push(m as u32);
    }
    arities
}

fn create_multi_operation_tables(
    k: u32,
    arities: Vec<u32>,
    operations: Vec<Vec<HashSet<u32>>>,
) -> Vec<HashMap<Vec<u32>, BTreeSet<u32>>> {
    arities.into_iter()
        .zip(operations.into_iter())
        .map(|(m, op)| {
            op.into_iter().enumerate()
                .map(|(num, results)| {
                    let key = (0..m)
                        .map(|i| (num as u32 / k.pow(m - i - 1) % k))
                        .collect();
                    (key, results.into_iter().collect())
                })
                .collect()
        })
        .collect()
}

fn cartesian_product(sets: &[&BTreeSet<u32>]) -> Vec<Vec<u32>> {
    let mut result = vec![vec![]];
    for set in sets {
        result = result.into_iter()
            .flat_map(|prefix| set.iter().map(move |x| {
                let mut new_prefix = prefix.clone();
                new_prefix.push(*x);
                new_prefix
            }))
            .collect();
    }
    result
}

fn compute_derived_sets(
    multi_op_tables: Vec<HashMap<Vec<u32>, BTreeSet<u32>>>,
    arities: Vec<u32>,
    current_sets: HashSet<Vec<BTreeSet<u32>>>,
) -> HashSet<Vec<BTreeSet<u32>>> {
    let mut new_sets = HashSet::new();
    let current_sets_vec: Vec<&Vec<BTreeSet<u32>>> = current_sets.iter().collect();

    multi_op_tables.into_iter()
        .zip(arities.into_iter())
        .for_each(|(op_table, m)| {
            product(&current_sets_vec, m as usize)
                .into_iter()
                .for_each(|args_combo| {
                    let n = args_combo.first().map_or(0, |v| v.len());
                    let mut new_tuple = Vec::with_capacity(n);

                    for pos in 0..n {
                        let elements: Vec<&BTreeSet<u32>> = args_combo.iter()
                            .map(|arg| &arg[pos])
                            .collect();

                        if elements.iter().any(|e| e.is_empty()) {
                            new_tuple.push(BTreeSet::new());
                            continue;
                        }

                        let product_args = cartesian_product(&elements);
                        let result_set: BTreeSet<_> = product_args.into_iter()
                            .filter_map(|combo| op_table.get(&combo))
                            .flatten()
                            .cloned()
                            .collect();

                        new_tuple.push(if result_set.is_empty() {
                            BTreeSet::new()
                        } else {
                            result_set
                        });
                    }

                    new_sets.insert(new_tuple);
                });
        });

    new_sets
}

fn find_derivable_sets(
    initial_sets: HashSet<Vec<BTreeSet<u32>>>,
    k: usize,
    operations: Vec<Vec<HashSet<u32>>>,
) -> Vec<String> {
    let mut current = initial_sets;
    let arities = determine_arities(k, operations.clone());
    let op_tables = create_multi_operation_tables(k as u32, arities.clone(), operations);

    loop {
        let new_sets = compute_derived_sets(op_tables.clone(), arities.clone(), current.clone());
        if new_sets.is_subset(&current) {
            break;
        }
        current.extend(new_sets);
    }

    let mut result: Vec<String> = current
        .iter()
        .map(|s| {
            s.iter()
                .map(|elem| match elem.len() {
                    0 => "∅".to_string(),
                    1 => elem.iter().next().unwrap().to_string(),
                    _ => {
                        let mut sorted: Vec<_> = elem.iter().collect();
                        sorted.sort_unstable();
                        format!("{{{}}}", sorted.iter().join(","))
                    }
                })
                .collect::<String>()
        })
        .collect();

    result.sort_unstable();
    result.dedup();
    result
}




#[allow(dead_code)]
pub struct InitialParser {
    k: u32,
    allowed_chars: HashSet<char>,
}

impl InitialParser {
    pub fn new(k: u32) -> Self {
        let allowed_chars: HashSet<char> = (0..k)
            .filter_map(|i| char::from_digit(i, 10))
            .chain(std::iter::once('_'))
            .collect();
        InitialParser { k, allowed_chars }
    }

    pub fn parse_element(&self, element_str: &str) -> Result<BTreeSet<u32>, String> {
        if element_str == "_" {
            return Ok(BTreeSet::new());
        }

        for c in element_str.chars() {
            if !self.allowed_chars.contains(&c) {
                return Err(format!("Invalid character '{}' in element '{}'", c, element_str));
            }
        }

        let elements: BTreeSet<u32> = element_str
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        Ok(elements)
    }

    pub fn parse_set(&self, input_str: &str) -> Result<HashSet<Vec<BTreeSet<u32>>>, String> {
        input_str.split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|part| {
                part.split_whitespace()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|e| self.parse_element(e))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect()
    }

    pub fn parse_operation(&self, input_str: &str) -> Result<Vec<HashSet<u32>>, String> {
        input_str
            .split_whitespace()
            .map(|e| {
                if e.trim().is_empty() {
                    Ok(HashSet::new())
                } else {
                    self.parse_element(e).map(|s| s.into_iter().collect())
                }
            })
            .collect()
    }
}

pub fn find_growth_rate(
    k: usize,
    n: usize,
    target_length: usize,
    operations: Vec<Vec<HashSet<u32>>>,
) -> (Option<usize>, Option<String>) {
    let parser = InitialParser::new(k as u32);

    for seq in generate_sequences(k, n) {
        let initial_seq = seq
            .iter()
            .map(|item| {
                item.iter()
                    .map(|num| num.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join(", ");

        let parsed_set = match parser.parse_set(&initial_seq) {
            Ok(set) => set,
            Err(_) => continue,
        };

        let initial_sets: HashSet<Vec<BTreeSet<u32>>> = parsed_set.clone()
            .into_iter()
            .map(|vec_set| {
                vec_set
                    .into_iter()
                    .map(|set| set.into_iter().collect())
                    .collect()
            })
            .collect();

        let result = find_derivable_sets(initial_sets, k, operations.clone());

        if result.len() == target_length {
            return (Some(parsed_set.len()), Some(initial_seq));
        }
    }

    (None, None)
}





fn format_operation(op_string: &str) -> String {
    op_string
        .split_whitespace()
        .map(|part| if part == "_" { "Ø" } else { part })
        .collect::<Vec<_>>()
        .join(", ")
}
pub fn batch_process_operations(
    k: usize,
    n: usize,
    target_length: usize,
    operations_length: usize,
    filename: String,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::OpenOptions;
    use std::io::{BufWriter, Write};
    use std::time::Instant;

    let start_time = Instant::now();
    let parser = InitialParser::new(k as u32);

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&filename)?;
    let mut writer = BufWriter::with_capacity(128*1024, file);

    for combo in generate_combinations(k as u32, operations_length) {
        let formatted_op = format_operation(&combo);

        let parsed_op = match parser.parse_operation(&combo) {
            Ok(op) => op,
            Err(_) => {
                writeln!(
                    writer,
                    "❌  Вектор мультиоперации: {}, ГЕН. МН-ВО НЕ СУЩ.",
                    formatted_op
                )?;
                continue;
            }
        };

        let (gen_size, gen_set) = find_growth_rate(k, n, target_length, vec![parsed_op]);

        if let (Some(size), Some(set)) = (gen_size, gen_set) {
            let mut cleaned = String::with_capacity(set.len()); 
            for part in set.split(',') {
                let trimmed = part.trim();
                if !trimmed.is_empty() {
                    if !cleaned.is_empty() {
                        cleaned.push_str(", ");
                    }
                    cleaned.push_str(trimmed);
                }
            }

            writeln!(
                writer,
                "✅  Вектор мультиоперации: {}, Мощ-ть мин. ген. мн-ва: {}, Мин. ген. мн-во: {}",
                formatted_op, size, cleaned
            )?;
        } else {
            // writeln!(
            //     writer,
            //     "❌  Вектор мультиоперации: {}, ГЕН. МН-ВО НЕ СУЩ.",
            //     formatted_op
            // )?;
        }
    }

    writer.flush()?; 
    println!("Выполнено за: {:.2} сек", start_time.elapsed().as_secs_f64());
    Ok(())
}


