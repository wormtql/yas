use std::os::windows::ffi::OsStrExt;

pub use winapi::um::winuser::{FindWindowW, GetClientRect, ClientToScreen, GetAsyncKeyState, VK_RBUTTON, SetProcessDPIAware, ShowWindow, SW_RESTORE, SetForegroundWindow, FindWindowExW, GetWindowLongPtrW, GWL_EXSTYLE, GWL_STYLE};
pub use winapi::shared::windef::{HWND, RECT as WinRect, POINT as WinPoint};
pub use winapi::um::winnt::{SID_IDENTIFIER_AUTHORITY, SECURITY_NT_AUTHORITY, PSID, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_ADMINS, CHAR};
pub use winapi::um::securitybaseapi::{AllocateAndInitializeSid, CheckTokenMembership, FreeSid};
pub use winapi::shared::minwindef::{BOOL, HINSTANCE};
pub use winapi::um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryA, LoadLibraryW};
// use winapi::um::shellscalingapi::{PROCESS_PER_MONITOR_DPI_AWARE, SetProcessDpiAwareness};