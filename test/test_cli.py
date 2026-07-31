import sys
import json
from pathlib import Path

import fetter


def test_module_has_run():
    assert hasattr(fetter, 'run')


def test_module_has_run_with_argv():
    assert hasattr(fetter, 'run_with_argv')


def test_module_has_validate():
    assert hasattr(fetter, 'validate')


def test_run_count_current_exe():
    # Counting over the current interpreter exercises platform-specific
    # executable and site-packages discovery (e.g. Windows
    # "Lib\\site-packages" vs POSIX "lib/pythonX.Y/site-packages"). This
    # should complete without raising on every supported platform,
    # including Windows.
    # The first argument is treated as the program name (as with argv) and
    # is discarded by the CLI parser.
    fetter.run(['fetter', '-e', sys.executable, 'count', 'display'])


def test_run_scan_current_exe():
    # Scanning reports installed packages for the current interpreter,
    # further exercising cross-platform path handling without network
    # access.
    fetter.run(['fetter', '-e', sys.executable, 'scan', 'display'])


def test_run_search_current_exe():
    # Searching adds glob-like matching over discovered package paths.
    fetter.run(
        ['fetter', '-e', sys.executable, 'search', '--pattern', '*', 'display']
    )


def test_validate_returns_json_string(tmp_path: Path):
    requirements = tmp_path / "requirements.txt"
    requirements.write_text("package-that-should-not-exist-anywhere==0.0.0\n")

    result = fetter.validate(
        str(requirements),
        exes=[sys.executable],
        superset=True,
    )
    payload = json.loads(result)
    assert isinstance(payload, list)
