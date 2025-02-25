function registerHostFunctions(importObject, wasm_memory_holder) {
  const MAX_GAMEPADS = 8;
  const BYTES_PER_GAMEPAD = 36;

  // How should deadzones be handled in browsers?
  // See e.g. https://github.com/ensemblejs/gamepad-api-mappings
  // For now use a value picked from some limited data points.
  const DEADZONE = 0.04;

  // Event listener on 'gamepadconnected' necessary for gamepad listing to work.
  globalThis.addEventListener(`gamepadconnected`, () => {});

  importObject.env.getGamepads = (wasm_memory_offset) => {
    const gamepads = navigator.getGamepads();
    // 'wasm_memory' is setup in https://github.com/not-fl3/miniquad/blob/master/js/gl.js
    const memory = wasm_memory_holder ? wasm_memory_holder.memory : wasm_memory;

    const f32 = new Float32Array(memory.buffer);
    const u32 = new Uint32Array(memory.buffer);
    const u8 = new Uint8Array(memory.buffer);

    for (const [gamepad_idx, gamepad] of gamepads
      .slice(0, MAX_GAMEPADS)
      .entries()) {
      let byteOffset =
        wasm_memory_offset +
        // Skip gamepads with lower index:
        BYTES_PER_GAMEPAD * gamepad_idx +
        // Skip the initial u8 gamepad id:
        1;

      if (!gamepad || !gamepad.connected || gamepad.mapping !== "standard") {
        u8[byteOffset] = 0;
        continue;
      }

      // Mark connected.
      u8[byteOffset++] = 1;

      // Record number of buttons and axes
      u8[byteOffset++] = gamepad.buttons.length;
      u8[byteOffset++] = gamepad.axes.length;

      // Write u32, pressed_bits:
      let pressed_bits = 0;
      for (const [index, button] of gamepad.buttons.entries()) {
        if (index < 17 && button.pressed) pressed_bits |= 1 << index;
      }
      u32[byteOffset / 4] = pressed_bits;
      byteOffset += 4;

      for (const [index, axes] of gamepad.axes.slice(0, 6).entries()) {
        if (index < 4) {
          // Joysticks
          const sign = index === 1 || index === 3 ? -1 : 1;
          f32[byteOffset / 4] =
            Math.abs(axes) < DEADZONE
              ? 0.0
              : (sign * (axes - Math.sign(axes) * DEADZONE)) / (1 - DEADZONE);
        } else {
          // Triggers
          if (Math.abs(axes) < 0.05 && f32[byteOffset / 4] < 0.05) {
            // Trigger has not been pressed, is in default half-way value. Ignore
          } else {
            f32[byteOffset / 4] =
              Math.abs(axes) + 1 < DEADZONE
                ? 0.0
                : (axes + 1) * 0.5;
          }
        }
        byteOffset += 4;
      }
    }
  };
  importObject.env.playEffect = (
    gamepadId,
    duration,
    startDelay,
    strongMagnitude,
    weakMagnitude,
  ) => {
    const gamepad = navigator.getGamepads().find((p) => p?.index === gamepadId);
    gamepad?.vibrationActuator?.playEffect("dual-rumble", {
      duration,
      startDelay,
      strongMagnitude,
      weakMagnitude,
    });
  };
}
