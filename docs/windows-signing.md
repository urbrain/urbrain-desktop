# Windows code signing

Unsigned Windows installers trigger SmartScreen's *"Windows protected your PC"*
warning, which scares users off. The release workflow signs the Windows build
when the right secrets are set — pick **one** of the two options below.

> The workflow is safe either way: with no signing secrets set, the Windows build
> is simply **unsigned** (it writes an empty `sign-config.json`), so releases keep
> working while you get signing set up.

---

## Which option?

| | Azure Trusted Signing (recommended) | Standard `.pfx` certificate |
| --- | --- | --- |
| SmartScreen warning | **Gone immediately** for verified orgs | **OV:** persists until download reputation builds. **EV:** gone immediately |
| Cost | ~$10/month | OV ~$100–200/yr · EV ~$250–400/yr |
| CI-friendly | ✅ cloud, no hardware token | ✅ OV. ❌ EV usually needs a hardware token (not cloud-CI friendly) |
| Setup time | Azure org identity validation (~1–5 days) | CA validation (OV hours–days, EV longer) |

**For the goal of "users never see the warning," use Azure Trusted Signing** (or an
EV cert if you have a cloud-signing arrangement). A plain OV `.pfx` signs the app
but SmartScreen still warns until your signature earns reputation.

---

## Option A — Azure Trusted Signing (recommended)

1. In the Azure Portal, create a **Trusted Signing account** and a **certificate
   profile** (Microsoft validates your organization's identity first).
2. Create an **App registration** (service principal) and give it the
   **Trusted Signing Certificate Profile Signer** role on the account.
3. Note: the signing **endpoint** (e.g. `https://eus.codesigning.azure.net`), the
   **account name**, and the **certificate profile name**.

Set these secrets on `urbrain/urbrain-desktop`
(*Settings → Secrets and variables → Actions*):

```bash
gh auth switch --user urbrain
# service principal auth
gh secret set AZURE_TENANT_ID        --repo urbrain/urbrain-desktop
gh secret set AZURE_CLIENT_ID        --repo urbrain/urbrain-desktop
gh secret set AZURE_CLIENT_SECRET    --repo urbrain/urbrain-desktop
# signing target
gh secret set AZURE_SIGNING_ENDPOINT --repo urbrain/urbrain-desktop   # e.g. https://eus.codesigning.azure.net
gh secret set AZURE_SIGNING_ACCOUNT  --repo urbrain/urbrain-desktop   # your Trusted Signing account name
gh secret set AZURE_SIGNING_PROFILE  --repo urbrain/urbrain-desktop   # your certificate profile name
```

The workflow installs `trusted-signing-cli`, sets Tauri's `signCommand`, and signs
every Windows binary using the service-principal credentials. Nothing else to do.

---

## Option B — Standard `.pfx` certificate

1. Buy a **code signing certificate** (OV or, for instant trust, a cloud/file-based
   EV) from a CA (DigiCert, Sectigo, SSL.com, …).
2. Export it (with its private key) as a **`.pfx`** with a password.
3. Base64-encode it:

```bash
base64 -i code-signing.pfx -o win_cert.b64
```

Set the secrets:

```bash
gh auth switch --user urbrain
gh secret set WINDOWS_CERTIFICATE          --repo urbrain/urbrain-desktop < win_cert.b64
gh secret set WINDOWS_CERTIFICATE_PASSWORD --repo urbrain/urbrain-desktop   # the .pfx password
# optional — defaults to http://timestamp.digicert.com
gh secret set WINDOWS_TIMESTAMP_URL        --repo urbrain/urbrain-desktop
```

Then delete the local files:

```bash
rm win_cert.b64 code-signing.pfx
```

The workflow imports the cert, reads its thumbprint, and signs via `signtool` with
an RFC3161 timestamp (so signatures stay valid after the cert expires).

> ⚠️ If **both** Azure and `.pfx` secrets are set, Azure Trusted Signing wins.

---

## Cutting a signed Windows release

Once the secrets are in place, tag a release as usual — the Windows job signs
automatically:

```bash
git tag -f -a v1.0.1 -m "Urbrain Desktop v1.0.1 (signed)" && git push origin -f v1.0.1
```

Verify on the produced installer: right-click → **Properties → Digital Signatures**
should list your certificate.

---

© 2025-2026 GITC · gitcz.com & gitcmena.tech — Global Innovation Technology Company (GITC)
