impl crate::Gamepads {
    pub fn setup_initially_connected_gilrs(&mut self) {
        let gamepad_ids = self
            .gilrs_instance
            .gamepads()
            .map(|(id, g)| (id, g.is_connected()))
            .collect::<Vec<_>>();
        for (id, connected) in gamepad_ids {
            if let Some(p) = self.find_or_insert(id) {
                self.gamepads[p].connected = connected;
            }
        }
    }

    fn find_or_insert(&mut self, gilrs_gamepad_id: gilrs::GamepadId) -> Option<usize> {
        for i in 0..crate::MAX_GAMEPADS {
            if self.gilrs_gamepad_ids[i] == gilrs_gamepad_id.into() {
                return Some(i);
            }
        }
        if self.num_connected_pads == crate::MAX_GAMEPADS as u8 {
            None
        } else {
            let index = self.num_connected_pads;
            self.num_connected_pads += 1;
            self.gilrs_gamepad_ids[index as usize] = gilrs_gamepad_id.into();
            Some(index as usize)
        }
    }

    pub fn poll_gilrs(&mut self) {
        for gamepad in self.gamepads.iter_mut() {
            gamepad.just_pressed_bits = 0;
        }

        while let Some(gilrs::Event { id, event, .. }) = self.gilrs_instance.next_event() {
            match event {
                gilrs::EventType::Connected => {
                    if let Some(gamepad_idx) = self.find_or_insert(id) {
                        self.gamepads[gamepad_idx].connected = true;

                        for (zone, axis) in [
                            (0, gilrs::Axis::LeftStickX),
                            (1, gilrs::Axis::LeftStickY),
                            (2, gilrs::Axis::RightStickY),
                            (3, gilrs::Axis::RightStickY),
                        ] {
                            if let Some(code) = self.gilrs_instance.gamepad(id).axis_code(axis) {
                                self.deadzones[gamepad_idx][zone] = self
                                    .gilrs_instance
                                    .gamepad(id)
                                    .deadzone(code)
                                    .unwrap_or_default();
                            }
                        }
                    }
                }
                gilrs::EventType::Disconnected => {
                    if let Some(gamepad_idx) = self.find_or_insert(id) {
                        self.gamepads[gamepad_idx].connected = false;
                    }
                }
                gilrs::EventType::ButtonPressed(button, _code) => {
                    if let Some(gamepad_idx) = self.find_or_insert(id) {
                        if let Some(b) = crate::Button::from_gilrs(button) {
                            let bit = 1 << (b as u32);
                            self.gamepads[gamepad_idx].pressed_bits |= bit;
                            self.gamepads[gamepad_idx].just_pressed_bits |= bit;
                        }
                    }
                }
                gilrs::EventType::ButtonReleased(button, _code) => {
                    if let Some(gamepad_idx) = self.find_or_insert(id) {
                        if let Some(b) = crate::Button::from_gilrs(button) {
                            let bit = 1 << (b as u32);
                            self.gamepads[gamepad_idx].pressed_bits &= !bit;
                        }
                    }
                }
                gilrs::EventType::AxisChanged(axis, value, _code) => {
                    if let Some(gamepad_idx) = self.find_or_insert(id) {
                        if let Some(axis_idx) = match axis {
                            gilrs::Axis::LeftStickX => Some(0),
                            gilrs::Axis::LeftStickY => Some(1),
                            gilrs::Axis::RightStickX => Some(2),
                            gilrs::Axis::RightStickY => Some(3),
                            _ => None,
                        } {
                            let deadzone = self.deadzones[gamepad_idx][axis_idx];
                            self.gamepads[gamepad_idx].axes[axis_idx] = if value.abs() < deadzone {
                                // Axis values within deadzone are 0:
                                0.
                            } else {
                                // Adjust so that interval of magnitude is [0.0, 1.0]:
                                value.signum().mul_add(-deadzone, value) / (1. - deadzone)
                            };
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn rumble_gilrs(
        &mut self,
        gamepad_id: crate::GamepadId,
        duration_ms: u32,
        start_delay_ms: u32,
        strong_magnitude: f32,
        weak_magnitude: f32,
    ) {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        // Purge old effects.
        for i in (0..self.playing_ff_effects.len()).rev() {
            if self.playing_ff_effects[i].1 < now_ms {
                self.playing_ff_effects.swap_remove(i);
            }
        }

        let gilrs_gamepad_id = self.gilrs_gamepad_ids[gamepad_id.0 as usize];
        let gilrs_gamepad_id: gilrs::GamepadId = unsafe { std::mem::transmute(gilrs_gamepad_id) };

        let play_for = gilrs::ff::Ticks::from_ms(duration_ms);
        let after = gilrs::ff::Ticks::from_ms(start_delay_ms);
        let scheduling = gilrs::ff::Replay {
            play_for,
            after,
            ..Default::default()
        };

        let strong_magnitude = (f32::from(u16::MAX) * strong_magnitude).round() as u16;
        let weak_magnitude = (f32::from(u16::MAX) * weak_magnitude).round() as u16;

        if let Ok(effect) = gilrs::ff::EffectBuilder::new()
            .add_effect(gilrs::ff::BaseEffect {
                kind: gilrs::ff::BaseEffectType::Strong {
                    magnitude: strong_magnitude,
                },
                scheduling,
                ..Default::default()
            })
            .add_effect(gilrs::ff::BaseEffect {
                kind: gilrs::ff::BaseEffectType::Weak {
                    magnitude: weak_magnitude,
                },
                scheduling,
                ..Default::default()
            })
            .repeat(gilrs::ff::Repeat::For(play_for + after))
            .gamepads(&[gilrs_gamepad_id])
            .finish(&mut self.gilrs_instance)
        {
            if effect.play().is_ok() {
                // Effects stop playing in drop(), so keep a reference.
                let throw_away_at = now_ms + u128::from(duration_ms) + u128::from(start_delay_ms);
                self.playing_ff_effects.push((effect, throw_away_at));
            }
        }
    }
}

impl crate::Button {
    const fn from_gilrs(button: gilrs::Button) -> Option<Self> {
        Some(match button {
            gilrs::Button::South => Self::ActionDown,
            gilrs::Button::East => Self::ActionRight,
            gilrs::Button::West => Self::ActionLeft,
            gilrs::Button::North => Self::ActionUp,
            gilrs::Button::LeftTrigger => Self::FrontLeftUpper,
            gilrs::Button::RightTrigger => Self::FrontRightUpper,
            gilrs::Button::LeftTrigger2 => Self::FrontLeftLower,
            gilrs::Button::RightTrigger2 => Self::FrontRightLower,
            gilrs::Button::Select => Self::LeftCenterCluster,
            gilrs::Button::Start => Self::RightCenterCluster,
            gilrs::Button::LeftThumb => Self::LeftStick,
            gilrs::Button::RightThumb => Self::RightStick,
            gilrs::Button::DPadUp => Self::DPadUp,
            gilrs::Button::DPadDown => Self::DPadDown,
            gilrs::Button::DPadLeft => Self::DPadLeft,
            gilrs::Button::DPadRight => Self::DPadRight,
            gilrs::Button::Mode => Self::Mode,
            // Other:
            _ => {
                return None;
            }
        })
    }
}
