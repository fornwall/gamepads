// Necessary for gamepad querying to work?
globalThis.addEventListener("gamepadconnected", function (e) {
    console.log(
        "Gamepad connected at index %d: id='%s'. %d buttons, %d axes.",
        e.gamepad.index,
        e.gamepad.id,
        e.gamepad.buttons.length,
        e.gamepad.axes.length
    );
});

miniquad_add_plugin({
    register_plugin: (importObject) => {
        const BYTES_PER_GAMEPAD = 256;

        importObject.env.getGamepads = (data_ptr, max_length) => {
            const getArray = (ptr, arr) => {
                return new arr(wasm_memory.buffer, ptr);
            }

            const f32 = getArray(data_ptr, Float32Array);
            const u32 = getArray(data_ptr, Uint32Array);
            const bytes = getArray(data_ptr, Uint8Array);

            const gamepads = navigator.getGamepads();

            let gamepadOffset = 0;
            for (const gamepad of gamepads) {
                if (!gamepad) continue;
                let byteOffset = BYTES_PER_GAMEPAD * gamepadOffset;

                bytes[byteOffset++] = gamepad.index;
                bytes[byteOffset++] = gamepad.connected ? 1 : 0;
                byteOffset += 2;

                for (const axes of gamepad.axes.slice(0, 4)) {
                    f32[byteOffset / 4] = axes;
                    byteOffset += 4;
                }

                let pressed_bits = 0;
                for (const [index, button] of gamepad.buttons.entries()) {
                    if (index < 17 && button.pressed) pressed_bits |= (1 << index);
                }
                u32[byteOffset / 4] = pressed_bits;

                gamepadOffset += 1;
            }

            return gamepadOffset;
        }
    },
    name: "gamepads",
    version: "0.1.0"
});
