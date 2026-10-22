# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Synthetic contract fixtures for the declarations-only module-root gate."""

import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

import check_mod_declarations as checker


class DeclarationTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        self.module = self.root / "mod.rs"

    def findings(self, source):
        self.module.write_text(source, encoding="utf-8")
        return checker.violations(self.module)

    def test_allowed_declarations(self):
        fixtures = [
            "//! Module docs.\nmod child;\npub mod exported;\n",
            "/* outer /* nested */\n*/\nmod child;\n",
            '/* docs */ pub /* visibility */ use crate::Thing; /* trailing */\n',
            '/* docs\n*/ pub use crate::Thing;\n',
            '#[doc = "{ [ /*"]\npub use crate::Thing;\n',
            '#[doc = r##"{ [ /* \\"#"##]\npub use crate::Thing;\n',
            '#[cfg_attr(feature = "{", doc = "[}")]\nmod child;\n',
            '#![doc = "}]"]\nmod child;\n',
            'pub(in crate::owner) use crate::Thing;\npub(super) mod child;\n',
            'pub use crate::{\n    First, // } ]\n    Second, /* { [ */\n};\n',
            'pub use crate::Thing; mod child;\n',
            'mod\nchild\n;\n',
            'mod r#type;\npub use crate::日本語;\n',
        ]
        for source in fixtures:
            with self.subTest(source=source):
                self.assertEqual(self.findings(source), [])

    def test_forbidden_items_report_once(self):
        fixtures = [
            ('const BAD: u8 = 1;', checker.VIOLATION_REASON),
            ('static BAD: u8 = 1;', checker.VIOLATION_REASON),
            ('struct Bad { field: u8 }', checker.VIOLATION_REASON),
            ('struct Bad(u8);', checker.VIOLATION_REASON),
            ('struct Bad<const N: usize = { 1 }>;', checker.VIOLATION_REASON),
            ('enum Bad { First, Second }', checker.VIOLATION_REASON),
            ('impl Bad { fn method() {} }', checker.VIOLATION_REASON),
            ('fn bad() { let value = [1, 2]; }', checker.VIOLATION_REASON),
            ('const fn bad() { let value = [1, 2]; }', checker.VIOLATION_REASON),
            ('type Bad = u8;', checker.VIOLATION_REASON),
            ('macro_rules! bad { () => {}; }', checker.VIOLATION_REASON),
            ('include!("implementation.rs");', checker.VIOLATION_REASON),
            ('use crate::{First, Second};', checker.PRIVATE_IMPORT_REASON),
            ('mod bad { fn body() {} }', checker.INLINE_MODULE_REASON),
            ('pub mod bad\n{ fn body() {} }', checker.INLINE_MODULE_REASON),
        ]
        for source, reason in fixtures:
            with self.subTest(source=source):
                findings = self.findings(source + '\n')
                self.assertEqual(len(findings), 1)
                self.assertEqual((findings[0][0], findings[0][2]), (1, reason))

    def test_comments_and_attributes_cannot_hide_definitions(self):
        fixtures = [
            ('const BAD: u8 = 1; /* explanation */\n', 1),
            ('/* explanation */ const BAD: u8 = 1;\n', 1),
            ('/* explanation\n*/ const BAD: u8 = 1;\n', 2),
            ('#[doc = "{"]\nconst BAD: u8 = 1;\n', 2),
            ('#[doc = "{"] const BAD: u8 = 1;\n', 1),
            ('#[doc = r#"{ /* ["#]\nconst BAD: u8 = 1;\n', 2),
            ('pub use crate::Thing; const BAD: u8 = 1;\n', 1),
            ('mod child; const BAD: u8 = 1;\n', 1),
            ('pub use crate::{\n    Thing, // }\n};\nconst BAD: u8 = 1;\n', 4),
        ]
        for source, line in fixtures:
            with self.subTest(source=source):
                findings = self.findings(source)
                self.assertEqual(len(findings), 1)
                self.assertEqual((findings[0][0], findings[0][2]),
                                 (line, checker.VIOLATION_REASON))

    def test_literal_contents_do_not_change_item_boundaries(self):
        literals = [
            ('"}; /* [ \\\\\\\""', "&'static str"),
            ('r##"}; /* [ \\"#"##', "&'static str"),
            ('b"}; /* ["', "&'static [u8]"),
            ('br#"}; /* ["#', "&'static [u8]"),
            ('c"}; /* ["', "&'static std::ffi::CStr"),
            ('cr#"}; /* ["#', "&'static std::ffi::CStr"),
            ("'}'", 'char'), ("b'{'", 'u8'),
            (r"'\u{7d}'", 'char'), (r"'\''", 'char'),
        ]
        for literal, rust_type in literals:
            source = f"const FIRST: {rust_type} = {literal};\nconst SECOND: u8 = 1;\n"
            with self.subTest(literal=literal):
                self.assertEqual([finding[0] for finding in self.findings(source)], [1, 2])

    def test_multiline_literals_preserve_following_diagnostic_lines(self):
        source = '#[doc = r#"{\n/* ]"#]\nconst BAD: u8 = 1;\n'
        self.assertEqual([finding[0] for finding in self.findings(source)], [3])

    def test_const_blocks_and_function_bodies_keep_following_items(self):
        source = ('const FIRST: u8 = { let local = 1; local };\n'
                  'const fn bad() { let local = 1; }\n'
                  'static SECOND: u8 = 2;\n')
        self.assertEqual([finding[0] for finding in self.findings(source)], [1, 2, 3])

    def test_unterminated_lexical_constructs_fail_closed(self):
        for source in ('/* unterminated', '#[doc = "unterminated',
                       '#[doc = r#"unterminated', '#[cfg(test)'):
            with self.subTest(source=source):
                self.assertTrue(self.findings(source))

    def run_checker(self, source, *, generated=False, github=False):
        scripts = self.root / 'scripts' / 'ci'
        scripts.mkdir(parents=True, exist_ok=True)
        for script in Path(checker.__file__).parent.glob('*.py'):
            shutil.copyfile(script, scripts / script.name)
        module = self.root / 'src' / ('generated' if generated else 'fixture') / 'mod.rs'
        module.parent.mkdir(parents=True, exist_ok=True)
        module.write_text(source, encoding='utf-8')
        env = os.environ.copy()
        env.pop('GITHUB_ACTIONS', None)
        if github:
            env['GITHUB_ACTIONS'] = 'true'
        return subprocess.run([sys.executable, str(scripts / 'check_mod_declarations.py')],
                              capture_output=True, text=True, env=env, check=False)

    def test_cli_rejects_review_reproductions(self):
        for source in ('const BAD: u8 = 1; /* explanation */\n',
                       '#[doc = "{"]\nconst BAD: u8 = 1;\n'):
            with self.subTest(source=source):
                result = self.run_checker(source)
                self.assertEqual(result.returncode, 1)
                self.assertIn(checker.VIOLATION_REASON, result.stderr)
                self.assertNotIn('all module roots are declarations-only', result.stdout)

    def test_cli_preserves_github_diagnostic_line(self):
        result = self.run_checker('#[doc = "{"]\nconst BAD: u8 = 1;\n', github=True)
        self.assertEqual(result.returncode, 1)
        self.assertIn('::error file=', result.stdout)
        self.assertIn(',line=2::', result.stdout)

    def test_cli_accepts_documented_generated_exemption(self):
        source = '// @generated\n// Generator: synthetic\n// Source: fixture\nconst OK: u8 = 1;\n'
        result = self.run_checker(source, generated=True)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn('1 documented generated exemptions', result.stdout)

    def test_cli_rejects_missing_or_late_generated_markers(self):
        for source in ('const BAD: u8 = 1;\n',
                       '\n' * 20 + '// @generated Generator: synthetic Source: fixture\n'):
            with self.subTest(source=source):
                result = self.run_checker(source, generated=True)
                self.assertEqual(result.returncode, 1)
                self.assertIn('generated-path exemption requires', result.stdout)

    def test_cli_accepts_compliant_root(self):
        result = self.run_checker('/* outer /* nested */\n*/\npub use crate::Thing;\n')
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_generated_ancestor_outside_src_does_not_exempt_module(self):
        self.root = self.root / 'generated'
        result = self.run_checker('pub use crate::Thing;\n')
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn('0 documented generated exemptions', result.stdout)


if __name__ == '__main__':
    unittest.main()
