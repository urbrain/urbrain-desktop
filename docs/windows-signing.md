# Windows code signing

Unsigned Windows installers trigger SmartScreen's *"Windows protected your PC"*
warning. The release workflow signs the Windows build when the right secrets are
set — pick **one** of the options below.

> Safe either way: with no signing secrets set, the Windows build is simply
> **unsigned** (the workflow writes an empty `sign-config.json`), so releases keep
> working. If more than one option's secrets are set, priority is
> **eSigner → Azure → .pfx**.

---

## Which option?

| | SSL.com eSigner (EV) | Azure Trusted Signing | Standard `.pfx` |
| --- | --- | --- | --- |
| SmartScreen warning | **Gone immediately** | Gone immediately (verified orgs) | OV: builds reputation over time · EV: gone |
| Works from Saudi Arabia / most countries | ✅ | ❌ Public Trust validation is country-limited | ✅ |
| Cloud / CI-friendly | ✅ | ✅ | ✅ (OV); EV usually needs a hardware token |
| Cost | ~$250–400/yr | ~$10/mo | OV ~$100–200/yr |

**Recommended here: SSL.com eSigner (EV).** Azure Trusted Signing's identity
validation isn't available in Saudi Arabia, and eSigner gives instant SmartScreen
trust with cloud signing that works in CI.

---

## Option A — SSL.com eSigner (recommended)

1. Buy an **EV Code Signing** certificate from **SSL.com** and complete their
   validation (they issue to Saudi / Middle East organizations).
2. In the SSL.com dashboard, enable **eSigner** for the certificate and set up
   automated signing:
   - Note your **credential ID** for the signing certificate.
   - Generate the **eSigner TOTP / automation secret** (a long base64 string used
     for headless 2FA).

Set these secrets on `urbrain/urbrain-desktop`
(*Settings → Secrets and variables → Actions*):

```bash
gh auth switch --user urbrain
gh secret set ESIGNER_USERNAME      --repo urbrain/urbrain-desktop   # SSL.com account username
gh secret set ESIGNER_PASSWORD      --repo urbrain/urbrain-desktop   # SSL.com account password
gh secret set ESIGNER_TOTP_SECRET   --repo urbrain/urbrain-desktop   # eSigner automation/TOTP secret
gh secret set ESIGNER_CREDENTIAL_ID --repo urbrain/urbrain-desktop   # signing credential id (needed if >1 cert)
```

Optional: if the CodeSignTool download in CI fails, pin the exact Windows zip URL
(from SSL.com's CodeSignTool guide) — the workflow defaults to SSL.com's latest:

```bash
gh secret set ESIGNER_CODESIGNTOOL_URL --repo urbrain/urbrain-desktop   # direct .zip URL (optional)
```

The workflow downloads CodeSignTool, points Tauri's `signCommand` at
`scripts/esigner-sign.ps1`, and signs every binary and installer in the cloud —
no certificate ever touches the runner.

---

## Option B — Azure Trusted Signing

Requires a Trusted Signing account + certificate profile + a service principal.
Note: **Public Trust identity validation is limited by country** (not available
in Saudi Arabia at time of writing). See `docs/azure-trusted-signing.md` for the
full walkthrough. Secrets:

```bash
gh secret set AZURE_TENANT_ID        --repo urbrain/urbrain-desktop
gh secret set AZURE_CLIENT_ID        --repo urbrain/urbrain-desktop
gh secret set AZURE_CLIENT_SECRET    --repo urbrain/urbrain-desktop
gh secret set AZURE_SIGNING_ENDPOINT --repo urbrain/urbrain-desktop   # e.g. https://eus.codesigning.azure.net
gh secret set AZURE_SIGNING_ACCOUNT  --repo urbrain/urbrain-desktop
gh secret set AZURE_SIGNING_PROFILE  --repo urbrain/urbrain-desktop
```

---

## Option C — Standard `.pfx` certificate

Buy an OV (or file-based EV) cert, export as `.pfx`, base64-encode it:

```bash
base64 -i code-signing.pfx -o win_cert.b64
gh secret set WINDOWS_CERTIFICATE          --repo urbrain/urbrain-desktop < win_cert.b64
gh secret set WINDOWS_CERTIFICATE_PASSWORD --repo urbrain/urbrain-desktop
gh secret set WINDOWS_TIMESTAMP_URL        --repo urbrain/urbrain-desktop   # optional; defaults to digicert
rm win_cert.b64 code-signing.pfx
```

OV certs sign the app but SmartScreen still warns until download reputation
builds; EV removes it immediately.

---

## Cutting a signed release

```bash
git tag -f -a v1.0.1 -m "Urbrain Desktop v1.0.1 (signed)" && git push origin -f v1.0.1
```

Verify on the produced installer: right-click → **Properties → Digital
Signatures** should list your certificate.

---

© 2025-2026 GITC · gitcz.com & gitcmena.tech — Global Innovation Technology Company (GITC)
