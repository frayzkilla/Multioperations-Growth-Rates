import pandas as pd
import glob
import os

def merge_csvs_by_k(k: int, folder: str = "results/csv", save_to: str = "results/csv/merged/merged_k2.csv") -> pd.DataFrame:
    pattern = f"growth_rates_of_k{k}_*.csv"
    path_pattern = os.path.join(folder, pattern)
    
    files = glob.glob(path_pattern)
    
    if not files:
        raise FileNotFoundError(f"Не найдено файлов с шаблоном: {pattern}")

    dfs = []
    for file in files:
        try:
            df = pd.read_csv(file, sep=';')
            dfs.append(df)
        except Exception as e:
            print(f"Ошибка при чтении {file}: {e}")

    merged_df = pd.concat(dfs, ignore_index=True)
    
    if save_to:
        merged_df.to_csv(save_to, index=False, sep=';')
        print(f"Сохранено в {save_to}")

    return merged_df

if __name__ == "__main__":
    k = 2
    merged_df = merge_csvs_by_k(k)
    print(merged_df.head())