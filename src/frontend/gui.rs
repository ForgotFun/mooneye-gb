use glium::Surface;
use glium::backend::Facade;
use imgui::{ImGui, ImGuiSetCond_Always, ImStr, ImVec4, Ui};
use imgui::glium_renderer::{Renderer, RendererError};
use sdl2::mouse::{MouseUtil};
use std::f32;
use time::Duration;

use super::{FrontendError, FrontendResult};

impl From<RendererError> for FrontendError {
  fn from(e: RendererError) -> FrontendError {
    FrontendError::Renderer(format!("{}", e))
  }
}

trait Scene {
  fn render(&mut self, ui: &Ui);
}

pub struct Gui {
  imgui: ImGui,
  renderer: Renderer
}

impl Gui {
  pub fn init<F: Facade>(ctx: &F) -> FrontendResult<Gui> {
    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);
    let renderer = try!(Renderer::init(&mut imgui, ctx));
    Ok(Gui {
      imgui: imgui,
      renderer: renderer
    })
  }
  pub fn render<S: Surface, T: Scene>(&mut self, surface: &mut S,
                                      delta: Duration, mouse: &MouseUtil,
                                      scene: &mut T) -> FrontendResult<()> {
    let delta_s = delta.num_nanoseconds().unwrap() as f32 / 1_000_000_000.0;
    let (width, height) = surface.get_dimensions();
    self.imgui.update_mouse(mouse);

    {
      let ui = self.imgui.frame(width, height, delta_s);
      scene.render(&ui);
      try!(self.renderer.render(surface, ui));
    }

    Ok(())
  }
}

#[derive(Default)]
pub struct WaitBootromScene {
  error: Option<ImStr<'static>>
}

impl WaitBootromScene {
  pub fn set_error(&mut self, text: String) {
    self.error = Some(text.into());
  }
}

impl Scene for WaitBootromScene {
  fn render(&mut self, ui: &Ui) {
    ui.window()
      .name(im_str!("Help overlay"))
      .title_bar(false)
      .resizable(false)
      .movable(false)
      .always_auto_resize(true)
      .position((f32::MIN, f32::MIN), ImGuiSetCond_Always)
      .build(|| {
        ui.text(im_str!("Mooneye GB requires a boot ROM to run"));
        ui.text(im_str!("Drag and drop here a boot rom of one of these types:"));
        ui.bullet_text(im_str!("Game Boy (usually called dmg_boot.bin)"));
        ui.bullet_text(im_str!("Game Boy Pocket (usually called mgb_boot.bin)"));

        if let Some(ref error) = self.error {
          ui.separator();
          ui.text_colored(ImVec4::new(1.0, 0.0, 0.0, 1.0), error.clone());
        }
      });
  }
}

pub struct WaitRomScene {
  title: ImStr<'static>,
  error: Option<ImStr<'static>>
}

impl WaitRomScene {
  pub fn new() -> WaitRomScene {
    WaitRomScene {
      title: im_str!("Mooneye GB v{}", ::VERSION),
      error: None
    }
  }
  pub fn set_error(&mut self, text: String) {
    self.error = Some(text.into());
  }
}

impl Scene for WaitRomScene {
  fn render(&mut self, ui: &Ui) {
    ui.window()
      .name(im_str!("Help overlay"))
      .title_bar(false)
      .resizable(false)
      .movable(false)
      .always_auto_resize(true)
      .position((f32::MIN, f32::MIN), ImGuiSetCond_Always)
      .build(|| {
        ui.text(self.title.clone());
        ui.separator();
        ui.text(im_str!("Drag and drop a Game Boy ROM file here to load it"));

        if let Some(ref error) = self.error {
          ui.separator();
          ui.text_colored(ImVec4::new(1.0, 0.0, 0.0, 1.0), error.clone());
        }
      });
  }
}

#[derive(Default)]
pub struct InGameScene {
  pub fps: f64,
  pub perf: f64,
  show_perf_overlay: bool
}

impl InGameScene {
  pub fn toggle_perf_overlay(&mut self) {
    self.show_perf_overlay = !self.show_perf_overlay;
  }
}

impl Scene for InGameScene {
  fn render(&mut self, ui: &Ui) {
    if self.show_perf_overlay {
      ui.window()
        .name(im_str!("Performance overlay"))
        .bg_alpha(0.4)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .always_auto_resize(true)
        .position((0.0, 0.0), ImGuiSetCond_Always)
        .build(|| {
          ui.text(im_str!("FPS: {:.0}, speed: {:.0} %", self.fps, self.perf));
        });
    }
  }
}