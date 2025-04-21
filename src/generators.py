from itertools import permutations, product
from itertools import combinations

def generate_subset_strings(k):
    subsets = ['_']
    for l in range(1, k + 1):
        for subset in combinations(range(k), l):
            subsets.append(''.join(map(str, subset)))
    return subsets

def generate_combinations(k, m):
    subsets = generate_subset_strings(k)
    for combo in product(subsets, repeat=m):
        yield ' '.join(combo)

def no_generate_sequences(k, n):
    base = list(product(range(k), repeat=n))
    max_length = k ** n  
    
    for length in range(1, max_length + 1):
        for sequence in product(base, repeat=length):
            print(sequence)
            yield sequence
            
def generate_sequences(k, n):
    base = list(product(range(k), repeat=n))
    seen = set()

    for length in range(1, len(base) + 1):
        for combo in combinations(base, length):
            sorted_combo = tuple(sorted(combo))
            if sorted_combo not in seen:
                seen.add(sorted_combo)
                yield combo  