//! Original 640x480 shuttle-cockpit main menu.
//!
//! `COMMON.DLL` bitmap 20001 contains only the empty cockpit shell. The
//! original game assembles the controls below from separate bitmap resources;
//! their geometry and command mapping are recovered in
//! `agent_docs/main-menu-parity.md`.

use egui_macroquad::egui::{
    self, Color32, FontFamily, FontId, Pos2, Rect, Sense, Stroke, Vec2, WidgetInfo, WidgetType,
};
use rebellion_core::dat::GalaxySize;
use rebellion_core::missions::MissionFaction;

use crate::bmp_cache::{resources, BmpCache, DllSource};
use crate::panels::game_setup::Difficulty;

pub const LOGICAL_WIDTH: f32 = 640.0;
pub const LOGICAL_HEIGHT: f32 = 480.0;
const ANIMATION_FPS: f64 = 15.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuControl {
    Easy,
    Intermediate,
    Expert,
    GalaxyLever,
    SmallGalaxy,
    MediumGalaxy,
    LargeGalaxy,
    GameType,
    Empire,
    Alliance,
    LoadOptions,
    Credits,
    Multiplayer,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogicalRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl LogicalRect {
    const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn contains(self, point: Pos2) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

pub const CONTROL_RECTS: &[(MainMenuControl, LogicalRect)] = &[
    (
        MainMenuControl::Easy,
        LogicalRect::new(61.0, 41.0, 51.0, 36.0),
    ),
    (
        MainMenuControl::Intermediate,
        LogicalRect::new(124.0, 40.0, 49.0, 36.0),
    ),
    (
        MainMenuControl::Expert,
        LogicalRect::new(187.0, 41.0, 45.0, 36.0),
    ),
    (
        MainMenuControl::GalaxyLever,
        LogicalRect::new(242.0, 271.0, 44.0, 47.0),
    ),
    (
        MainMenuControl::SmallGalaxy,
        LogicalRect::new(290.0, 293.0, 24.0, 21.0),
    ),
    (
        MainMenuControl::MediumGalaxy,
        LogicalRect::new(326.0, 293.0, 24.0, 21.0),
    ),
    (
        MainMenuControl::LargeGalaxy,
        LogicalRect::new(362.0, 293.0, 24.0, 21.0),
    ),
    (
        MainMenuControl::GameType,
        LogicalRect::new(305.0, 333.0, 42.0, 30.0),
    ),
    (
        MainMenuControl::Empire,
        LogicalRect::new(153.0, 308.0, 62.0, 55.0),
    ),
    (
        MainMenuControl::Alliance,
        LogicalRect::new(437.0, 307.0, 62.0, 55.0),
    ),
    (
        MainMenuControl::LoadOptions,
        LogicalRect::new(67.0, 381.0, 51.0, 61.0),
    ),
    (
        MainMenuControl::Credits,
        LogicalRect::new(411.0, 232.0, 40.0, 37.0),
    ),
    (
        MainMenuControl::Multiplayer,
        LogicalRect::new(459.0, 242.0, 33.0, 28.0),
    ),
    (
        MainMenuControl::Quit,
        LogicalRect::new(536.0, 393.0, 63.0, 64.0),
    ),
];

/// Persistent selections and animation state for the cockpit menu.
#[derive(Debug, Clone)]
pub struct MainMenuState {
    pub difficulty: Difficulty,
    pub galaxy_size: GalaxySize,
    pub headquarters_only: bool,
    hovered: Option<MainMenuControl>,
    hover_started_at: f64,
}

impl Default for MainMenuState {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Easy,
            galaxy_size: GalaxySize::Standard,
            headquarters_only: false,
            hovered: None,
            hover_started_at: 0.0,
        }
    }
}

/// Actions produced by the original cockpit controls.
#[derive(Debug, Clone, PartialEq)]
pub enum MainMenuAction {
    StartGame {
        difficulty: Difficulty,
        faction: MissionFaction,
        galaxy_size: GalaxySize,
        headquarters_only: bool,
    },
    LoadGame,
    Credits,
    Multiplayer,
    Quit,
}

pub fn main_menu_canvas_rect(viewport: Rect) -> Rect {
    let scale = (viewport.width() / LOGICAL_WIDTH)
        .min(viewport.height() / LOGICAL_HEIGHT)
        .max(0.0);
    let size = Vec2::new(LOGICAL_WIDTH * scale, LOGICAL_HEIGHT * scale);
    Rect::from_center_size(viewport.center(), size)
}

pub fn control_rect(canvas: Rect, logical: LogicalRect) -> Rect {
    let scale = canvas.width() / LOGICAL_WIDTH;
    Rect::from_min_size(
        canvas.min + Vec2::new(logical.x * scale, logical.y * scale),
        Vec2::new(logical.width * scale, logical.height * scale),
    )
}

fn logical_pointer(canvas: Rect, pointer: Pos2) -> Option<Pos2> {
    if !canvas.contains(pointer) || canvas.width() <= 0.0 {
        return None;
    }
    let scale = canvas.width() / LOGICAL_WIDTH;
    Some(Pos2::new(
        (pointer.x - canvas.min.x) / scale,
        (pointer.y - canvas.min.y) / scale,
    ))
}

pub fn hit_test(canvas: Rect, pointer: Pos2) -> Option<MainMenuControl> {
    let logical = logical_pointer(canvas, pointer)?;
    CONTROL_RECTS
        .iter()
        .find_map(|(control, rect)| rect.contains(logical).then_some(*control))
}

fn control_label(control: MainMenuControl) -> &'static str {
    match control {
        MainMenuControl::Easy => "Easy difficulty, X-wing",
        MainMenuControl::Intermediate => "Intermediate difficulty, Star Destroyer",
        MainMenuControl::Expert => "Expert difficulty, Death Star",
        MainMenuControl::GalaxyLever => "Cycle galaxy size",
        MainMenuControl::SmallGalaxy => "Small galaxy",
        MainMenuControl::MediumGalaxy => "Medium galaxy",
        MainMenuControl::LargeGalaxy => "Large galaxy",
        MainMenuControl::GameType => "Toggle Standard or Headquarters Only game",
        MainMenuControl::Empire => "Start as the Galactic Empire",
        MainMenuControl::Alliance => "Start as the Rebel Alliance",
        MainMenuControl::LoadOptions => "Load game and options",
        MainMenuControl::Credits => "Credits",
        MainMenuControl::Multiplayer => "Multiplayer",
        MainMenuControl::Quit => "Quit",
    }
}

fn animation_resource(start: u32, count: u32, elapsed: f64) -> u32 {
    start + ((elapsed * ANIMATION_FPS) as u32 % count)
}

fn texture_for(
    control: MainMenuControl,
    state: &MainMenuState,
    hovered: bool,
    elapsed: f64,
) -> u32 {
    match control {
        MainMenuControl::Easy => {
            if state.difficulty == Difficulty::Easy {
                11273
            } else if hovered {
                animation_resource(11061, 30, elapsed)
            } else {
                11061
            }
        }
        MainMenuControl::Intermediate => {
            if state.difficulty == Difficulty::Medium {
                11275
            } else if hovered {
                animation_resource(11091, 30, elapsed)
            } else {
                11091
            }
        }
        MainMenuControl::Expert => {
            if state.difficulty == Difficulty::Hard {
                11274
            } else if hovered {
                animation_resource(11121, 30, elapsed)
            } else {
                11121
            }
        }
        MainMenuControl::GalaxyLever => match state.galaxy_size {
            GalaxySize::Standard => 10001,
            GalaxySize::Large => 10002,
            GalaxySize::Huge => 10003,
        },
        MainMenuControl::SmallGalaxy => 10017,
        MainMenuControl::MediumGalaxy => 10018,
        MainMenuControl::LargeGalaxy => 10019,
        MainMenuControl::GameType => {
            if state.headquarters_only {
                10159
            } else {
                10158
            }
        }
        MainMenuControl::Empire => {
            if hovered {
                animation_resource(11001, 15, elapsed)
            } else {
                10009
            }
        }
        MainMenuControl::Alliance => {
            if hovered {
                animation_resource(11031, 15, elapsed)
            } else {
                10007
            }
        }
        MainMenuControl::LoadOptions => {
            if hovered {
                animation_resource(11151, 30, elapsed)
            } else {
                10005
            }
        }
        MainMenuControl::Credits => {
            if hovered {
                animation_resource(11241, 15, elapsed)
            } else {
                10013
            }
        }
        MainMenuControl::Multiplayer => {
            if hovered && ((elapsed * 5.0) as u32 % 2 == 1) {
                11272
            } else {
                11271
            }
        }
        MainMenuControl::Quit => {
            if hovered {
                animation_resource(11181, 30, elapsed)
            } else {
                10011
            }
        }
    }
}

fn draw_texture(
    painter: &egui::Painter,
    cache: &mut BmpCache,
    ctx: &egui::Context,
    resource_id: u32,
    rect: Rect,
) {
    if let Some(texture) = cache.get(ctx, DllSource::Common, resource_id) {
        painter.image(
            texture.id(),
            rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
    }
}

fn activate(control: MainMenuControl, state: &mut MainMenuState) -> Option<MainMenuAction> {
    match control {
        MainMenuControl::Easy => state.difficulty = Difficulty::Easy,
        MainMenuControl::Intermediate => state.difficulty = Difficulty::Medium,
        MainMenuControl::Expert => state.difficulty = Difficulty::Hard,
        MainMenuControl::GalaxyLever => {
            state.galaxy_size = match state.galaxy_size {
                GalaxySize::Standard => GalaxySize::Large,
                GalaxySize::Large => GalaxySize::Huge,
                GalaxySize::Huge => GalaxySize::Standard,
            }
        }
        MainMenuControl::SmallGalaxy => state.galaxy_size = GalaxySize::Standard,
        MainMenuControl::MediumGalaxy => state.galaxy_size = GalaxySize::Large,
        MainMenuControl::LargeGalaxy => state.galaxy_size = GalaxySize::Huge,
        MainMenuControl::GameType => state.headquarters_only = !state.headquarters_only,
        MainMenuControl::Empire | MainMenuControl::Alliance => {
            return Some(MainMenuAction::StartGame {
                difficulty: state.difficulty,
                faction: if control == MainMenuControl::Alliance {
                    MissionFaction::Alliance
                } else {
                    MissionFaction::Empire
                },
                galaxy_size: state.galaxy_size,
                headquarters_only: state.headquarters_only,
            });
        }
        MainMenuControl::LoadOptions => return Some(MainMenuAction::LoadGame),
        MainMenuControl::Credits => return Some(MainMenuAction::Credits),
        MainMenuControl::Multiplayer => return Some(MainMenuAction::Multiplayer),
        MainMenuControl::Quit => return Some(MainMenuAction::Quit),
    }
    None
}

/// Draw the assembled cockpit and return an action when a control activates.
pub fn draw_main_menu(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    state: &mut MainMenuState,
) -> Option<MainMenuAction> {
    let mut action = None;
    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(Color32::BLACK))
        .show(ctx, |ui| {
            let canvas = main_menu_canvas_rect(ui.max_rect());
            let now = ctx.input(|input| input.time);
            let hovered = ctx
                .input(|input| input.pointer.hover_pos())
                .and_then(|pointer| hit_test(canvas, pointer));
            if hovered != state.hovered {
                state.hovered = hovered;
                state.hover_started_at = now;
            }
            let hover_elapsed = (now - state.hover_started_at).max(0.0);

            if let Some(background) =
                cache.get(ctx, DllSource::Common, resources::common::MAIN_MENU_BG)
            {
                ui.painter().image(
                    background.id(),
                    canvas,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );
            }

            let galaxy_indicator = match state.galaxy_size {
                GalaxySize::Standard => MainMenuControl::SmallGalaxy,
                GalaxySize::Large => MainMenuControl::MediumGalaxy,
                GalaxySize::Huge => MainMenuControl::LargeGalaxy,
            };

            for (control, logical) in CONTROL_RECTS {
                // Only the selected galaxy screen receives the original
                // highlight overlay; the three galaxy images are in 20001.
                let draw_control = !matches!(
                    control,
                    MainMenuControl::SmallGalaxy
                        | MainMenuControl::MediumGalaxy
                        | MainMenuControl::LargeGalaxy
                ) || *control == galaxy_indicator;
                let rect = control_rect(canvas, *logical);
                let response = ui.interact(rect, ui.id().with(*control as u8), Sense::click());
                response.widget_info(|| {
                    WidgetInfo::labeled(WidgetType::Button, true, control_label(*control))
                });
                let keyboard_activation = response.has_focus()
                    && ui.input(|input| {
                        input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
                    });

                if draw_control {
                    let resource_id = texture_for(
                        *control,
                        state,
                        state.hovered == Some(*control),
                        hover_elapsed,
                    );
                    draw_texture(ui.painter(), cache, ctx, resource_id, rect);
                }

                if response.has_focus() {
                    ui.painter().rect_stroke(
                        rect.expand(2.0),
                        1.0,
                        Stroke::new((canvas.width() / LOGICAL_WIDTH).max(1.0), Color32::GOLD),
                        egui::StrokeKind::Outside,
                    );
                }
                // egui expands clickable widgets by its interaction radius. The
                // original menu only activated inside its Win32 control rect.
                let exact_pointer_activation =
                    response.clicked() && state.hovered == Some(*control);
                if exact_pointer_activation || keyboard_activation {
                    action = activate(*control, state);
                }
            }

            let label_rect = control_rect(canvas, LogicalRect::new(271.0, 374.0, 110.0, 14.0));
            let scale = canvas.width() / LOGICAL_WIDTH;
            ui.painter().text(
                label_rect.center(),
                egui::Align2::CENTER_CENTER,
                if state.headquarters_only {
                    "Headquarters Only"
                } else {
                    "Standard Game"
                },
                FontId::new((10.0 * scale).max(8.0), FontFamily::Monospace),
                Color32::from_rgb(80, 255, 80),
            );
        });
    action
}

#[cfg(test)]
mod tests {
    use super::*;

    fn viewport(width: f32, height: f32) -> Rect {
        Rect::from_min_size(Pos2::ZERO, Vec2::new(width, height))
    }

    #[test]
    fn preserves_four_by_three_and_centers_letterbox() {
        let wide = main_menu_canvas_rect(viewport(1280.0, 800.0));
        assert!((wide.width() - 1066.6666).abs() < 0.001);
        assert_eq!(wide.height(), 800.0);
        assert!((wide.min.x - 106.6667).abs() < 0.001);
        assert_eq!(wide.min.y, 0.0);

        let tall = main_menu_canvas_rect(viewport(640.0, 600.0));
        assert_eq!(tall.size(), Vec2::new(640.0, 480.0));
        assert_eq!(tall.min, Pos2::new(0.0, 60.0));
    }

    #[test]
    fn transformed_hit_testing_matches_original_regions() {
        let canvas = main_menu_canvas_rect(viewport(1280.0, 960.0));
        assert_eq!(
            hit_test(canvas, Pos2::new(122.0, 82.0)),
            Some(MainMenuControl::Easy)
        );
        assert_eq!(
            hit_test(canvas, Pos2::new(936.0, 670.0)),
            Some(MainMenuControl::Alliance)
        );
        assert_eq!(hit_test(canvas, Pos2::new(5.0, 5.0)), None);
    }

    #[test]
    fn expert_hit_region_does_not_leak_into_adjacent_pixels() {
        let canvas = Rect::from_min_size(Pos2::ZERO, Vec2::new(640.0, 480.0));

        assert_eq!(hit_test(canvas, Pos2::new(186.9, 59.0)), None);
        assert_eq!(
            hit_test(canvas, Pos2::new(187.0, 59.0)),
            Some(MainMenuControl::Expert)
        );
        assert_eq!(
            hit_test(canvas, Pos2::new(232.0, 59.0)),
            Some(MainMenuControl::Expert)
        );
        assert_eq!(hit_test(canvas, Pos2::new(232.1, 59.0)), None);
    }

    #[test]
    fn original_defaults_and_control_transitions_are_stable() {
        let mut state = MainMenuState::default();
        assert_eq!(state.difficulty, Difficulty::Easy);
        assert_eq!(state.galaxy_size, GalaxySize::Standard);
        assert!(!state.headquarters_only);

        activate(MainMenuControl::GalaxyLever, &mut state);
        assert_eq!(state.galaxy_size, GalaxySize::Large);
        activate(MainMenuControl::Expert, &mut state);
        activate(MainMenuControl::GameType, &mut state);
        assert_eq!(state.difficulty, Difficulty::Hard);
        assert!(state.headquarters_only);

        assert_eq!(
            activate(MainMenuControl::Empire, &mut state),
            Some(MainMenuAction::StartGame {
                difficulty: Difficulty::Hard,
                faction: MissionFaction::Empire,
                galaxy_size: GalaxySize::Large,
                headquarters_only: true,
            })
        );
    }

    #[test]
    fn resource_families_match_binary_mapping() {
        let state = MainMenuState::default();
        assert_eq!(
            texture_for(MainMenuControl::Easy, &state, false, 0.0),
            11273
        );
        assert_eq!(
            texture_for(MainMenuControl::Empire, &state, false, 0.0),
            10009
        );
        assert_eq!(
            texture_for(MainMenuControl::Alliance, &state, true, 0.0),
            11031
        );
        assert_eq!(texture_for(MainMenuControl::Quit, &state, true, 1.0), 11196);
    }
}
