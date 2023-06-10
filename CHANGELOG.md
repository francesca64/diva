# Unreleased

# v0.1.0 (2023-06-09)

(Relative to [`bossy` v0.2.1](https://github.com/BrainiumLLC/bossy/blob/master/CHANGELOG.md#021-2021-01-08))

- **Breaking:** Removed explicit `pure`/`impure` constructors; commands now inherit the environment by default, and the `pure` method can be called to clean the environment.
- Added methods to `diva::Command` for setting the current working directory.
- Added `run_and_wait_for_trimmed` convenience method due to overwhelming ubiquity.
- Fixed Windows build.
