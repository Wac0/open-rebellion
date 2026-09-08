//! Cockpit frame rendering — the chrome border around the galaxy map.
//!
//! Renders a faction-specific decorative border that frames the galaxy map
//! area, replicating the "cockpit" aesthetic of the original game's strategy
//! view.  When BMP assets are staged (`data/base/ui/`), the background texture
//! is loaded via `BmpCache`; otherwise a styled fallback is drawn using theme
//! colors and macroquad primitives.
//!
//! # Screen layout
//!
//! ```text
//! ┌──────────────────────────────────────────────────┐
//! │  [top bar: faction logo + status]                │
//! │                                                  │
//! │  ┌────────────────────────────────────────────┐  │
//! │  │                                            │  │
//! │  │         GALAXY MAP VIEWPORT                │  │
//! │  │                                            │  │
//! │  └────────────────────────────────────────────┘  │
//! │                                                  │
//! │  [bottom bar: cockpit control buttons]           │
//! └──────────────────────────────────────────────────┘
//! ```
//!
//! The returned `CockpitViewport` tells `draw_galaxy_map` exactly which
//! rectangle to render into.
//!
//! # BMP resource IDs
//!
//! | DLL | ID | Content |
//! |-----|----|---------|
//! | STRATEGY | 900 | Galaxy map starfield background (640×481) |
//! | COMMON | 20001 | Main-menu background (640×480) |
//! | COMMON | 11001-11275 | Animated cockpit display sequences, not a sequential logical-button map |
//!
//! Alliance-specific cockpit elements are distinguished in the original game
//! by color palettes applied at render time.  We approximate with `ALLIANCE_BLUE`
//! vs `EMPIRE_RED` accents drawn over a shared chrome layout.

use egui_macroquad::egui::{self, Ui};
use macroquad::prelude::*;

use crate::bmp_cache::{resources, BmpCache, DllSource};
use crate::theme;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Which player faction owns this cockpit chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CockpitFaction {
    Alliance,
    Empire,
}

/// Cockpit button identifiers.
///
/// These correspond to the nine main strategy-view control buttons in the
/// original game.  Keyboard shortcuts are listed as fallbacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CockpitButton {
    /// Officers panel (O)
    Officers,
    /// Fleets panel (F)
    Fleets,
    /// Manufacturing panel (M)
    Manufacturing,
    /// Missions panel (N)
    Missions,
    /// Research panel (T)
    Research,
    /// Encyclopedia (E)
    Encyclopedia,
    /// Save / Load
    SaveLoad,
    /// Speed: decrease
    SpeedDown,
    /// Speed: increase
    SpeedUp,
}

/// Pixel viewport the galaxy map should render into.
///
/// All coordinates are in macroquad screen pixels.
#[derive(Debug, Clone, Copy)]
pub struct CockpitViewport {
    /// Left edge of the usable map area (pixels from left).
    pub x: f32,
    /// Top edge of the usable map area (pixels from top).
    pub y: f32,
    /// Width of the usable map area in pixels.
    pub width: f32,
    /// Height of the usable map area in pixels.
    pub height: f32,
}

impl CockpitViewport {
    /// Viewport that fills the entire screen (no cockpit chrome).
    pub fn fullscreen() -> Self {
        CockpitViewport {
            x: 0.0,
            y: 0.0,
            width: screen_width(),
            height: screen_height(),
        }
    }
}

/// All mutable state owned by the cockpit module.
pub struct CockpitState {
    /// Which faction's chrome to render.
    pub faction: CockpitFaction,
    /// Height of the top decorative bar in pixels.
    pub top_bar_h: f32,
    /// Height of the bottom button bar in pixels.
    pub bottom_bar_h: f32,
    /// Side gutters width in pixels (equal left/right).
    pub side_gutter_w: f32,
}

impl Default for CockpitState {
    fn default() -> Self {
        CockpitState {
            faction: CockpitFaction::Alliance,
            top_bar_h: 32.0,
            bottom_bar_h: 40.0,
            side_gutter_w: 0.0, // no side gutters for now — full width
        }
    }
}

impl CockpitState {
    pub fn new(faction: CockpitFaction) -> Self {
        CockpitState {
            faction,
            ..Default::default()
        }
    }

    /// Compute the galaxy map viewport given the current screen size.
    ///
    /// The returned `CockpitViewport` is the rectangle that callers should
    /// pass to `draw_galaxy_map` as its clipping region.
    pub fn galaxy_viewport(&self) -> CockpitViewport {
        let sw = screen_width();
        let sh = screen_height();
        CockpitViewport {
            x: self.side_gutter_w,
            y: self.top_bar_h,
            width: sw - self.side_gutter_w * 2.0,
            height: sh - self.top_bar_h - self.bottom_bar_h,
        }
    }
}

// ---------------------------------------------------------------------------
// Draw functions
// ---------------------------------------------------------------------------

/// Draw the top and bottom cockpit chrome bars using macroquad.
///
/// Call before `egui_macroquad::ui` so the chrome renders beneath egui panels.
/// Returns the viewport reserved for the galaxy map.
pub fn draw_cockpit_chrome(state: &CockpitState) -> CockpitViewport {
    let sw = screen_width();
    let sh = screen_height();

    let faction_color: Color = if state.faction == CockpitFaction::Alliance {
        Color::new(0.15, 0.35, 0.65, 1.0) // deep Alliance blue
    } else {
        Color::new(0.55, 0.10, 0.10, 1.0) // deep Empire crimson
    };
    let accent_color: Color = if state.faction == CockpitFaction::Alliance {
        Color::new(0.4, 0.6, 1.0, 1.0)
    } else {
        Color::new(1.0, 0.35, 0.35, 1.0)
    };

    // ── Top bar ──────────────────────────────────────────────────────────────
    draw_rectangle(0.0, 0.0, sw, state.top_bar_h, faction_color);
    // Thin accent line at bottom of top bar
    draw_rectangle(0.0, state.top_bar_h - 2.0, sw, 2.0, accent_color);

    // Faction label
    let label = if state.faction == CockpitFaction::Alliance {
        "REBEL ALLIANCE — COMMAND BRIDGE"
    } else {
        "GALACTIC EMPIRE — COMMAND BRIDGE"
    };
    let font_size = 14.0;
    let dims = measure_text(label, None, font_size as u16, 1.0);
    draw_text(
        label,
        (sw - dims.width) / 2.0,
        state.top_bar_h * 0.72,
        font_size,
        Color::new(0.9, 0.85, 0.6, 1.0),
    );

    // ── Bottom bar ───────────────────────────────────────────────────────────
    let bottom_y = sh - state.bottom_bar_h;
    draw_rectangle(0.0, bottom_y, sw, state.bottom_bar_h, faction_color);
    // Thin accent line at top of bottom bar
    draw_rectangle(0.0, bottom_y, sw, 2.0, accent_color);

    state.galaxy_viewport()
}

/// Draw the faction's authentic STRATEGY.DLL cockpit frame as the first egui
/// layer of the frame. Panels rendered afterward remain readable above it.
pub fn draw_cockpit_background(ctx: &egui::Context, state: &CockpitState, cache: &mut BmpCache) {
    let background_id = if state.faction == CockpitFaction::Alliance {
        resources::strategy::GALAXY_BACKGROUND
    } else {
        resources::strategy::GALAXY_BACKGROUND_EMPIRE
    };
    let Some(texture) = cache.get(ctx, DllSource::Strategy, background_id) else {
        return;
    };

    // `SidePanel` paints on egui's canonical background layer. Painting the
    // cockpit into a separate `Order::Background` layer can still place that
    // layer above side panels, depending on egui's area ordering. Use the same
    // canonical layer instead: this shape is appended first, then panels append
    // their frames, text, and bitmaps over it later in the frame.
    let painter = ctx.layer_painter(egui::LayerId::background());
    painter.image(
        texture.id(),
        egui::Rect::from_min_max(
            egui::Pos2::ZERO,
            egui::pos2(screen_width(), screen_height()),
        ),
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}

/// Draw egui-layer cockpit elements: control button bar.
///
/// Call inside `egui_macroquad::ui(|ctx| { ... })`.
///
/// Returns the `CockpitButton` that was clicked this frame, if any.
pub fn draw_cockpit_egui_layer(
    ctx: &egui::Context,
    state: &CockpitState,
    _cache: &mut BmpCache,
    // Panel visibility flags so buttons show active state
    show_officers: bool,
    show_fleets: bool,
    show_manufacturing: bool,
    show_missions: bool,
    show_research: bool,
    enc_open: bool,
) -> Option<CockpitButton> {
    let sw = screen_width();
    let sh = screen_height();
    let bottom_y = sh - state.bottom_bar_h;

    let mut clicked: Option<CockpitButton> = None;

    // ── Bottom button bar ────────────────────────────────────────────────────
    // Place an egui panel anchored to the bottom of the screen, matching the
    // macroquad-drawn chrome bar.
    egui::Area::new(egui::Id::new("cockpit_buttons"))
        .fixed_pos(egui::pos2(0.0, bottom_y + 2.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            ui.set_width(sw);
            ui.set_height(state.bottom_bar_h - 2.0);

            ui.horizontal_centered(|ui| {
                ui.add_space(8.0);

                let faction_active = if state.faction == CockpitFaction::Alliance {
                    theme::ALLIANCE_BLUE
                } else {
                    theme::EMPIRE_RED
                };

                // The extracted 11001–11275 resources are animated cockpit
                // sequences, not one logical button per numeric triplet. Until
                // the original command-to-sequence table is resolved, use a
                // clear functional label instead of displaying unrelated art.
                let control_btn =
                    |ui: &mut Ui, label: &str, key: &str, active: bool| -> bool {
                        let text = format!("{}\n[{}]", label, key);
                        let rt = egui::RichText::new(text).size(9.0).color(if active {
                            faction_active
                        } else {
                            theme::TEXT_SECONDARY
                        });
                        ui.add(egui::Button::new(rt).min_size(egui::vec2(52.0, 32.0)).fill(
                            if active {
                                egui::Color32::from_rgba_unmultiplied(30, 60, 120, 200)
                            } else {
                                egui::Color32::from_rgba_unmultiplied(10, 15, 30, 200)
                            },
                        ))
                        .on_hover_text(format!("{} [{}]", label, key))
                        .clicked()
                    };

                // Main panel buttons (Officers → Encyclopedia)
                let buttons: &[(CockpitButton, &str, &str, bool)] = &[
                    (CockpitButton::Officers, "Officers", "O", show_officers),
                    (CockpitButton::Fleets, "Fleets", "F", show_fleets),
                    (CockpitButton::Manufacturing, "Mfg", "M", show_manufacturing),
                    (CockpitButton::Missions, "Missions", "N", show_missions),
                    (CockpitButton::Research, "Research", "T", show_research),
                    (CockpitButton::Encyclopedia, "Encyclopedia", "E", enc_open),
                ];
                for &(btn_id, label, key, active) in buttons {
                    if control_btn(ui, label, key, active) {
                        clicked = Some(btn_id);
                    }
                }

                ui.add_space(16.0);

                if control_btn(ui, "Save/Load", "S", false) {
                    clicked = Some(CockpitButton::SaveLoad);
                }

                ui.add_space(16.0);

                if control_btn(ui, "Slower", "<", false) {
                    clicked = Some(CockpitButton::SpeedDown);
                }
                if control_btn(ui, "Faster", ">", false) {
                    clicked = Some(CockpitButton::SpeedUp);
                }
            });
        });

    clicked
}
