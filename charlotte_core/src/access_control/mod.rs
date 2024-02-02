//! #Access Control
//! This module contains the access control system, which is responsible for managing access to
//! various resources in the system. It uses capabilities to control access to resources and provides
//! an interface for creating, deleting, and transferring capabilities. The access control system is
//! capable of enforcing access control policies and ensuring that only authorized entities can access
//! resources and that they can only perform authorized operations on those resources.

pub type CapabilityKey = u64;
pub type ContextId = i64;

pub trait Capability {
    fn get_key(&self) -> CapabilityKey;
    fn get_context_id(&self) -> ContextId;
}

pub struct PhysicalMemoryCapability {
    key: CapabilityKey,
    context_id: ContextId,
    pfa_index: usize,
}