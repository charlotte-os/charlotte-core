use core::ops::Add;

use crate::arch::ISA_PARAMS;
use super::DIRECT_MAP;

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
pub struct PhysicalAddress(UAddr);

impl PhysicalAddress {
    #[inline]
    pub const fn new(addr: UAddr) -> Self {
        Self(addr)
    }

    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn bits(&self) -> UAddr {
        self.0
    }

    #[inline]
    pub const fn pfn(&self) -> UAddr {
        self.bits() >> PAGE_SHIFT
    }

    #[inline]
    pub const fn from_pfn(pfn: UAddr) -> Self {
        Self::new(pfn << PAGE_SHIFT)
    }

    #[inline]
    /// # Safety
    /// This function will panic in case align == 0 or align - 1 == 0
    pub const fn is_aligned_to(&self, align: UAddr) -> bool {
        if align == 0 {
            panic!("Tried to test alignment to 0")
        }
        self.bits() & (align - 1) == 0
    }

    #[inline]
    pub const fn is_page_aligned(&self) -> bool {
        self.is_aligned_to(PAGE_SIZE)
    }

    #[inline]
    pub fn iter_frames(&self, n_frames: UAddr) -> impl Iterator<Item = PhysicalAddress> {
        (self.bits()..(self.bits() + n_frames * PAGE_SIZE))
            .step_by(PAGE_SIZE as usize)
            .map(PhysicalAddress::new)
    }

    pub unsafe fn as_ref<T>(&self) -> &T {
        unsafe { &*(self.bits() as *const T) }
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut<T>(&self) -> &mut T {
        unsafe { &mut *(self.bits() as *mut T) }
    }
}

impl MemoryAddress for PhysicalAddress {
    type MemoryAddress = PhysicalAddress;

    fn is_aligned(&self, alignment: UAddr) -> bool {
        self.is_aligned_to(alignment)
    }

    fn is_page_aligned(&self) -> bool {
        self.is_page_aligned()
    }

    fn is_vaddress() -> bool {
        false
    }
}

impl From<UAddr> for PhysicalAddress {
    #[inline]
    fn from(val: UAddr) -> Self {
        Self::new(val)
    }
}

impl From<PhysicalAddress> for usize {
    #[inline]
    fn from(addr: PhysicalAddress) -> Self {
        addr.as_usize()
    }
}

impl<T> From<PhysicalAddress> for *const T {
    #[inline]
    #[allow(clippy::unconditional_recursion)]
    fn from(addr: PhysicalAddress) -> *const T {
        (*DIRECT_MAP + addr.0).into()
    }
}

#[allow(clippy::unconditional_recursion)]
impl<T> From<PhysicalAddress> for *mut T {
    #[inline]
    fn from(addr: PhysicalAddress) -> *mut T {
        (*DIRECT_MAP + addr.0).into()
    }
}

impl Add<UAddr> for PhysicalAddress {
    type Output = Self;

    #[inline]
    fn add(self, val: UAddr) -> Self::Output {
        Self::new(self.0 + val)
    }
}