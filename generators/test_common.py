import tempfile
import unittest
from pathlib import Path

from common import read_json, write_rust


class GeneratorTests(unittest.TestCase):
    def test_read_json_accepts_utf8_bom(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'catalog.json'
            path.write_text('{"name": "天氣"}', encoding='utf-8-sig')
            self.assertEqual(read_json(path), {'name': '天氣'})

    def test_missing_and_invalid_json_report_errors(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'catalog.json'
            with self.assertRaises(SystemExit) as missing:
                read_json(path)
            self.assertIn('Cannot load JSON input', str(missing.exception))
            path.write_text('{', encoding='utf-8')
            with self.assertRaises(SystemExit) as invalid:
                read_json(path)
            self.assertIn('Cannot load JSON input', str(invalid.exception))

    def test_write_rust_formats_valid_output(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'nested/model.rs'
            write_rust({path: 'pub struct Model{pub value:Option<String>,}'})
            content = path.read_text(encoding='utf-8')
            self.assertIn('pub struct Model {', content)
            self.assertIn('    pub value: Option<String>,', content)

    def test_format_failure_does_not_overwrite_files(self):
        with tempfile.TemporaryDirectory() as directory:
            first = Path(directory) / 'first.rs'
            second = Path(directory) / 'second.rs'
            first.write_bytes(b'original first\r\n')
            second.write_bytes(b'original second\r\n')
            with self.assertRaises(SystemExit) as error:
                write_rust({first: 'pub struct Valid;', second: 'pub struct {'})
            self.assertIn('Cannot format', str(error.exception))
            self.assertEqual(first.read_bytes(), b'original first\r\n')
            self.assertEqual(second.read_bytes(), b'original second\r\n')


if __name__ == '__main__':
    unittest.main()