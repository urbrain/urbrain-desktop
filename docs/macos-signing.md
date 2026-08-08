# macOS code signing + notarization

Unsigned macOS builds trigger Gatekeeper's *"Apple could not verify Urbrain is
free of malware"* block. To ship a build every user can open, the app must be
**signed** with a *Developer ID Application* certificate and **notarized** by Apple.

The release workflow already reads these six secrets — once they're set, every
tagged build is signed, notarized, and stapled automatically:

| Secret | What it is |
| --- | --- |
| `APPLE_CERTIFICATE` | base64 of your Developer ID Application `.p12` |
| `APPLE_CERTIFICATE_PASSWORD` | the password you set when exporting the `.p12` |
| `APPLE_SIGNING_IDENTITY` | e.g. `Developer ID Application: Your Name (TEAMID)` |
| `APPLE_ID` | your Apple ID email (for notarization) |
| `APPLE_PASSWORD` | an **app-specific** password (NOT your real Apple ID password) |
| `APPLE_TEAM_ID` | your 10-character Team ID |

> ⚠️ All six must be set. The workflow passes them unconditionally, so a missing
> or empty secret makes the macOS codesign step fail. If you ever want unsigned
> builds again, remove the `APPLE_*` env lines from `.github/workflows/release.yml`.

---

## Prerequisite: Apple Developer Program

You need a paid **Apple Developer Program** membership ($99/year):
https://developer.apple.com/programs/enroll/
Your **Team ID** is shown under *Membership details* (10 chars, e.g. `ABCDE12345`).

## 1. Create a Developer ID Application certificate

**Easiest (Xcode):** Xcode → Settings → Accounts → add your Apple ID →
*Manage Certificates…* → **+** → **Developer ID Application**.

**Or via the portal + CSR:**
1. Keychain Access → *Certificate Assistant* → *Request a Certificate from a
   Certificate Authority* → save a `.certSigningRequest` to disk.
2. https://developer.apple.com/account/resources/certificates → **+** →
   **Developer ID Application** → upload the CSR → download the `.cer` →
   double-click it to install into your login keychain.

## 2. Export the certificate as `.p12`

Keychain Access → **My Certificates** → find
`Developer ID Application: <Name> (TEAMID)` (it must show a disclosure triangle
with a private key under it) → right-click → **Export…** → format
*Personal Information Exchange (.p12)* → save as `Urbrain-DeveloperID.p12` and
set a strong password. **That password = `APPLE_CERTIFICATE_PASSWORD`.**

## 3. Base64-encode the `.p12`

```bash
base64 -i Urbrain-DeveloperID.p12 -o apple_cert.b64
```

## 4. Get the signing identity string

```bash
security find-identity -v -p codesigning
```

Copy the full quoted name, e.g. `Developer ID Application: Your Name (ABCDE12345)`
— that's `APPLE_SIGNING_IDENTITY`.

## 5. Create an app-specific password (for notarization)

https://account.apple.com → *Sign-In and Security* → *App-Specific Passwords* →
**+** → name it "Urbrain notarization" → copy the generated `xxxx-xxxx-xxxx-xxxx`.
**That = `APPLE_PASSWORD`.** (`APPLE_ID` is your Apple ID email.)

## 6. Store all six secrets on the release repo

Run these (the file redirect and prompts keep values out of your shell history):

```bash
gh auth switch --user urbrain
gh secret set APPLE_CERTIFICATE          --repo urbrain/urbrain-desktop < apple_cert.b64
gh secret set APPLE_CERTIFICATE_PASSWORD --repo urbrain/urbrain-desktop   # paste .p12 password
gh secret set APPLE_SIGNING_IDENTITY     --repo urbrain/urbrain-desktop   # paste identity string
gh secret set APPLE_ID                   --repo urbrain/urbrain-desktop   # your Apple ID email
gh secret set APPLE_PASSWORD             --repo urbrain/urbrain-desktop   # app-specific password
gh secret set APPLE_TEAM_ID              --repo urbrain/urbrain-desktop   # 10-char Team ID
```

Verify: `gh secret list --repo urbrain/urbrain-desktop` — you should see all six
plus `CLIENT_CHECKOUT_TOKEN`.

Then delete the local sensitive files:

```bash
rm apple_cert.b64 Urbrain-DeveloperID.p12
```

## 7. Cut a signed release

Once the secrets are set, re-cut the release (a signed, notarized build):

```bash
git tag -f -a v1.0.1 -m "Urbrain Desktop v1.0.1 (signed)" && git push origin -f v1.0.1
```

The workflow signs the `.app`, notarizes it with Apple, staples the ticket, and
publishes the release. Users can then open Urbrain with no Gatekeeper warning.

---

© 2025-2026 GITC · gitcz.com & gitcmena.tech — Global Innovation Technology Company (GITC)
