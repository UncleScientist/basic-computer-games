use macroquad::prelude::*;

use target::*;

const DIST_FROM_ORIGIN: f32 = 120.0;
const TARGET_TO_DISPLAY: f32 = 1000.0;

enum Entry {
    DegreesX,
    DegreesZ,
    Distance,
}

#[derive(Default)]
struct WeaponSettings {
    degrees_x: f32,
    degrees_z: f32,
    distance: f32,
}

#[macroquad::main("3D")]
async fn main() {
    let mut buf = String::new();
    let mut input_state = Entry::DegreesX;

    let mut dist_from_origin = DIST_FROM_ORIGIN;
    let mut last_mouse_loc = (0f32, 0f32);
    let mut angle = 180f32;
    let mut position = Vec3::new(
        angle.to_radians().sin() * dist_from_origin,
        dist_from_origin * 3.0 / 4.0,
        angle.to_radians().cos() * dist_from_origin,
    );
    let mut grid_size = 1f32;

    let mut last_mouse_wheel = (0f32, 0f32);

    let mut target = Target::new();
    let target_info = target.approx_location();
    let dist_est = target.estimated_distance();
    let mut target_loc: Vec3 = (
        target_info.loc.0 / TARGET_TO_DISPLAY,
        target_info.loc.1 / TARGET_TO_DISPLAY,
        target_info.loc.2 / TARGET_TO_DISPLAY,
    )
        .into();

    let mut weapon_settings = WeaponSettings::default();
    let mut shot_result: Option<FireResult> = None;

    loop {
        clear_background(LIGHTGRAY);

        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::Backspace => {
                    buf.pop();
                }
                KeyCode::Period
                | KeyCode::Key0
                | KeyCode::Key1
                | KeyCode::Key2
                | KeyCode::Key3
                | KeyCode::Key4
                | KeyCode::Key5
                | KeyCode::Key6
                | KeyCode::Key7
                | KeyCode::Key8
                | KeyCode::Key9 => buf.push(key as u8 as char),
                KeyCode::Enter => {
                    if let Ok(num) = buf.parse::<f32>() {
                        match input_state {
                            Entry::DegreesX => {
                                weapon_settings.degrees_x = num;
                                input_state = Entry::DegreesZ;
                            }
                            Entry::DegreesZ => {
                                weapon_settings.degrees_z = num;
                                input_state = Entry::Distance;
                            }
                            Entry::Distance => {
                                weapon_settings.distance = num;
                                input_state = Entry::DegreesX;
                                shot_result = Some(target.fire(
                                    weapon_settings.degrees_x,
                                    weapon_settings.degrees_z,
                                    weapon_settings.distance,
                                ));
                            }
                        }
                    }
                    buf.clear();
                }
                _ => {}
            }
        }

        let wheel = mouse_wheel();
        if wheel != last_mouse_wheel {
            last_mouse_wheel = wheel;
            dist_from_origin = DIST_FROM_ORIGIN.max(dist_from_origin - wheel.1);
            position.y = dist_from_origin * 3.0 / 4.0;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let target = Target::new();
            let target_info = target.approx_location();
            target_loc = (
                target_info.loc.0 / 1000.0,
                target_info.loc.1 / 1000.0,
                target_info.loc.2 / 1000.0,
            )
                .into();
        }

        if is_mouse_button_down(MouseButton::Left) {
            let cur_mouse_loc = mouse_position();
            angle += last_mouse_loc.0 - cur_mouse_loc.0;
        }
        last_mouse_loc = mouse_position();
        position.x = angle.to_radians().sin() * dist_from_origin;
        position.z = angle.to_radians().cos() * dist_from_origin;

        set_camera(&Camera3D {
            position,
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        let new_grid_size = 1f32.max((dist_from_origin / 400.0).round() * 20.0);
        if grid_size != new_grid_size {
            grid_size = new_grid_size;
        }
        draw_grid(20, grid_size, BLACK, GRAY);

        if let Some(ref fire_result) = shot_result {
            match fire_result {
                FireResult::SelfDestructed => {}
                FireResult::Miss(Explosion { coords, .. }) => {
                    draw_line_3d(
                        vec3(0., 0., 0.),
                        vec3(
                            coords.0 / TARGET_TO_DISPLAY,
                            coords.1 / TARGET_TO_DISPLAY,
                            coords.2 / TARGET_TO_DISPLAY,
                        ),
                        RED,
                    );
                }
                FireResult::Hit(_, _) => {}
            };
        };

        draw_sphere(target_loc, 20000.0 / TARGET_TO_DISPLAY, None, BLUE);

        // Back to screen space, render some text

        set_default_camera();
        draw_text(
            &format!(
                "Radians from x axis = {}, from z axis = {}, dist = {}",
                target_info.radians_x, target_info.radians_z, dist_est,
            ),
            10.0,
            20.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!(
                "{}? {}",
                match input_state {
                    Entry::DegreesX => "Angle X in degrees",
                    Entry::DegreesZ => "Angle Z in degrees",
                    Entry::Distance => "Distance",
                },
                buf
            ),
            10.0,
            35.0,
            15.0,
            BLACK,
        );

        next_frame().await
    }
}
