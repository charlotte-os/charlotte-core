//! # Boot Information
//! This module contains requests for information from the Limine boot protocol.

pub use limine::memory_map;
pub use limine::request::*;
pub use limine::response::*;
use limine::BaseRevision;

#[allow(unused)]
/// Require version 1 or later of the Limine boot protocol
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

/// This request is used to obtain a direct mapping of physical memory
/// in the kernel's address space.
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();
/// This request is used to obtain the memory map.
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

/// This request is used to obtain the framebuffer
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();
