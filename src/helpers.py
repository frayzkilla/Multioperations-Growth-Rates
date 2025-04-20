def format_operation(op_string):
    return ', '.join('Ø' if part == '_' else part for part in op_string.split())

def handle_none(value):
    return "Не сущ." if value is None else value