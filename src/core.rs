#![windows_subsystem = "windows"]

use std::fs;
use std::path::Path;
use std::{thread, time::Duration};

use windows::core::HSTRING;
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Media::MediaFoundation::{
            IMFAttributes, IMFMediaType, IMFSample, MF_MT_DEFAULT_STRIDE, MF_MT_FRAME_SIZE,
            MF_MT_MAJOR_TYPE, MF_MT_SUBTYPE, MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING,
            MF_SOURCE_READERF_ENDOFSTREAM, MF_VERSION, MFCreateAttributes, MFCreateMediaType,
            MFCreateSourceReaderFromURL, MFMediaType_Video, MFSTARTUP_FULL, MFStartup,
            MFVideoFormat_RGB32,
        },
        System::{
            Com::StructuredStorage::{PROPVARIANT, PropVariantClear},
            LibraryLoader::GetModuleHandleW,
            Variant::VT_I8,
        },
        UI::WindowsAndMessaging::{
            CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW,
            DispatchMessageW, FindWindowExW, FindWindowW, GetSystemMetrics, IDC_ARROW, LoadCursorW,
            MSG, PM_REMOVE, PeekMessageW, PostQuitMessage, RegisterClassExW, SM_CXSCREEN,
            SM_CYSCREEN, SW_SHOWDEFAULT, SendMessageW, ShowWindow, TranslateMessage, WM_DESTROY,
            WM_QUIT, WNDCLASSEXW, WS_CHILD, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT,
        },
    },
    core::{GUID, Result, w},
};

mod cblanks;
mod f3d;
mod ferror;

pub struct Settings {
    pub width: i32,
    pub heigth: i32,
    pub latency: u64,
    pub vsync: u32,
    pub filename: String,
}

fn main() -> Result<()> {
    unsafe {
        let settings_structure = create_or_get_settings()?;

        let width = settings_structure.width;
        let heigth = settings_structure.heigth;
        let latency = settings_structure.latency;
        let vsync = settings_structure.vsync;
        let filename = settings_structure.filename;

        let filename = HSTRING::from(filename);

        let hinstance = GetModuleHandleW(None)?;

        let wndclass: WNDCLASSEXW = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndprocedure),
            hInstance: hinstance.into(),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            lpszClassName: cblanks::CLASSNAME,
            ..Default::default()
        };

        let regwinclassinfo = RegisterClassExW(&wndclass);

        if regwinclassinfo == 0 {
            return Err(ferror::get_error_win32());
        }

        let winparent = create_and_get_workerw().unwrap();

        let aniwin = CreateWindowExW(
            WS_EX_NOACTIVATE | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW,
            cblanks::CLASSNAME,
            w!(""),
            WS_CHILD,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            heigth,
            Some(winparent),
            None,
            Some(hinstance.into()),
            None,
        )?;

        let _ = ShowWindow(aniwin, SW_SHOWDEFAULT);

        MFStartup(MF_VERSION, MFSTARTUP_FULL)?;

        let mut attributes: Option<IMFAttributes> = None;
        MFCreateAttributes(&mut attributes, 1)?;
        let attributes = attributes.unwrap();

        attributes.SetUINT32(&MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING, 1)?;

        let reader = MFCreateSourceReaderFromURL(&filename, Some(&attributes))?;
        let media_type = MFCreateMediaType()?;

        media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;
        media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_RGB32)?;

        reader.SetCurrentMediaType(0xFFFFFFFC, None, &media_type)?;

        let current_type: IMFMediaType = reader.GetCurrentMediaType(0xFFFFFFFC)?;
        let frame_size = current_type.GetUINT64(&MF_MT_FRAME_SIZE)?;

        let width_video = (frame_size >> 32) as u32;
        let height_video = (frame_size & 0xFFFF_FFFF) as u32;

        let stride = current_type
            .GetUINT32(&MF_MT_DEFAULT_STRIDE)
            .map(|value| value as i32)
            .unwrap_or((width_video * 4) as i32);

        let renderer = f3d::Renderer::create_renderer(
            aniwin,
            width as u32,
            heigth as u32,
            width_video,
            height_video,
        )?;

        let mut msg: MSG = MSG::default();

        loop {
            if PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {
                if msg.message == WM_QUIT {
                    break;
                }

                if msg.message == WM_DESTROY {
                    break;
                }

                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            } else {
                let mut stream_flags = 0u32;
                let mut sample: Option<IMFSample> = None;

                reader.ReadSample(
                    0xFFFFFFFC,
                    0,
                    None,
                    Some(&mut stream_flags),
                    None,
                    Some(&mut sample),
                )?;

                if stream_flags & MF_SOURCE_READERF_ENDOFSTREAM.0 as u32 != 0 {
                    let mut position = PROPVARIANT::default();

                    let inner = &mut *position.Anonymous.Anonymous;
                    inner.vt = VT_I8;
                    inner.Anonymous.hVal = 0;

                    reader.SetCurrentPosition(&GUID::zeroed(), &position)?;

                    PropVariantClear(&mut position)?;

                    continue;
                }

                let Some(sample) = sample else {
                    continue;
                };

                let buffer = sample.ConvertToContiguousBuffer()?;

                let mut data: *mut u8 = std::ptr::null_mut();
                let mut max_len = 0u32;
                let mut cur_len = 0u32;

                buffer.Lock(&mut data, Some(&mut max_len), Some(&mut cur_len))?;

                let row_bytes = (width_video * 4) as usize;
                let stride_abs = stride.unsigned_abs() as usize;

                let mut pixels = vec![0u8; row_bytes * height_video as usize];

                for y in 0..height_video as usize {
                    let src_row_start: *const u8 = if stride >= 0 {
                        data.add(y * stride_abs)
                    } else {
                        data.add((height_video as usize - 1 - y) * stride_abs)
                    };

                    let dst_row_start: *mut u8 = pixels.as_mut_ptr().add(y * row_bytes);

                    for x in 0..width_video as usize {
                        let src_byte_ptr = src_row_start.add(x * 4);
                        let dst_byte_ptr = dst_row_start.add(x * 4);

                        *dst_byte_ptr.add(0) = *src_byte_ptr.add(2);

                        *dst_byte_ptr.add(1) = *src_byte_ptr.add(1);

                        *dst_byte_ptr.add(2) = *src_byte_ptr.add(0);
                        
                        *dst_byte_ptr.add(3) = *src_byte_ptr.add(3);
                    }
                }

                buffer.Unlock()?;

                renderer.change_texture(&pixels, width_video as u32, height_video as u32)?;
                renderer.render([0.1, 0.1, 0.3, 1.0], vsync);
                thread::sleep(Duration::from_millis(latency));
            }
        }
    }

    Ok(())
}

unsafe extern "system" fn wndprocedure(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => unsafe {
            PostQuitMessage(0);
            LRESULT(0)
        },
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

unsafe fn create_and_get_workerw() -> Option<HWND> {
    unsafe {
        let progman = FindWindowW(w!("Progman"), None).ok()?;
        SendMessageW(progman, 0x052c, Some(WPARAM(0)), Some(LPARAM(0)));

        let mut worker_w_behind_icons = HWND::default();
        let mut found_def_view = false;

        let mut current = HWND::default();

        loop {
            current = FindWindowExW(Some(HWND::default()), Some(current), w!("WorkerW"), None)
                .unwrap_or_default();

            if current == HWND::default() {
                break;
            }

            if found_def_view {
                worker_w_behind_icons = current;
                break;
            }

            let def_view = FindWindowExW(
                Some(current),
                Some(HWND::default()),
                w!("SHELLDLL_DefView"),
                None,
            );

            if def_view.is_ok() {
                found_def_view = true;
            }
        }

        return Some(worker_w_behind_icons);
    }
}

fn create_or_get_settings() -> Result<Settings> {
    unsafe {
        let path = Path::new("bin\\settings\\config.txt");

        if !path.exists() {
            let width = GetSystemMetrics(SM_CXSCREEN);
            let heigth = GetSystemMetrics(SM_CYSCREEN);
            let latency = 23_u64;
            let vsync = 0;
            let filename = "video.mp4".to_string();

            fs::create_dir_all("bin\\settings")?;

            let settings = format!(
                "width: {width}\nheigth: {heigth}\nlatency: {latency}\nfilename: {filename}\nvsync: {vsync}"
            );

            fs::write(path, settings)?;

            Ok(Settings {
                width,
                heigth,
                latency,
                vsync,
                filename,
            })
        } else {
            let mut structure = Settings {
                width: 0,
                heigth: 0,
                latency: 0,
                vsync: 0,
                filename: "".to_string(),
            };

            let settings = fs::read_to_string("bin\\settings\\config.txt")?;

            for array in settings.lines() {
                let lir: Vec<&str> = array.splitn(2, ":").collect();

                let key = lir[0].trim();
                let value = lir[1].trim();

                match key {
                    "width" => structure.width = value.parse().unwrap(),
                    "heigth" => structure.heigth = value.parse().unwrap(),
                    "latency" => structure.latency = value.parse().unwrap(),
                    "vsync" => structure.vsync = value.parse().unwrap(),
                    "filename" => structure.filename = value.to_string(),
                    _ => {}
                }
            }

            Ok(structure)
        }
    }
}
