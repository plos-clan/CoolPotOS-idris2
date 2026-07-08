#![no_std]
#![no_main]

use kernel::{kernel_entry, serial};
use limine::{
    BaseRevision, RequestsEndMarker, RequestsStartMarker,
    request::{EntryPointRequest, StackSizeRequest},
};

#[used]
#[unsafe(link_section = ".requests_start")]
static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static STACK_SIZE: StackSizeRequest = StackSizeRequest::new(1024 * 1024);

#[used]
#[unsafe(link_section = ".requests")]
static ENTRY_POINT: EntryPointRequest = EntryPointRequest::new(_start);

#[used]
#[unsafe(link_section = ".requests_end")]
static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    serial::write_str("kernel: limine handoff ok\r\n");
    kernel_entry()
}
