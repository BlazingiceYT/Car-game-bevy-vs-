use bevy::prelude::*;

use crate::car::{Car, NitroState, BASE_MAX_SPEED, NITRO_MAX_SPEED};

#[cfg(target_arch = "wasm32")]
mod js_bridge {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window, js_name = updateHud)]
        pub fn update_hud(speed_kmh: f32, max_speed: f32, nitro_percent: f32, nitro_active: bool);
    }
}

/// Sends live speed/nitro data to the JS-drawn HUD every frame.
pub fn hud_bridge_system(
    keys: Res<ButtonInput<KeyCode>>,
    nitro: Res<NitroState>,
    query: Query<&Car>,
) {
    let Ok(car) = query.single() else { return; };

    let nitro_active = keys.pressed(KeyCode::KeyN) && nitro.amount > 0.0;
    let max_speed_now = if nitro_active { NITRO_MAX_SPEED } else { BASE_MAX_SPEED };

    #[cfg(target_arch = "wasm32")]
    js_bridge::update_hud(car.speed.abs(), max_speed_now, nitro.amount, nitro_active);
}
