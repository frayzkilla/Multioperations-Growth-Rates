class InitialParser:
    def __init__(self, k: int):
        self.k = k
        self.allowed_chars = set(str(i) for i in range(k)) | {'_'}

    def _parse_element(self, element_str: str) -> frozenset[int]:
        if element_str == '_':
            return frozenset()
        
        for c in element_str:
            if c not in self.allowed_chars:
                raise ValueError(f"Недопустимый символ '{c}' в элементе '{element_str}'")
        
        elements = {int(c) for c in element_str}
        return frozenset(elements)

    def parse_set(self, input_str: str) -> set[tuple[frozenset[int], ...]]:
        result = set()
        
        parts = [p.strip() for p in input_str.split(',') if p.strip()]
        
        for part in parts:
            elements = [e.strip() for e in part.split() if e.strip()]
            
            parsed_elements = tuple(self._parse_element(e) for e in elements)
            
            result.add(parsed_elements)
            
        return result

    def parse_operation(self, input_str: str) -> list[set[int]]:
        elements = [e.strip() for e in input_str.split() if e.strip()]
        return [set(self._parse_element(e)) for e in elements]
