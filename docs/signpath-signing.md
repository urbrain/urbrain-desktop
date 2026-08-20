# Windows signing via SignPath Foundation (free, open source)

[SignPath Foundation](https://signpath.org/) provides **free code signing** for
open-source projects. Because `urbrain-desktop` is public and Apache-2.0, it can
qualify. Signing happens as a cloud service **after** the build: the release
workflow's `sign-windows` job submits the Windows installers, SignPath signs them,
and the job replaces the unsigned assets on the release.

> The job is dormant until you set the `SIGNPATH_ORGANIZATION_ID` repository
> variable, so nothing changes until SignPath is fully set up. macOS/Linux are
> unaffected. Note: SignPath issues **OV-class** certificates — SmartScreen trust
> builds with download reputation rather than being instant.

---

## Step 1 — Apply to the SignPath Foundation program
1. Go to <https://signpath.org/apply> and submit `urbrain-desktop`.
2. Provide: the public repo URL (`https://github.com/urbrain/urbrain-desktop`),
   the OSI license (Apache-2.0), and a short project description.
3. SignPath reviews it (this is the gate — allow a few days). They favor projects
   with a real user base and a clean, buildable public repo.

Everything below happens **after** they approve you and create your SignPath
organization.

## Step 2 — Configure the SignPath project
In the SignPath web app:
1. **Project** — create/confirm a project (note its **slug**, e.g. `urbrain-desktop`).
2. **Artifact configuration** — add one that matches what the workflow submits: a
   **ZIP** containing the Windows installers, with the `.exe` and `.msi` marked for
   **Authenticode** signing. Note its **slug**.
3. **Signing policy** — create a policy (e.g. `release-signing`) bound to your
   Foundation certificate. Note its **slug**.
4. **CI integration** — connect the GitHub repo and create a **CI user / API token**
   for the signing request.

## Step 3 — Add GitHub variables + secret
On `urbrain/urbrain-desktop` → **Settings → Secrets and variables → Actions**:

**Variables** tab → **New repository variable** (these are not secret):
| Name | Value |
| --- | --- |
| `SIGNPATH_ORGANIZATION_ID` | your SignPath organization id (GUID) |
| `SIGNPATH_PROJECT_SLUG` | project slug from Step 2.1 |
| `SIGNPATH_ARTIFACT_CONFIG_SLUG` | artifact configuration slug from Step 2.2 |
| `SIGNPATH_SIGNING_POLICY_SLUG` | signing policy slug from Step 2.3 |

**Secrets** tab → **New repository secret**:
| Name | Value |
| --- | --- |
| `SIGNPATH_API_TOKEN` | the CI user API token from Step 2.4 |

Setting `SIGNPATH_ORGANIZATION_ID` is what switches the `sign-windows` job on.

## Step 4 — Cut a release
Actions → **Release** → Run workflow → tag `v1.0.1` (or push the tag). The build
publishes a draft with unsigned installers, then `sign-windows` signs the `.exe`
/ `.msi` and replaces them. Review the draft, then publish.

Verify: on the installer, right-click → **Properties → Digital Signatures** should
list the SignPath Foundation certificate.

---

## Notes
- **OV reputation:** the first signed downloads may still show SmartScreen until
  reputation accrues; it improves as more users download the signed build.
- **Not approved (yet)?** Keep shipping unsigned — the download page already tells
  users how to open the app. Revisit SignPath once the project has more traction,
  or use a paid EV/Azure path if you need instant trust sooner.

---

© 2025-2026 GITC · gitcz.com & gitcmena.tech — Global Innovation Technology Company (GITC)
