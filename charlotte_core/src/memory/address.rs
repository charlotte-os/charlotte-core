use core::num::NonZeroUsize;
use core::ops::Add;

use crate::arch::{Api, ArchApi};
use crate::memory::pmm::DIRECT_MAP;

const PAGE_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(4096) };
const PAGE_SHIFT: usize = 12;
const PAGE_MASK: u64 = !0xfffu64;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(transparent)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    #[inline]
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    #[inline]
    pub const fn bits(&self) -> usize {
        self.0
    }

    #[inline]
    pub const fn pfn(&self) -> usize {
        self.bits() >> PAGE_SHIFT
    }

    #[inline]
    pub const fn from_pfn(pfn: usize) -> Self {
        Self::new(pfn << PAGE_SHIFT)
    }

    #[inline]
    pub const fn is_aligned_to(&self, align: NonZeroUsize) -> bool {
        self.bits() & (align.get() - 1) == 0
    }

    #[inline]
    pub const fn is_page_aligned(&self) -> bool {
        self.is_aligned_to(PAGE_SIZE)
    }

    #[inline]
    pub fn iter_frames(&self, n_frames: usize) -> impl DoubleEndedIterator<Item = PhysicalAddress> {
        (self.bits()..(self.bits() + n_frames * PAGE_SIZE.get()))
            .step_by(PAGE_SIZE.get())
            .map(PhysicalAddress::new)
    }
}

impl From<usize> for PhysicalAddress {
    #[inline]
    fn from(val: usize) -> Self {
        Self::new(val)
    }
}

impl From<PhysicalAddress> for usize {
    #[inline]
    fn from(addr: PhysicalAddress) -> Self {
        addr.bits()
    }
}

impl<T> From<PhysicalAddress> for *const T {
    #[inline]
    fn from(addr: PhysicalAddress) -> *const T {
        (*DIRECT_MAP + addr.bits()).into()
    }
}

impl<T> From<PhysicalAddress> for *mut T {
    #[inline]
    fn from(addr: PhysicalAddress) -> *mut T {
        (*DIRECT_MAP + addr.bits()).into()
    }
}

impl Add<usize> for PhysicalAddress {
    type Output = Self;

    #[inline]
    fn add(self, val: usize) -> Self::Output {
        Self::new(self.bits() + val)
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(transparent)]
pub struct VirtualAddress(u64);

#[derive(Debug)]
pub enum VAddrError {
    InvalidForm,
    InvalidAlignment,
}

impl VirtualAddress {
    const NULL: VirtualAddress = VirtualAddress(0);
    /// Default constructor that provides a null virtual address
    /// (0x0 is to be used as the null virtual address in CharlotteOS)
    #[inline]
    pub fn new() -> Self {
        Self(0)
    }
    #[inline]
    pub fn bits(&self) -> u64 {
        self.0
    }
    /// Check if the virtual address is null
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
    /// Check if the virtual address is aligned to the specified alignment
    #[inline]
    pub fn is_aligned_to(&self, align: u64) -> bool {
        self.0 % align == 0
    }
    /// Get the base address of the page that the virtual address is in
    #[inline]
    pub fn get_page_base(&self) -> u64 {
        self.0 & PAGE_MASK
    }
    /// Get the offset of the virtual address from the base address of the page
    #[inline]
    pub fn get_page_offset(&self) -> u64 {
        self.0 & !PAGE_MASK
    }
}

impl TryFrom<u64> for VirtualAddress {
    type Error = VAddrError;

    fn try_from(addr: u64) -> Result<Self, Self::Error> {
        if ArchApi::validate_vaddr(addr) {
            Ok(Self(addr))
        } else {
            Err(VAddrError::InvalidForm)
        }
    }
}

impl From<VirtualAddress> for u64 {
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
        Self(self.0 + val as u64)
    }
}
