use crate::arch::{ArchApi, ISA_PARAMS};
use core::num::NonZeroUsize;
use core::ops::Add;

pub const PAGE_SIZE: UAddr = ISA_PARAMS.paging.page_size;
pub const PAGE_SHIFT: UAddr = ISA_PARAMS.paging.page_shift;
pub const PAGE_MASK: UAddr = ISA_PARAMS.paging.page_mask;

#[cfg(not(target_pointer_width = "64"))]
compile_error! {"Unsupported ISA pointer width"}

#[cfg(target_pointer_width = "64")]
pub type UAddr = u64;

pub trait MemoryAddress {
    type MemoryAddress: MemoryAddress;
    fn is_aligned(&self, alignment: UAddr) -> bool;
    fn is_page_aligned(&self) -> bool;
    fn is_vaddress() -> bool;
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(transparent)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    const FRAME_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(4096) };
    const FRAME_SHIFT: usize = 12;

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
        self.bits() >> Self::FRAME_SHIFT
    }

    #[inline]
    pub const fn from_pfn(pfn: usize) -> Self {
        Self::new(pfn << Self::FRAME_SHIFT)
    }

    #[inline]
    pub const fn is_aligned_to(&self, align: NonZeroUsize) -> bool {
        self.bits() & (align.get() - 1) == 0
    }

    #[inline]
    pub const fn is_page_aligned(&self) -> bool {
        self.is_aligned_to(Self::FRAME_SIZE)
    }

    #[inline]
    pub fn iter_frames(&self, n_frames: usize) -> impl DoubleEndedIterator<Item = PhysicalAddress> {
        (self.bits()..(self.bits() + n_frames * Self::FRAME_SIZE.get()))
            .step_by(Self::FRAME_SIZE.get())
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

impl Add<usize> for PhysicalAddress {
    type Output = Self;

    #[inline]
    fn add(self, val: usize) -> Self::Output {
        Self::new(self.bits() + val)
    }
}
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(transparent)]
pub struct VirtualAddress(UAddr);

#[derive(Debug, Copy, Clone)]
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

// impl TryFrom<u64> for VirtualAddress {
//     type Error = VAddrError;
//
//     fn try_from(addr: u64) -> Result<Self, Self::Error> {
//         if ArchApi::validate_vaddr(addr) {
//             Ok(Self(addr as UAddr))
//         } else {
//             Err(VAddrError::InvalidForm(addr))
//         }
//     }
// }

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

pub fn is_aligned(addr: UAddr, alignment: UAddr) -> bool {
    addr & (alignment - 1) == 0
}
