import math
from itertools import product
from parser import InitialParser
from generators import generate_sequences

def determine_arities(k, operations):
    arities = []
    for op in operations:
        m = int(math.log(len(op), k))
        if k**m != len(op):
            raise ValueError(f"Некорректная длина операции: {len(op)}")
        arities.append(m)
        
    return arities

def create_multi_operation_tables(k, arities, operations):
    tables = []
    for m, op in zip(arities, operations):
        table = {
            tuple((num // (k**(m-i-1))) % k for i in range(m)): set(results)
            for num, results in enumerate(op)
        }
        tables.append(table)
        
    return tables

def compute_derived_sets(multi_op_tables, arities, current_sets):
    new_sets = set()
    
    for op_table, m in zip(multi_op_tables, arities):
        for args_combo in product(current_sets, repeat=m):
            n = len(args_combo[0]) if args_combo else 0
            new_tuple = []
            for pos in range(n):
                elements = [arg[pos] for arg in args_combo]
                if any(len(e) == 0 for e in elements):
                    new_element = frozenset()
                else:
                    element_values = [list(e) for e in elements]
                    all_combinations = product(*element_values)
                    result_set = set()
                    for combo in all_combinations:
                        if combo in op_table:
                            result_set.update(op_table[combo])
                    new_element = frozenset(result_set) if result_set else frozenset()
                new_tuple.append(new_element)
            new_set = tuple(new_tuple)
            new_sets.add(new_set)
    
    return new_sets

def find_derivable_sets(initial_sets, k, operations):
    current = initial_sets.copy()
    
    arities = determine_arities(k, operations)
    op_tables = create_multi_operation_tables(k, arities, operations)
    
    while True:
        new_sets = compute_derived_sets(op_tables, arities, current)
        updated = current | new_sets
        
        if len(updated) == len(current):
            break
            
        current = updated
    
    result = []
    for s in current:
        elements_str = []
        for elem in s:
            if len(elem) == 0:
                elements_str.append('∅')
            elif len(elem) == 1:
                elements_str.append(str(next(iter(elem))))
            else:
                sorted_elem = ''.join(sorted(str(x) for x in elem))
                elements_str.append(f'{{{sorted_elem}}}')
        result.append(''.join(elements_str))
    
    return sorted(result)

def find_growth_rate(k, n, target_length, operations):
    parser = InitialParser(k)
    
    for seq in generate_sequences(k, n):
        initial_seq = ', '.join([' '.join(map(str, item)) for item in seq])
        parsed_seq = parser.parse_set(initial_seq)
        result = find_derivable_sets(parsed_seq, k, operations)
        if len(result) == target_length:
            print("\n\n\n", initial_seq)
            return(len(parsed_seq))
    
    return None