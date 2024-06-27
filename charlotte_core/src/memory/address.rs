use core::num::NonZeroUsize;
use core::ops::Add;

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
    pub const fn bits(self) -> usize {
        self.0
    }

    #[inline]
    pub const fn pfn(self) -> usize {
        self.bits() >> Self::FRAME_SHIFT
    }

    #[inline]
    pub const fn from_pfn(pfn: usize) -> Self {
        Self::new(pfn << Self::FRAME_SHIFT)
    }

    #[inline]
    pub const fn is_aligned_to(self, align: NonZeroUsize) -> bool {
        self.bits() & (align.get() - 1) == 0
    }

    #[inline]
    pub const fn is_page_aligned(self) -> bool {
        self.is_aligned_to(Self::FRAME_SIZE)
    }

    #[inline]
    pub fn iter_frames(self, n_frames: usize) -> impl DoubleEndedIterator<Item = PhysicalAddress> {
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
