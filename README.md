

## Auto-update via GitHub Releases
1. Build a release bundle (msi/dmg).
2. Generate a Tauri updater JSON (`latest-{{target}}.json`) and host it on GitHub Pages or in a release asset.
3. Set `apps/desktop/tauri.conf.json > updater.endpoints[0]` to your raw URL, e.g.:
   `https://raw.githubusercontent.com/<owner>/<repo>/gh-pages/latest-{{target}}.json`
4. On app start, the updater will check and prompt when a newer version exists.
