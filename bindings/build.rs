fn main() {
    windows::build!(
        Windows::Data::Xml::Dom::*,
        Windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
        Windows::Win32::System::WindowsProgramming::CloseHandle,
        Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
    );
}