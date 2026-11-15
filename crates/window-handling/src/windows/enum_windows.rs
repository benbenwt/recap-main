use std::num::NonZeroIsize;

use windows::{
    Win32::{
        Foundation::{HWND, LPARAM},
        UI::WindowsAndMessaging::{
            EnumWindows, GA_ROOTOWNER, GetAncestor, GetLastActivePopup, IsWindowVisible,
        },
    },
    core::BOOL,
};

impl crate::EnumWindows<super::Window> for super::Window {
    fn enum_windows() -> Vec<super::Window> {
        let mut windows = Vec::new();
        #[allow(unsafe_code)]
        unsafe {
            EnumWindows(
                Some(callback),
                LPARAM(&mut windows as *mut Vec<super::Window> as isize),
            )
            .unwrap();
        }
        windows
    }

    fn forground_window() -> super::Window {
        let hwnd = super::Window::forground_hwnd();
        super::Window::from_hwnd(hwnd)
    }
}

#[allow(unsafe_code)]
unsafe extern "system" fn callback(hwnd: HWND, param1: LPARAM) -> BOOL {
    unsafe {
        // Skip invisible windows and tool windows
        if !is_alt_tab_window(hwnd) {
            return true.into();
        }

        let windows = &mut *(param1.0 as *mut Vec<super::Window>);

        windows.push(super::Window {
            handle: raw_window_handle::Win32WindowHandle::new(
                NonZeroIsize::new(hwnd.0 as isize).unwrap(),
            ),
        });

        true.into()
    }
}

/// Check if the window is an alt-tab window \
/// SAFETY: hwnd must be a valid window handle
#[allow(unsafe_code)]
unsafe fn is_alt_tab_window(hwnd: HWND) -> bool {
    if !unsafe { IsWindowVisible(hwnd) }.as_bool() {
        return false;
    }

    let mut hwnd_try = unsafe { GetAncestor(hwnd, GA_ROOTOWNER) };
    let mut hwnd_walk = HWND::default();
    while hwnd_try != hwnd_walk {
        hwnd_walk = hwnd_try;
        hwnd_try = unsafe { GetLastActivePopup(hwnd_walk) };
        if unsafe { IsWindowVisible(hwnd_try) }.as_bool() {
            break;
        }
    }
    if hwnd_walk != hwnd {
        return false;
    }
    true
}

#[test]
#[tracing_test::traced_test]
fn test_enum_windows() {
    use crate::EnumWindows;
    let windows = super::Window::enum_windows();
    assert!(!windows.is_empty(), "No windows found");
    for window in windows {
        println!("{window:?}");
    }
}

#[test]
#[tracing_test::traced_test]
fn test_forground_window() {
    use crate::EnumWindows;
    let window = super::Window::forground_window();
    assert!(window.handle.hwnd.get() != 0, "No window found");
    println!("{window:?}");
}
