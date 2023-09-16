#!/bin/sh
set -e -u

# Install uglify with:
# npm install uglify-js -g

PACKAGE_VERSION=$(cd .. && cargo metadata --format-version=1 --no-deps | jq '.packages[0].version')

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
