mod enum_windows;

use std::fmt::Debug;

use raw_window_handle::HasWindowHandle;
use windows::{
    Win32::{
        Foundation::{CloseHandle, GetLastError, HWND, MAX_PATH, RECT, SetLastError, WIN32_ERROR},
        Graphics::Gdi::{
            DEVMODEW, ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_FLAGS, EnumDisplaySettingsExW,
            GetMonitorInfoW, HMONITOR, MONITOR_DEFAULTTONEAREST, MONITORINFO, MONITORINFOEXW,
            MonitorFromWindow,
        },
        System::{
            ProcessStatus::GetModuleFileNameExW,
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        },
        UI::{
            HiDpi::{GetDpiForMonitor, GetDpiForWindow, MDT_EFFECTIVE_DPI},
            WindowsAndMessaging::{
                GetForegroundWindow, GetWindowRect, GetWindowTextW, GetWindowThreadProcessId,
            },
        },
    },
    core::PCWSTR,
};

use crate::MonitorError;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Window {
    pub handle: raw_window_handle::Win32WindowHandle,
}

impl Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("handle", &self.handle)
            .field("title", &crate::WindowInfo::title(self))
            .finish()
    }
}

impl Window {
    fn from_hwnd(hwnd: HWND) -> Self {
        let handle = raw_window_handle::Win32WindowHandle::new(
            std::num::NonZeroIsize::new(hwnd.0 as isize).unwrap(),
        );
        Self { handle }
    }

    fn hwnd(&self) -> HWND {
        HWND(self.handle.hwnd.get() as _)
    }

    fn forground_hwnd() -> HWND {
        #[allow(unsafe_code)]
        unsafe {
            GetForegroundWindow()
        }
    }

    fn get_window_rect(&self) -> super::Result<RECT> {
        let mut rect = RECT::default();
        #[allow(unsafe_code)]
        unsafe {
            GetWindowRect(self.hwnd(), &mut rect).map_err(|err| {
                tracing::error!("Failed to get window rect: {:?}", err);
                super::Error::GetWindowRect
            })?;
        };
        Ok(rect)
    }
}

impl HasWindowHandle for Window {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        #[allow(unsafe_code)]
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(self.handle.into()) })
    }
}

impl super::WindowInfo for Window {
    fn dpi(&self) -> crate::Result<u32> {
        #[allow(unsafe_code)]
        let dpi = unsafe { GetDpiForWindow(self.hwnd()) };

        Ok(dpi)
    }

    fn scale_factor(&self) -> f64 {
        let dpi = self.dpi().unwrap_or(96) as f64;

        dpi / 96.0
    }

    fn current_monitor(&self) -> crate::Result<Monitor> {
        #[allow(unsafe_code)]
        let monitor = unsafe { MonitorFromWindow(self.hwnd(), MONITOR_DEFAULTTONEAREST) };
        Ok(Monitor::new(monitor))
    }

    fn size(&self) -> super::Result<(i32, i32)> {
        let rect = self.get_window_rect()?;
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        Ok((width, height))
    }

    fn position(&self) -> super::Result<(i32, i32)> {
        let rect = self.get_window_rect()?;
        let x = rect.left;
        let y = rect.top;

        Ok((x, y))
    }

    fn title(&self) -> Option<String> {
        let mut buffer: [u16; 500] = [0; 500];
        #[allow(unsafe_code)]
        let written = unsafe {
            // set last error to 0 before calling GetWindowTextW to avoid false positives in error checking
            SetLastError(WIN32_ERROR(0));
            GetWindowTextW(self.hwnd(), &mut buffer)
        };
        if written == 0 {
            #[allow(unsafe_code)]
            let maybe_error = unsafe { GetLastError() };
            if maybe_error.is_err() {
                tracing::error!("Failed to get window title: {:?}", maybe_error);
            }
            return None;
        }
        let title = String::from_utf16_lossy(&buffer[..written as usize])
            .trim()
            .to_owned();

        if title.is_empty() { None } else { Some(title) }
    }

    fn minimized(&self) -> bool {
        #[allow(unsafe_code)]
        let is_iconic = unsafe { windows::Win32::UI::WindowsAndMessaging::IsIconic(self.hwnd()) };
        is_iconic.as_bool()
    }

    fn focused(&self) -> bool {
        let forground_window = Window::forground_hwnd();
        self.hwnd() == forground_window
    }

    fn process_id(&self) -> u32 {
        let mut pid: u32 = 0;
        #[allow(unsafe_code)]
        unsafe {
            GetWindowThreadProcessId(self.hwnd(), Some(&mut pid))
        };
        pid
    }

    fn process_path(&self) -> super::Result<String> {
        let pid = self.process_id();
        #[allow(unsafe_code)]
        let handle =
            unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) }
                .map_err(|err| {
                    tracing::error!("Failed to open process: {:?}", err);
                    super::Error::OpenProcess
                })?;

        let mut exe_buf = [0u16; MAX_PATH as usize + 1];

        #[allow(unsafe_code)]
        unsafe {
            GetModuleFileNameExW(Some(handle), None, &mut exe_buf);
            let _ = CloseHandle(handle);
        }

        let path = String::from_utf16_lossy(&exe_buf).trim().to_owned();
        if path.is_empty() {
            Err(super::Error::StringEmpty)
        } else {
            Ok(path)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Monitor {
    monitor: HMONITOR,
}

impl Monitor {
    fn new(monitor: HMONITOR) -> Self {
        Self { monitor }
    }

    fn get_monitor_info(&self) -> super::Result<MONITORINFOEXW> {
        let mut monitor_info = MONITORINFOEXW::default();
        monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
        #[allow(unsafe_code)]
        let status = unsafe {
            GetMonitorInfoW(
                self.monitor,
                &mut monitor_info as *mut MONITORINFOEXW as *mut MONITORINFO,
            )
        }
        .as_bool();

        if !status {
            #[allow(unsafe_code)]
            let error = unsafe { GetLastError() };
            tracing::error!("Failed to get monitor info: {:?}", error);
            Err(super::Error::GetWindowRect)
        } else {
            Ok(monitor_info)
        }
    }

    fn current_display_mode(&self) -> super::Result<DEVMODEW> {
        let monitor_info = self.get_monitor_info()?;
        let device_name = monitor_info.szDevice;
        let mut mode = DEVMODEW {
            dmSize: std::mem::size_of::<DEVMODEW>() as u16,
            ..Default::default()
        };

        #[allow(unsafe_code)]
        let res = unsafe {
            EnumDisplaySettingsExW(
                PCWSTR(device_name.as_ptr() as _),
                ENUM_CURRENT_SETTINGS,
                &mut mode,
                ENUM_DISPLAY_SETTINGS_FLAGS::default(),
            )
        }
        .as_bool();

        if !res {
            #[allow(unsafe_code)]
            let error = unsafe { GetLastError() };
            tracing::error!("Failed to get current display mode: {:?}", error);
            Err(MonitorError::GetMonitorVideoMode.into())
        } else {
            Ok(mode)
        }
    }

    fn dpi(&self) -> super::Result<u32> {
        // We are on Windows 8.1 or later.
        let mut dpi_x = 0;
        let mut dpi_y = 0;
        #[allow(unsafe_code)]
        let res =
            unsafe { GetDpiForMonitor(self.monitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y) };

        if res.is_err() {
            tracing::error!("Failed to get DPI: {:?}", res);
            Err(MonitorError::GetMonitorVideoMode.into())
        } else {
            Ok(dpi_x)
        }
    }
}

impl super::MonitorInfo for Monitor {
    fn id(&self) -> u128 {
        self.native_id() as u128
    }

    fn native_id(&self) -> u64 {
        self.monitor.0 as u64
    }

    fn name(&self) -> Option<String> {
        let monitor_info = self.get_monitor_info().ok()?;
        let name = String::from_utf16_lossy(&monitor_info.szDevice[..])
            .trim()
            .to_owned();
        if name.is_empty() { None } else { Some(name) }
    }

    fn position(&self) -> super::Result<(i32, i32)> {
        let monitor_info = self.get_monitor_info()?;
        let x = monitor_info.monitorInfo.rcMonitor.left;
        let y = monitor_info.monitorInfo.rcMonitor.top;
        Ok((x, y))
    }

    fn dpi(&self) -> crate::Result<u32> {
        let dpi = self.dpi()?;
        Ok(dpi)
    }

    fn scale_factor(&self) -> f64 {
        let dpi = self.dpi().unwrap_or(96) as f64;

        dpi / 96.0
    }

    fn size(&self) -> super::Result<(i32, i32)> {
        let monitor_info = self.get_monitor_info()?;
        let width =
            monitor_info.monitorInfo.rcMonitor.right - monitor_info.monitorInfo.rcMonitor.left;
        let height =
            monitor_info.monitorInfo.rcMonitor.bottom - monitor_info.monitorInfo.rcMonitor.top;

        #[cfg(debug_assertions)]
        {
            let mode = self.current_display_mode()?;
            assert!(width == mode.dmPelsWidth as i32);
            assert!(height == mode.dmPelsHeight as i32);
        }

        Ok((width, height))
    }

    fn bit_depth(&self) -> super::Result<u32> {
        let mode = self.current_display_mode()?;
        Ok(mode.dmBitsPerPel)
    }

    fn refresh_rate(&self) -> super::Result<u32> {
        let mode = self.current_display_mode()?;
        Ok(mode.dmDisplayFrequency)
    }
}
