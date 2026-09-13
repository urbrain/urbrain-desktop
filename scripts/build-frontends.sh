#!/usr/bin/env sh
# Bundle BOTH front-ends into one dist for the desktop app:
#   consumer app       (urbrain-client)     -> served at /
#   business dashboard (urbrain-dashboard)  -> served at /dashboard/
#
# Both are separate sibling repos checked out next to this one (../urbrain-client,
# ../urbrain-dashboard). Run from the urbrain-desktop root (Tauri's beforeBuildCommand
# cwd). Their dependencies must already be installed (CI does `npm ci` in each).
#
# The API base is baked into both builds via VITE_API_URL. In a desktop bundle there
# is no same-origin backend, so this MUST be an absolute URL. Override by exporting
# VITE_API_URL before the build; otherwise it defaults to the production API.
set -eu

export VITE_API_URL="${VITE_API_URL:-https://api.urbrain.ai/api/v1}"
echo "Using VITE_API_URL=$VITE_API_URL"

echo "==> Building consumer app (urbrain-client) at / ..."
( cd ../urbrain-client && npm run build )

echo "==> Building business dashboard (urbrain-dashboard) at /dashboard/ ..."
( cd ../urbrain-dashboard && APP_BASE=/dashboard/ npm run build )

echo "==> Combining: urbrain-client/dist/dashboard <- urbrain-dashboard/dist ..."
rm -rf ../urbrain-client/dist/dashboard
cp -R ../urbrain-dashboard/dist ../urbrain-client/dist/dashboard

echo "==> Done. Combined frontendDist ready at ../urbrain-client/dist"
