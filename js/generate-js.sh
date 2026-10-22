#!/bin/sh
set -e -u

# Install uglify with:
# npm install uglify-js -g

# miniquad compares the plugin version with gamepads_crate_version()
# (src/backend_web_direct.rs), which encodes it as a number:
# (major << 24) + (minor << 16) + patch. Write it the same way.
PACKAGE_VERSION=$(cd .. && cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version | split(".") | "(\(.[0]) << 24) + (\(.[1]) << 16) + \(.[2])"')

(cat - | uglifyjs --mangle eval - > gamepads-0.1.js) < gamepads-src-0.1.js

sed 's/function registerHostFunctions/export default function/' < gamepads-src-0.1.js | uglifyjs --mangle eval - > gamepads-module-0.1.js

SOURCE_UNNAMED_FUNCTION=$(sed 's/ registerHostFunctions//' < gamepads-src-0.1.js)
(cat - | uglifyjs --mangle toplevel - > macroquad-gamepads-0.1.js) <<EOF
miniquad_add_plugin({
    name: "gamepads",
    version: $PACKAGE_VERSION,
    register_plugin: $SOURCE_UNNAMED_FUNCTION
});
EOF
