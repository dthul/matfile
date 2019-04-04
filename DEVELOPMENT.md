# Publishing a new version

## Adjust version number

In `Cargo.toml` as well as the `html_root_url` in `lib.rs`. Stick to [semantic versioning](https://semver.org/spec/v2.0.0.html).

## Tag the current commit

```bash
GIT_COMMITTER_DATE=$(git log -n1 --pretty=%aD) git tag -a -m "Release 0.3.0" 0.3.0
git push --tags
```