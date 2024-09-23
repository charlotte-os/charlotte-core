mod isa;

pub use isa::InitApiImpl;

pub trait InitApi {
    fn init_system();
    fn init_bsp();
    fn init_ap();
}
