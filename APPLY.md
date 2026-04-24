# APPLY

Run these commands exactly:

```bash
cd ~/Downloads || exit 1
unzip -o doppelcore_phase_8e_repo_init_slice_2026-04-24.zip

mkdir -p ~/Forge/ecosystem/DoppelCore
cp -R ~/Downloads/doppelcore_phase_8e_repo_init_slice_2026-04-24/. ~/Forge/ecosystem/DoppelCore/

cd ~/Forge/ecosystem/DoppelCore || exit 1
[ -d .git ] || git init -b main

cargo test
cargo check
git status --short
```
