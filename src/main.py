from parser import InitialParser
from generators import generate_sequences, generate_subset_strings, generate_combinations
from core import determine_arities, create_multi_operation_tables
from helpers import format_operation, handle_none
from core import find_derivable_sets, find_growth_rate

if __name__ == "__main__":
    
    input_str = "1 0, 0 1"
    op_str = "0 01 _ 1 0 1 01 _"
    k = 2
    n = 3
    target_length = len(generate_subset_strings(k))**n
    arity = 4
    
    parser = InitialParser(k)
    
    for combo in generate_combinations(k, arity):
        parsed_op = parser.parse_operation(combo)
    
        operations_combo = []
        operations_combo.append(parsed_op)
        result = find_growth_rate(k, n, target_length, operations_combo)
        if result[0] is not None:
            output_str = "\n✅  Вектор мультиоперации: " + str(format_operation(combo)) + ", Мощ-ть мин. ген. мн-ва: " + str(result[0]) + ", Мин. ген. мн-во: " + str(result[1])
            with open("results/growth_rates_of_k2_n3_arity2.txt", "a", encoding="utf-8") as f:
                f.write(output_str)
        else:
            output_str = "\n❌  Вектор мультиоперации: " + str(format_operation(combo)) + ", ГЕН. МН-ВО НЕ СУЩ." 
        
        
        print(output_str)

    
    # parsed_set = parser.parse_set(input_str)
    # print(parsed_set)
        
    # formatted = set()
    # for s in parsed_set:
    #     elements = []
    #     for e in s:
    #         if not e:
    #             elements.append('∅')
    #         else:
    #             elements.append(f'{{{",".join(map(str, sorted(e)))}}}')
    #     formatted.add(f'({", ".join(elements)})')
        
    # print("Результат парсинга наборов:")
    # print('\n'.join(sorted(formatted)))
        
    #     # Парсинг операции
    
    # parsed_op = parser.parse_operation(op_str)
    # print("\nРезультат парсинга операции:")
    # print(parsed_op)
    
    # operations = []
    # operations.append(parsed_op)
    # initial = parsed_set.copy()
    
    # result = find_derivable_sets(initial, k, operations)
    # print("\n\nВсе выводимые наборы:", result)
    # print("\nМощность множества выводимых наборов:", len(result))
    
    # for seq in generate_sequences(k, n):
    #     initial_seq = ', '.join([' '.join(map(str, item)) for item in seq])
    #     parsed_seq = parser.parse_set(initial_seq)
    #     result = find_derivable_sets(parsed_seq, k, operations)
    #     if len(result) == target_length:
    #         print("\n\n НАЙДЕНО МИНИМАЛЬНОЕ ГЕН. МН-ВО ДЛЯ ОПЕРАЦИИ \n РАЗМЕР ГЕН. МН-ВА:", len(parsed_seq))
    #         print("\n Генерирующее множество:", initial_seq, "\n\n Все выводимые наборы", result, "\n\n Мощность множества выводимых наборов", len(result))
    #         break
        
    
