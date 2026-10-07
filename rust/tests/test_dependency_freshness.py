"""Offline regression coverage for the direct-dependency freshness policy."""
import importlib.util
from pathlib import Path
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "scripts/check-dependency-freshness.py"
SPEC = importlib.util.spec_from_file_location("freshness", SCRIPT)


class FreshnessTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.checker = importlib.util.module_from_spec(SPEC)
        SPEC.loader.exec_module(cls.checker)

    def test_latest_stable_ignores_yanked_and_prerelease(self):
        versions = [{"vers": "0.7.1", "yanked": False},
                    {"vers": "0.8.0-beta.1", "yanked": False},
                    {"vers": "0.9.0", "yanked": True}]
        self.assertEqual(self.checker.latest_stable(versions), "0.7.1")

    def test_numeric_versions_are_compared_numerically(self):
        self.assertEqual(self.checker.latest_stable([
            {"vers": "1.9.0", "yanked": False},
            {"vers": "1.10.0+build-1", "yanked": False}]), "1.10.0+build-1")

    def test_stale_dependency_fails_without_blocker(self):
        with self.assertRaisesRegex(ValueError, "0.6.11.*0.7.1"):
            self.checker.check_version("tower-http", "0.6.11", "0.7.1", None, lambda _: {})

    def test_open_issue_exception_is_allowed(self):
        self.checker.check_version("tower-http", "0.6.11", "0.7.1",
            "https://github.com/link-assistant/web-search/issues/123",
            lambda _: {"state": "open", "body": "Blocked by upstream API change"})

    def test_closed_issue_and_pull_request_cannot_exempt(self):
        for issue in ({"state": "closed"}, {"state": "open", "pull_request": {}},
                      {"state": "open", "body": ""}, {"state": "open", "body": "  "}):
            with self.subTest(issue=issue), self.assertRaises(ValueError):
                self.checker.check_version("x", "1.0.0", "2.0.0",
                    "https://github.com/o/r/issues/1", lambda _: issue)

    def test_network_failure_is_not_silently_allowed(self):
        def fail(_):
            raise OSError("registry unavailable")
        with self.assertRaises(OSError):
            self.checker.check_version("x", "1.0.0", "2.0.0",
                "https://github.com/o/r/issues/1", fail)

    def test_exception_must_be_on_dependency_line(self):
        text = '[dependencies]\n# https://github.com/o/r/issues/2\nx = "1"\ny = "1" # blocked: https://github.com/o/r/issues/3\n'
        self.assertIsNone(self.checker.blocker_for(text, "x"))
        self.assertEqual(self.checker.blocker_for(text, "y"), "https://github.com/o/r/issues/3")

    def test_current_version_passes_without_network(self):
        self.checker.check_version("x", "2.0.0", "2.0.0", None,
                                  lambda _: self.fail("unexpected network request"))


if __name__ == "__main__":
    unittest.main()
