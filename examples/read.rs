use memflex::external::{open_process_by_name};
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;

fn main() {
    dbg!(open_process_by_name("firefox.exe", false, PROCESS_ALL_ACCESS).unwrap().name());
}
