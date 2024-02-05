//! # Access Control and Capabilities
//! This module contains the access control system, which is responsible for managing access to
//! various resources in the system. It uses capabilities to control access to resources and provides
//! an interface for creating, deleting, and transferring capabilities. The access control system is
//! capable of enforcing access control policies and ensuring that only authorized entities can access
//! resources and that they can only perform authorized operations on those resources.

/// Each context will have it's own capability key table, which is a hash map that maps capability keys that
/// are unique to that context and map to a tuple containing the capability type and its ID.

pub type CapabilityKey = u64;
pub type ContextId = i64;
pub enum CapabilityType {
    PhysicalMemory,
    VirtualMemory,
    IOPort,
}

/// There will be a global capability table for each capability type that maps capability IDs to the
/// capability struct itself. This table will be used to look up capabilities by their IDs.

type CapabilityId = u64;

/// A capability represents a resource and the operations that can be performed on that resource. All 
/// capabilities must implement the Drop trait so that the resource can be cleaned up when the capability
/// is dropped.
trait Capability: Drop {
    /// Obtains the ID that uniquely identifies this capability
    fn get_id(&self) -> CapabilityId;
    /// Obtains the number of contexts that currently hold a reference to this capability
    fn get_refcount(&self) -> usize;
}

/// The physical memory capability represents a capability to access a region of physical memory with
/// a specific set of permissions. It is used to control access to physical memory and to ensure that
/// only authorized entities can access physical memory and that they can only perform authorized operations
/// on that memory.
pub struct PhysicalMemoryCapability {
    id: CapabilityId,
    refcount: usize,
    /// Physical frame allocator index
    pfa_index: usize,
    permissions: u8
}

enum PhysicalMemoryPermissions {
    DirectMapRead = 0b00000001,
    DirectMapWrite = 0b00000010,
    VASMapReadable = 0b00000100,
    VASMapWritable = 0b00001000,
    VASMapExecutable = 0b00010000,
    ModCacheSettings = 0b00100000,
    ModSwapSettings = 0b01000000,
    ModCopySettings = 0b10000000,
    ModMiscSettings = 0b100000000
}

impl Capability for PhysicalMemoryCapability {
    fn get_id(&self) -> CapabilityId {
        self.id
    }

    fn get_refcount(&self) -> usize {
        self.refcount
    }
}

impl Drop for PhysicalMemoryCapability {
    fn drop(&mut self) {
        //deallocate the associated physical memory region
    }
}