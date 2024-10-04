mod archive;
mod layer;
mod panel;

#[cfg(feature = "smartstring")]
type SmartString = smartstring::SmartString<smartstring::LazyCompact>;
#[cfg(not(feature = "smartstring"))]
type SmartString = String;

pub use crate::{layer::EguiLayer, panel::LogPanel};

pub fn layer() -> EguiLayer {
    EguiLayer::new()
}

pub fn show(ctx: &egui::CtxRef, open: &mut bool) -> Option<egui::Response> {
    let window = egui::Window::new("Log")
        .resizable(true)
        .collapsible(true)
        .scroll(true)
        .open(open);
    show_in(ctx, window)
}

pub fn show_in(ctx: &egui::CtxRef, window: egui::Window<'_>) -> Option<egui::Response> {
    window.show(ctx, |ui| {
        ui.add(LogPanel);
    })
}
