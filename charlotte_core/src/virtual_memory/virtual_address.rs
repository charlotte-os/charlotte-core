use core::ops::Add;

use crate::arch::{Api, ArchApi};
use crate::physical_memory::physical_address::{MemoryAddress, PAGE_MASK, UAddr};

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(transparent)]
pub struct VirtualAddress(UAddr);

#[derive(Debug, Clone, Copy)]
pub enum VAddrError {
    InvalidForm(u64),
    InvalidAlignment(u64),
}

impl VirtualAddress {
    const NULL: VirtualAddress = VirtualAddress(0);
    /// Default constructor that provides a null virtual address
    /// (0x0 is to be used as the null virtual address in CharlotteOS)
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn bits(&self) -> UAddr {
        self.0
    }
    /// Check if the virtual address is null
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
    /// Check if the virtual address is aligned to the specified alignment
    #[inline]
    pub fn is_aligned_to(&self, align: UAddr) -> bool {
        self.0 % align == 0
    }
    /// Get the base address of the page that the virtual address is in
    #[inline]
    pub fn get_page_base(&self) -> UAddr {
        self.0 & PAGE_MASK
    }
    /// Get the offset of the virtual address from the base address of the page
    #[inline]
    pub fn get_page_offset(&self) -> usize {
        (self.0 & 0xfff) as usize
    }
    #[inline]
    pub fn pml4_index(&self) -> usize {
        ((self.0 >> 39) & 0x1ff) as usize
    }
    #[inline]
    pub fn pdpt_index(&self) -> usize {
        ((self.0 >> 30) & 0x1ff) as usize
    }
    #[inline]
    pub fn pd_index(&self) -> usize {
        ((self.0 >> 21) & 0x1ff) as usize
    }
    #[inline]
    pub fn pt_index(&self) -> usize {
        ((self.0 >> 12) & 0x1ff) as usize
    }
}

impl MemoryAddress for VirtualAddress {
    type MemoryAddress = VirtualAddress;

    fn is_aligned(&self, alignment: UAddr) -> bool {
        self.is_aligned_to(alignment)
    }

    fn is_page_aligned(&self) -> bool {
        self.is_aligned_to(self.get_page_base())
    }

    fn is_vaddress() -> bool {
        true
    }
}

impl Default for VirtualAddress {
    fn default() -> Self {
        Self::NULL
    }
}

impl TryFrom<u64> for VirtualAddress {
    type Error = VAddrError;

    fn try_from(addr: u64) -> Result<Self, Self::Error> {
        if ArchApi::validate_vaddr(addr) {
            Ok(Self(addr as UAddr))
        } else {
            Err(VAddrError::InvalidForm(addr))
        }
    }
}

impl From<VirtualAddress> for UAddr {
    #[inline]
    fn from(addr: VirtualAddress) -> Self {
        addr.0
    }
}

/// VirtualAddress can be converted to a const or mut pointer.
/// Safety: This conversion itself is safe, but dereferencing the resulting pointer may not be.
/// This is why only converstions to raw pointers are provided because dereferencing raw pointers is considered unsafe
/// and requires that invariants the compiler cannot verify are upheld by the developer.
impl<T> From<VirtualAddress> for *const T {
    #[inline]
    fn from(addr: VirtualAddress) -> *const T {
        addr.0 as *const T
    }
}

impl<T> From<VirtualAddress> for *mut T {
    #[inline]
    fn from(addr: VirtualAddress) -> *mut T {
        addr.0 as *mut T
    }
}

impl Add<usize> for VirtualAddress {
    type Output = Self;

    #[inline]
    fn add(self, val: usize) -> Self::Output {
        Self(self.0 + val as UAddr)
    }
}

impl Add<UAddr> for VirtualAddress {
    type Output = Self;

    #[inline]
    fn add(self, val: UAddr) -> Self::Output {
        Self(self.0 + val)
    }
}
