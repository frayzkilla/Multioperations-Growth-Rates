import re
import csv

def parse_file(input_file, output_file):
    pattern = re.compile(
        r"✅\s+M\s*=\s*\{(.*?)\},\s+Мощ-ть мин\. ген\. мн-ва:\s+(\d+),\s+Мин\. ген\. мн-во:\s+(.*?)$"
    )

    with open(input_file, "r", encoding="utf-8") as infile, \
         open(output_file, "w", newline="", encoding="utf-8") as outfile:

        writer = csv.writer(outfile, delimiter=";")
        writer.writerow(["M", "cardinality", "g_set"])

        for line in infile:
            line = line.strip()
            if not line:
                continue

            if "✅" in line:
                match = pattern.match(line)
                if match:
                    m_elements = match.group(1).split(";")
                    m_set = "{{" + "}, {".join([elem.strip() for elem in m_elements]) + "}}"
                    
                    cardinality = match.group(2)
                
                    g_set = "{" + ", ".join([elem.strip() for elem in match.group(3).split(",")]) + "}"
                    
                    writer.writerow([m_set, cardinality, g_set])

if __name__ == "__main__":
    parse_file("results/raw/growth_rates_of_sets2_k2_n2_arity2.txt", "results/csv/growth_rates_of_sets2_k2_n2_arity2.csv")