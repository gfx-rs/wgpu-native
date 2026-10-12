"""CPU-only checks for test harness setup and runtime selection."""

import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location("runner", Path(__file__).resolve().parents[1] / "run-wgpu-tests.py")
RUNNER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RUNNER)


class RunnerTests(unittest.TestCase):
    def test_lockfile_field_order_does_not_matter(self):
        with tempfile.TemporaryDirectory() as directory:
            lock = Path(directory) / "Cargo.lock"
            commit = "a" * 40
            lock.write_text(f'[[package]]\nsource = "git+https://example.invalid/wgpu#{commit}"\nversion = "30.0.0"\nname = "wgpu"\n')
            with patch.object(RUNNER, "CARGO_LOCK", lock):
                self.assertEqual(RUNNER.get_wgpu_commit(), commit)

    def test_lockfile_rejects_ambiguous_wgpu_revisions(self):
        with tempfile.TemporaryDirectory() as directory:
            lock = Path(directory) / "Cargo.lock"
            lock.write_text('[[package]]\nname = "wgpu"\nsource = "git+https://example.invalid/wgpu#' + "a" * 40 + '"\n' +
                            '[[package]]\nname = "wgpu"\nsource = "git+https://example.invalid/wgpu#' + "b" * 40 + '"\n')
            with patch.object(RUNNER, "CARGO_LOCK", lock), self.assertRaises(SystemExit):
                RUNNER.get_wgpu_commit()

    def test_dirty_checkout_is_not_reset(self):
        with tempfile.TemporaryDirectory() as directory:
            with patch.object(RUNNER, "LOCAL_WGPU", Path(directory)), \
                    patch.object(RUNNER.subprocess, "check_output", return_value=b" M Cargo.toml\n"), \
                    patch.object(RUNNER, "run") as run, self.assertRaises(SystemExit):
                RUNNER.clone_at_commit("a" * 40, offline=True)
            run.assert_not_called()

    def test_offline_setup_never_clones(self):
        with tempfile.TemporaryDirectory() as directory:
            with patch.object(RUNNER, "LOCAL_WGPU", Path(directory) / "absent"), \
                    patch.object(RUNNER, "run") as run, self.assertRaises(SystemExit):
                RUNNER.clone_at_commit("a" * 40, offline=True)
            run.assert_not_called()

    def test_patch_drift_does_not_write(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / "Cargo.toml"
            source.write_text("changed upstream")
            with self.assertRaises(SystemExit):
                RUNNER.patch_file(source, "expected", "replacement")
            self.assertEqual(source.read_text(), "changed upstream")

    def test_upstream_test_patch_is_checked_before_application(self):
        with patch.object(RUNNER, "run") as run:
            RUNNER.apply_test_patches()
        self.assertEqual(run.call_count, 2)
        self.assertEqual(run.call_args_list[0].args[3:5], ("apply", "--check"))
        self.assertEqual(run.call_args_list[1].args[3], "apply")

    def test_upstream_test_patch_drift_stops_application(self):
        with patch.object(RUNNER, "run", side_effect=RuntimeError("patch drift")) as run:
            with self.assertRaises(RuntimeError):
                RUNNER.apply_test_patches()
        self.assertEqual(run.call_count, 1)


if __name__ == "__main__":
    unittest.main()
