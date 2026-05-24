#!/usr/bin/env python3
"""Converte CSV/Parquet para JSON no formato do batch API do FuzzySimulated.
Uso:
  ./scripts/importar_dados.py dados.csv            # CSV (1a linha = cabecalho)
  ./scripts/importar_dados.py dataset.parquet       # Parquet
  ./scripts/importar_dados.py dados.csv --output batch.json  # salva em arquivo
"""
import json, sys, csv
from pathlib import Path

def ler_csv(path):
    with open(path, newline='', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        rows = []
        for row in reader:
            parsed = {}
            for k, v in row.items():
                k = k.strip()
                v = v.strip() if v else ''
                try:
                    if '.' in v or ',' in v:
                        parsed[k] = float(v.replace(',', '.'))
                    else:
                        parsed[k] = int(v) if v else float(v) if v else 0.0
                except ValueError:
                    parsed[k] = float(v) if v else 0.0
            rows.append(parsed)
        return rows

def ler_parquet(path):
    try:
        import pyarrow.parquet as pq
    except ImportError:
        print("Erro: instale pyarrow: pip install pyarrow", file=sys.stderr)
        sys.exit(1)
    table = pq.read_table(path)
    rows = []
    for i in range(len(table)):
        row = {}
        for col in table.column_names:
            val = table[col][i].as_py()
            row[col] = float(val) if val is not None else 0.0
        rows.append(row)
    return rows

path = Path(sys.argv[1])
suf = path.suffix.lower()

if suf == '.csv':
    rows = ler_csv(path)
elif suf == '.parquet':
    rows = ler_parquet(path)
else:
    print(f"Formato nao suportado: {suf} (use .csv ou .parquet)", file=sys.stderr)
    sys.exit(1)

output = sys.argv[3] if len(sys.argv) > 3 and sys.argv[2] == '--output' else None
json_str = json.dumps(rows, indent=2, ensure_ascii=False)

if output:
    Path(output).write_text(json_str, encoding='utf-8')
    print(f"{len(rows)} linhas convertidas -> {output}")
else:
    print(json_str)
