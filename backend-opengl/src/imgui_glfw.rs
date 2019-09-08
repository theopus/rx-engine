    /// Use the reexported imgui crate to avoid version conflicts.
extern crate imgui;

use std::os::raw::{c_char, c_void};
use std::time::Instant;

/// Use the reexported glfw crate to avoid version conflicts.
pub use glfw;
use glfw::{Action, Key, Modifiers, MouseButton, StandardCursor, Window, WindowEvent};
use glfw::ffi::GLFWwindow;
use imgui::ImGui;
use imgui::Key as ImGuiKey;
use imgui::MouseCursor as ImGuiMouseCursor;
use imgui::sys as imgui_sys;

static mut WINDOW: *mut c_void = std::ptr::null_mut() as *mut c_void;

pub struct ImguiGLFW {
    last_frame: Instant,
    mouse_press: [bool; 5],
    cursor_pos: (f64, f64),
    cursor: (ImGuiMouseCursor, Option<StandardCursor>),
}

impl ImguiGLFW {
    pub fn new(imgui: &mut ImGui, window: &glfw::Window) -> Self {
        unsafe {
            WINDOW = glfw::ffi::glfwGetCurrentContext() as *mut c_void;
        }

        {
            let window_size = window.get_size();
            let dpi_factor = window.get_framebuffer_size().0 / window_size.0;
            let io = imgui.io_mut();
            io.display_size = [window_size.0 as f32, window_size.1 as f32];
            io.display_framebuffer_scale = [dpi_factor as f32, dpi_factor as f32];
        }

        {
            imgui.set_imgui_key(ImGuiKey::Tab, Key::Tab as u8);
            imgui.set_imgui_key(ImGuiKey::LeftArrow, Key::Left as u8);
            imgui.set_imgui_key(ImGuiKey::RightArrow, Key::Right as u8);
            imgui.set_imgui_key(ImGuiKey::UpArrow, Key::Up as u8);
            imgui.set_imgui_key(ImGuiKey::DownArrow, Key::Down as u8);
            imgui.set_imgui_key(ImGuiKey::PageUp, Key::PageUp as u8);
            imgui.set_imgui_key(ImGuiKey::PageDown, Key::PageDown as u8);
            imgui.set_imgui_key(ImGuiKey::Home, Key::Home as u8);
            imgui.set_imgui_key(ImGuiKey::End, Key::End as u8);
            imgui.set_imgui_key(ImGuiKey::Delete, Key::Delete as u8);
            imgui.set_imgui_key(ImGuiKey::Backspace, Key::Backspace as u8);
            imgui.set_imgui_key(ImGuiKey::Enter, Key::Enter as u8);
            imgui.set_imgui_key(ImGuiKey::Escape, Key::Escape as u8);
            imgui.set_imgui_key(ImGuiKey::A, Key::A as u8);
            imgui.set_imgui_key(ImGuiKey::C, Key::C as u8);
            imgui.set_imgui_key(ImGuiKey::V, Key::V as u8);
            imgui.set_imgui_key(ImGuiKey::X, Key::X as u8);
            imgui.set_imgui_key(ImGuiKey::Y, Key::Y as u8);
            imgui.set_imgui_key(ImGuiKey::Z, Key::Z as u8);
        }

        Self {
            last_frame: Instant::now(),
            mouse_press: [false; 5],
            cursor_pos: (0., 0.),
            cursor: (ImGuiMouseCursor::Arrow, None),
        }
    }

    pub fn handle_event(&mut self, imgui: &mut ImGui, event: &WindowEvent) {
        match *event {
            WindowEvent::MouseButton(mouse_btn, action, _) => {
                let index = match mouse_btn {
                    MouseButton::Button1 => 0,
                    MouseButton::Button2 => 1,
                    MouseButton::Button3 => 2,
                    MouseButton::Button4 => 3,
                    MouseButton::Button5 => 4,
                    _ => 0,
                };
                let press = action != Action::Release;
                self.mouse_press[index] = press;
                imgui.set_mouse_down(self.mouse_press);
            }
            WindowEvent::CursorPos(w, h) => {
                imgui.set_mouse_pos(w as f32, h as f32);
                self.cursor_pos = (w, h);
            }
            WindowEvent::Scroll(_, d) => {
                imgui.set_mouse_wheel(d as f32);
            }
            WindowEvent::Char(character) => {
                imgui.add_input_character(character);
            }
            WindowEvent::Key(key, _, action, modifier) => {
                Self::set_mod(imgui, modifier);
                if action != Action::Release {
                    imgui.set_key(key as u8, true);
                } else {
                    imgui.set_key(key as u8, false);
                }
            }
            _ => {}
        }
    }

    pub fn prepare_render(&self,
                          ui: &imgui::Ui,
                          window: &mut Window,) {
        match ui.mouse_cursor() {
            None => {}
            Some(cursor) => {
                let glfw_cusor = match cursor {
                    ImGuiMouseCursor::Arrow => StandardCursor::Arrow,
                    ImGuiMouseCursor::TextInput => StandardCursor::IBeam,
                    ImGuiMouseCursor::ResizeAll => StandardCursor::Arrow,
                    ImGuiMouseCursor::ResizeNS => StandardCursor::VResize,
                    ImGuiMouseCursor::ResizeEW => StandardCursor::HResize,
                    ImGuiMouseCursor::ResizeNESW => StandardCursor::Arrow,
                    ImGuiMouseCursor::ResizeNWSE => StandardCursor::Arrow,
                    ImGuiMouseCursor::Hand => StandardCursor::Hand,
                };

                window.set_cursor(Some(glfw::Cursor::standard(glfw_cusor)));
            }
        }
    }

    pub fn frame<'a>(&mut self, window: &mut Window, imgui: &'a mut ImGui) -> imgui::Ui<'a> {
//        let mouse_cursor = imgui.mouse_cursor();
//        if imgui.mouse_draw_cursor() || mouse_cursor == ImGuiMouseCursor::None {
//            self.cursor = (ImGuiMouseCursor::None, None);
//            window.set_cursor(None);
//        } else if mouse_cursor != self.cursor.0 {
//            let cursor = match mouse_cursor {
//                ImGuiMouseCursor::None => unreachable!("mouse_cursor was None!"),
//                ImGuiMouseCursor::Arrow => StandardCursor::Arrow,
//                ImGuiMouseCursor::TextInput => StandardCursor::IBeam,
//                ImGuiMouseCursor::ResizeAll => StandardCursor::Arrow,
//                ImGuiMouseCursor::ResizeNS => StandardCursor::VResize,
//                ImGuiMouseCursor::ResizeEW => StandardCursor::HResize,
//                ImGuiMouseCursor::ResizeNESW => StandardCursor::Arrow,
//                ImGuiMouseCursor::ResizeNWSE => StandardCursor::Arrow,
//                ImGuiMouseCursor::Hand => StandardCursor::Hand,
//            };
//
//            window.set_cursor(Some(glfw::Cursor::standard(cursor)));
//        }


        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        let window_size = window.get_size();
        let dpi_factor = window.get_framebuffer_size().0 / window_size.0;
        let io = imgui.io_mut();
        io.display_size = [window_size.0 as f32, window_size.1 as f32];
        io.display_framebuffer_scale = [dpi_factor as f32, dpi_factor as f32];
        io.delta_time = delta_s;
        imgui.frame()
    }

    fn set_mod(imgui: &mut ImGui, modifier: Modifiers) {
        imgui.set_key_ctrl(modifier.intersects(Modifiers::Control));
        imgui.set_key_alt(modifier.intersects(Modifiers::Alt));
        imgui.set_key_shift(modifier.intersects(Modifiers::Shift));
        imgui.set_key_super(modifier.intersects(Modifiers::Super));
    }
}

#[doc(hidden)]
pub extern "C" fn get_clipboard_text(_user_data: *mut c_void) -> *const c_char {
    unsafe { glfw::ffi::glfwGetClipboardString(WINDOW as *mut GLFWwindow) }
}

#[doc(hidden)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::not_unsafe_ptr_arg_deref))]
pub extern "C" fn set_clipboard_text(_user_data: *mut c_void, text: *const c_char) {
    unsafe {
        glfw::ffi::glfwSetClipboardString(WINDOW as *mut GLFWwindow, text);
    }
}
