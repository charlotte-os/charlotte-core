extern "C" {
    pub fn asm_cpuid(output: &mut [u32; 4], eax: u32, ecx: u32);
    pub fn asm_get_vendor_string(dest: &mut [u8; 12]);
}
