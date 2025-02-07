fn main() {
    windows::build!(
      Windows::Win32::FileSystem::{CreateFileA, ReadFile},
      Windows::Win32::SystemServices::{
          CreateEventA, WaitForSingleObject, GetOverlappedResult,
      },
      Windows::Win32::Debug::GetLastError,
      Windows::Win32::WindowsProgramming::CloseHandle,
    );
}
