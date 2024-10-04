#![feature(downcast_unchecked)]
mod archive;
mod layer;
mod panel;

#[cfg(feature = "smartstring")]
type SmartString = smartstring::SmartString<smartstring::LazyCompact>;
#[cfg(not(feature = "smartstring"))]
type SmartString = String;

use parking_lot::MutexGuard;

pub use crate::{archive::LogEvent, layer::EguiLayer, panel::LogPanel};

pub fn layer() -> EguiLayer {
    EguiLayer
}
pub fn poll_events() -> MutexGuard<'static, Vec<LogEvent>> {
    archive::LOG_ENTRIES.lock()
}

pub fn show(ctx: &egui::Context, open: &mut bool) -> Option<egui::Response> {
    let window = egui::Window::new("Log")
        .resizable(true)
        .collapsible(true)
        .vscroll(true)
        .open(open);
    show_in(ctx, window)
}

pub fn show_in(ctx: &egui::Context, window: egui::Window<'_>) -> Option<egui::Response> {
    window
        .show(ctx, |ui| {
            ui.add(LogPanel);
        })
        .map(|r| r.response)
}
