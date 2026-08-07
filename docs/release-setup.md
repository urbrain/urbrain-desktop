# Release setup — urbrain-desktop

The desktop installers (macOS `.dmg`, Windows `.msi`/`.exe`, Linux `.deb`/`.AppImage`)
are built and published by [`.github/workflows/release.yml`](../.github/workflows/release.yml)
using [`tauri-action`](https://github.com/tauri-apps/tauri-action). The website's
[Download page](https://urbrain.ai/download) reads the **latest release of this repo**.

## How a release works

1. The workflow triggers on any pushed tag matching `v*` (or manually via
   **Actions → Release → Run workflow**, passing a tag).
2. It checks out **two repos side by side** under the workspace root, because the
   Tauri config points at the frontend as a sibling folder (`../urbrain-client`):
   - `urbrain-desktop/` — this repo
   - `urbrain-client/` — the private web frontend (built by `beforeBuildCommand`)
3. It builds the frontend, then runs `tauri-action` across a
   macOS (universal) / Ubuntu 22.04 / Windows matrix.
4. Installers are attached to a **draft** GitHub Release named `Urbrain vX.Y.Z`.
   Review the assets, then publish the release — the Download page picks it up.

## Required secret

| Secret | What it is | Scope |
| --- | --- | --- |
| `CLIENT_CHECKOUT_TOKEN` | Fine-grained PAT used to check out the private `projectminovative/urbrain-client` frontend | **Contents: Read** on `projectminovative/urbrain-client` |

**Generate it** (as an account with access to `urbrain-client`):
GitHub → *Settings → Developer settings → Fine-grained personal access tokens →
Generate new token* → Resource owner `projectminovative`, Repository access:
*Only select repositories* → `urbrain-client`, Permissions: *Repository → Contents → Read-only*.

**Store it** in this repo: *Settings → Secrets and variables → Actions →
New repository secret* → name `CLIENT_CHECKOUT_TOKEN`, paste the token value.

> Do not paste tokens into code, chat, or commits. Add them only via the
> encrypted **Actions secrets** UI.

The release upload itself uses the built-in `GITHUB_TOKEN` — no extra secret needed.

## Optional secrets (code signing / auto-update)

Leave these unset to ship unsigned builds; add them later to notarize macOS
builds and enable the Tauri updater. All are read by the workflow if present:

- **macOS notarization:** `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`,
  `APPLE_SIGNING_IDENTITY`, `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`
- **Tauri updater signing:** `TAURI_SIGNING_PRIVATE_KEY`,
  `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

## Cutting a release

```bash
# from the urbrain-desktop repo
git tag v1.0.0
git push origin v1.0.0
```

Then watch **Actions → Release**, review the draft Release it creates, and publish.

---

© 2025-2026 GITC · gitcz.com & gitcmena.tech — Global Innovation Technology Company (GITC)
