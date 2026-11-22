import argparse
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def arguments(description, openapi=False):
    parser = argparse.ArgumentParser(description=description)
    parser.add_argument('--catalog', type=Path, default=ROOT / 'target/dashboard-catalog.json')
    if openapi:
        parser.add_argument('--openapi', type=Path, default=ROOT / 'target/dashboard-openapi.json')
    return parser.parse_args()


def read_json(path):
    try:
        return json.loads(path.read_text(encoding='utf-8-sig'))
    except (OSError, ValueError) as error:
        raise SystemExit(f'Cannot load JSON input {path}: {error}') from error


def write_rust(outputs):
    rustfmt = shutil.which('rustfmt')
    if rustfmt is None:
        raise SystemExit('rustfmt is required; install it with rustup component add rustfmt')
    formatted = {}
    for path, text in outputs.items():
        result = subprocess.run(
            [rustfmt, '--edition', '2024', '--emit', 'stdout'],
            input=text, encoding='utf-8', capture_output=True,
        )
        if result.returncode:
            raise SystemExit(f'Cannot format {path}:\n{result.stderr}')
        formatted[path] = result.stdout
    for path, text in formatted.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding='utf-8', newline='\n')