extern crate core;

#[repr(C, packed)]
pub struct VbeMode(u16);
impl VbeMode {
    #[allow(dead_code)]
    pub fn is_vesa(&self) -> bool {
        self.0 & (1 << 7) > 0
    }

    #[allow(dead_code)]
    pub fn is_flat_buffer(&self) -> bool {
        self.0 & (1 << 13) > 0
    }

    #[allow(dead_code)]
    pub fn code(&self) -> u8 {
        (self.0 & 0xF) as u8
    }
}
impl core::fmt::Debug for VbeMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

#[repr(C, packed)]
pub struct VbeControlInfo {
    signature: [u8; 4],
    version: u16,
    oem: u32,
    capabilities: u32,
    video_modes: u32,
    video_memory: u16,
    software_rev: u16,
    vendor: u32,
    product_name: u32,
    product_rev: u32,
    reserved: [u8; 222],
    oem_data: [u8; 256],
}
impl core::fmt::Debug for VbeControlInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "OMITTING VbeControlInfo")  // TODO: better info
    }
}

#[repr(C, packed)]
pub struct VbeModeInfo {
    attributes: u16,
    window_a: u8,
    window_b: u8,
    granularity: u16,
    window_size: u16,
    segment_a: u16,
    segment_b: u16,
    win_func_ptr: u32,
    pix_pitch: u16,
    frame_width: u16,
    frame_height: u16,
    w_char: u8,
    y_char: u8,
    planes: u8,
    bits_per_pix: u8,
    banks: u8,
    memory_model: u8,
    bank_size: u8,
    image_pages: u8,
    reserved0: u8,
 
    red_mask: u8,
    red_position: u8,
    green_mask: u8,
    green_position: u8,
    blue_mask: u8,
    blue_position: u8,
    reserved_mask: u8,
    reserved_position: u8,
    direct_color_attributes: u8,
 
    framebuffer: u32,
    off_screen_mem_off: u32,
    off_screen_mem_size: u16,
    reserved1: [u8; 206],
}
impl VbeModeInfo {
    #[allow(dead_code)]
    pub fn addr(&self) -> usize {
        self.framebuffer as usize
    }

    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.frame_width as usize
    }

    #[allow(dead_code)]
    pub fn height(&self) -> usize {
        self.frame_width as usize
    }

    #[allow(dead_code)]
    pub fn bpp(&self) -> u8 {
        self.bits_per_pix
    }

    #[allow(dead_code)]
    pub fn pitch(&self) -> usize {
        self.pix_pitch as usize
    }
}
impl core::fmt::Debug for VbeModeInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "mode: {}x{}x{}", self.width(), self.height(), self.bpp()) // TODO: better info
    }
}

#[derive(Debug)]
#[repr(packed)]
pub struct VbeTag {
    typ: u32,
    size: u32,

    mode_code: VbeMode,
    int_seg: u16,
    int_off: u16,
    int_len: u16,

    control_info: VbeControlInfo,
    mode_info: VbeModeInfo,
}

impl VbeTag {
    pub fn mode(&self) -> &'static VbeModeInfo {
        unsafe { &*(&self.mode_info as *const VbeModeInfo) }
    }
}
