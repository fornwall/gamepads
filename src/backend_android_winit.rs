//use winit::platform::android::activity::input::{InputEvent, Source};
use winit::event::{Event, WindowEvent};
use winit::keyboard::{Key, NativeKey};

impl crate::Gamepads {
    pub fn on_event<T>(&mut self, event: &Event<T>) {
        if let Event::WindowEvent {
            event: ref window_event,
            ..
        } = event
        {
            match window_event {
                WindowEvent::KeyboardInput {
                    event: key_event,
                    device_id,
                    ..
                } => {
                    log::info!("Gamepad keyboard event {key_event:?} from device {device_id:?}");

                    if let Key::Unidentified(NativeKey::Android(scancode)) = key_event.logical_key {
                        if let Some(gamepad_idx) = self.find_or_insert(*device_id) {
                            let gamepad_button = match scancode {
                            // See https://developer.android.com/develop/ui/views/touch-and-input/game-controllers/controller-input#dpad
                            // Most controllers report hat axis events instead of D-pad presses, but some might:
                            // "Some controllers instead report D-pad presses with a key code. If your game cares about D-pad
                            // presses, you should treat the hat axis events and the D-pad key codes as the same input events"
                            19 /* AKEYCODE_DPAD_UP */ => crate::Button::DPadUp,
                            20 /* AKEYCODE_DPAD_DOWN */ => crate::Button::DPadDown,
                            21 /* AKEYCODE_DPAD_LEFT */ => crate::Button::DPadLeft,
                            22 /* AKEYCODE_DPAD_RIGHT */ => crate::Button::DPadRight,
                            96 /* AKEYCODE_BUTTON_A */ => crate::Button::ActionDown,
                            97 /* AKEYCODE_BUTTON_B */ => crate::Button::ActionRight,
                            99 /* AKEYCODE_BUTTON_X */ => crate::Button::ActionLeft,
                            100 /* AKEYCODE_BUTTON_Y */ => crate::Button::ActionUp,
                            102 /* AKEYCODE_BUTTON_L1 */ => crate::Button::FrontLeftUpper,
                            103 /* AKEYCODE_BUTTON_R1 */ => crate::Button::FrontRightUpper,
                            104 /* AKEYCODE_BUTTON_L2 */ => crate::Button::FrontLeftLower,
                            105 /* AKEYCODE_BUTTON_R2 */ => crate::Button::FrontRightLower,
                            106 /* AKEYCODE_BUTTON_THUMBL */ => crate::Button::LeftStick,
                            107 /* AKEYCODE_BUTTON_THUMBR */ => crate::Button::RightStick,
                            108 /* AKEYCODE_BUTTON_START */ => crate::Button::RightCenterCluster,
                            109 /* AKEYCODE_BUTTON_SELECT */ => crate::Button::LeftCenterCluster,
                            _ => {
                                return;
                            }
                        };
                            let bit = 1 << (gamepad_button as u32);
                            if key_event.state.is_pressed() {
                                self.gamepads[gamepad_idx].pressed_bits |= bit;
                                self.gamepads[gamepad_idx].just_pressed_bits |= bit;
                            } else {
                                self.gamepads[gamepad_idx].pressed_bits &= !bit;
                            }

                            log::error!(
                                "Gamepad button {:?}, device id = {:?}",
                                gamepad_button,
                                device_id,
                            );
                        }
                    }
                }
                WindowEvent::AxisUpdate { device_id, values } => {
                    log::error!("Axis update: {:?}, {:?}", device_id, values);
                    if let Some(gamepad_idx) = self.find_or_insert(*device_id) {
                        for (val, negative_button, positive_button) in [
                            (values[0], crate::Button::DPadLeft, crate::Button::DPadRight),
                            (values[0], crate::Button::DPadUp, crate::Button::DPadDown),
                        ] {
                            let negative_bit = 1 << (negative_button as u32);
                            let posive_bit = 1 << (positive_button as u32);
                            if val < 0. {
                                self.gamepads[gamepad_idx].pressed_bits |= negative_bit;
                                self.gamepads[gamepad_idx].just_pressed_bits |= negative_bit;
                                self.gamepads[gamepad_idx].pressed_bits &= !posive_bit;
                            } else if val > 0. {
                                self.gamepads[gamepad_idx].pressed_bits |= posive_bit;
                                self.gamepads[gamepad_idx].just_pressed_bits |= posive_bit;
                                self.gamepads[gamepad_idx].pressed_bits &= !negative_bit;
                            } else {
                                self.gamepads[gamepad_idx].pressed_bits &=
                                    !(negative_bit | posive_bit);
                            }
                        }

                        self.gamepads[gamepad_idx].axes =
                            [values[2], values[3], values[4], values[5]];
                    }
                }
                WindowEvent::Touch(touch) => {
                    // https://docs.rs/winit/latest/winit/event/struct.Touch.html
                    // Note device_id being present.
                    log::error!("Touch event: {:?}", touch);
                }
                _ => {}
            };
        }
    }

    fn find_or_insert(&mut self, winit_device_id: winit::event::DeviceId) -> Option<usize> {
        for i in 0..crate::MAX_GAMEPADS {
            if self.android_winit_gamepad_ids[i] == winit_device_id {
                return Some(i);
            }
        }
        if self.num_connected_pads == crate::MAX_GAMEPADS as u8 {
            None
        } else {
            let index = self.num_connected_pads;
            self.num_connected_pads += 1;
            self.android_winit_gamepad_ids[index as usize] = winit_device_id;
            Some(index as usize)
        }
    }

    #[allow(clippy::expect_used)]
    pub fn rumble_android(
        &mut self,
        _gamepad_id: crate::GamepadId,
        duration_ms: u32,
        _start_delay_ms: u32,
        strong_magnitude: f32,
        weak_magnitude: f32,
    ) {
        // See https://android.googlesource.com/platform/frameworks/opt/gamesdk/+/refs/heads/main/games-controller/src/main/java/com/google/android/games/paddleboat/GameControllerManager.java
        //
        // See also implementation in chromium:
        // https://chromium-review.googlesource.com/c/chromium/src/+/3721715/12/device/gamepad/android/java/src/org/chromium/device/gamepad/GamepadDevice.java#73
        fn scale_magnitude(magnitude: f32) -> i32 {
            // Vibration magnitudes on android are between 1 and 255
            const VIBRATION_MAX_AMPLITUDE: f32 = 255.;
            (magnitude.clamp(0., 1.) * VIBRATION_MAX_AMPLITUDE).round() as i32
        }

        const STRONG_MAGNITUDE_IDX: i32 = 0;
        const WEAK_MAGNITUDE_IDX: i32 = 1;

        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let mut env = vm.attach_current_thread().unwrap();

        let class = env
            .find_class("android/view/InputDevice")
            .expect("Failed to load the target class");

        // let device_id = self.android_winit_gamepad_ids[gamepad_id.value() as usize];
        let device_id_i32 = 0; /* TODO: expose API in winit, or for now: unsafe { std::mem::transmute(device_id) }; */

        let java_input_device = if let jni::objects::JValueGen::Object(java_input_device) = env
            .call_static_method(
                class,
                "getDevice",
                "(I)Landroid/view/InputDevice",
                &[jni::objects::JValue::Int(device_id_i32)],
            )
            .expect("getDevice failed")
        {
            java_input_device
        } else {
            log::error!("getDevice did not return an object");
            return;
        };

        let vibration_manager = if let jni::objects::JValueGen::Object(vibration_manager) = env
            .call_method(
                java_input_device,
                "getVibratorManager",
                "()Landroid/os/VibratorManager;",
                &[],
            )
            .expect("getVibratorManager failed")
        {
            vibration_manager
        } else {
            log::error!("getVibratorManager did not return an object");
            return;
        };

        let java_vibrator_ids_object =
            if let jni::objects::JValueGen::Object(java_vibrator_ids_object) = env
                .call_method(&vibration_manager, "getVibratorIds", "()[I", &[])
                .expect("getVibratorIds failed")
            {
                java_vibrator_ids_object
            } else {
                log::error!("getVibratorIds did not return an object");
                return;
            };
        let java_vibrator_ids_array = jni::objects::JIntArray::from(java_vibrator_ids_object);

        let num_vibrators = env.get_array_length(&java_vibrator_ids_array).unwrap();
        if num_vibrators < 2 {
            log::warn!("Too few vibrators {num_vibrators}");
            return;
        }

        // https://chromium-review.googlesource.com/c/chromium/src/+/3721715/12/device/gamepad/android/java/src/org/chromium/device/gamepad/GamepadDevice.java#275
        // TODO: Check for hasAmplitudeControl() on both vibrators?

        let vibration_effect_class = env
            .find_class("android/os/VibrationEffect")
            .expect("Failed to load the android/os/VibrationEffect class");

        let combined_vibration_class = env
            .find_class("android/os/CombinedVibration")
            .expect("Failed to load the android/os/CombinedVibration class");

        let parallel_combination = if let jni::objects::JValueGen::Object(parallel_combination) =
            env.call_static_method(
                combined_vibration_class,
                "startParallel",
                "()Landroid/os/CombinedVibration#ParallelCombination",
                &[],
            )
            .expect("startParallel failed")
        {
            parallel_combination
        } else {
            log::error!("startParallel did not return an object");
            return;
        };

        let mut add_vibrator = |vibrator_idx, magnitude| {
            // public static VibrationEffect createOneShot (long milliseconds, int amplitude)
            // https://developer.android.com/reference/android/os/VibrationEffect#createOneShot(long,%20int)
            let vibration_effect = if let jni::objects::JValueGen::Object(vibration_effect) = env
                .call_static_method(
                    &vibration_effect_class,
                    "createOneShot",
                    "(JI)Landroid/os/VibrationEffect",
                    &[
                        jni::objects::JValue::Long(i64::from(duration_ms)),
                        jni::objects::JValue::Int(magnitude),
                    ],
                )
                .expect("createOneShot failed")
            {
                vibration_effect
            } else {
                log::error!("createOneShot did not return an object");
                return;
            };

            // public CombinedVibration.ParallelCombination addVibrator (int vibratorId, VibrationEffect effect)
            // https://developer.android.com/reference/android/os/CombinedVibration.ParallelCombination#addVibrator(int,%20android.os.VibrationEffect)
            env.call_method(
                &parallel_combination,
                "addVibrator",
                "(ILandroid/os/VibrationEffect;)V",
                &[
                    jni::objects::JValue::Int(vibrator_idx),
                    jni::objects::JValue::Object(&vibration_effect),
                ],
            )
            .expect("addVibrator failed");
        };
        let strong = scale_magnitude(strong_magnitude);
        if strong > 0 {
            // effect.addVibrator(0, VibrationEffect.createOneShot(durationMillis, strongMagnitude));
            add_vibrator(WEAK_MAGNITUDE_IDX, strong);
        }
        let weak = scale_magnitude(weak_magnitude);
        if weak > 0 {
            // effect.addVibrator(1, VibrationEffect.createOneShot(durationMillis, strongMagnitude));
            add_vibrator(STRONG_MAGNITUDE_IDX, weak);
        }

        // TODO: Verify early that one of strong > 0, weak > 0 is true.

        // var combined = effect.combine();
        let combined_vibration = if let jni::objects::JValueGen::Object(object) = env
            .call_method(&parallel_combination, "combine", "()V", &[])
            .expect("effect.combine() failed")
        {
            object
        } else {
            log::error!("combine() did not return an object");
            return;
        };

        // vibratorManager.vibrate(combined);
        env.call_method(
            vibration_manager,
            "vibrate",
            "(L/android/os/CombinedVibration)V",
            &[jni::objects::JValue::Object(&combined_vibration)],
        )
        .expect("vibrate failed");
    }
}
