# The OG CLI

The powertool for DG developers.

> :warning: Thanks to the ongoing effort to package the DG CLI to make it more
> reliable, there's no longer as big a need for this. The repo will be kept
> around and you're free to develop and release tools you want to build for
> yourself, but this is not an officially supported project.

## Releasing

1. Update version in `Cargo.toml` and run `cargo check` to ensure version is
   propagated to `Cargo.lock`
2. Create tag with `git tag -am "Version 0.10.0" v0.10.0`
3. Push with `git push --follow-tag`

The release workflow will start on the tagged commit.
