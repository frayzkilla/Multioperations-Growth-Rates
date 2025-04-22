import csv
import math

def count_multiops_sets_combinations(k: int, n: int, r: int, q: int) -> int:
    total_vectors = 2 ** (k * n * (k ** r))
    if q > total_vectors:
        return 0
    return math.comb(total_vectors, q)

def append_structure_stats(csv_path: str, k: int, n: int, r: int, q: int):
    with open(csv_path, newline='', encoding='utf-8') as csvfile:
        reader = csv.reader(csvfile)
        rows = list(reader)
        record_count = len(rows) - 1 if rows and any(rows[0]) else len(rows)

    denominator = count_multiops_sets_combinations(k, n, r, q)
    result = record_count / denominator if denominator != 0 else 0
    result_str = "{:.10f}".format(result)

    new_row = [k, n, r, q, record_count, result_str]

    new_csv_path = "results/csv/merged/stats.csv"

    with open(new_csv_path, 'a', newline='', encoding='utf-8') as outfile:
        writer = csv.writer(outfile)
        writer.writerow(new_row)

    print(f"Новая строка добавлена в файл: {new_csv_path}")
    
append_structure_stats("results/csv/growth_rates_of_sets3_k2_n2_arity1.csv", 2, 2, 1, 3)