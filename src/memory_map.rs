extern crate core;

#[derive(Debug)]
#[repr(C)]
pub struct MemoryMapTag {
    typ: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
    first_area: MemoryArea,
}

impl MemoryMapTag {
    pub fn memory_areas(&self) -> MemoryAreaIter {
        let self_ptr = self as *const MemoryMapTag;
        let start_area = (&self.first_area) as *const MemoryArea;
        MemoryAreaIter {
            current_area: start_area as u64,
            last_area: (self_ptr as u64 + (self.size - self.entry_size) as u64),
            entry_size: self.entry_size,
        }
    }
}

#[repr(u32)]
#[derive(Clone, PartialEq)]
// Multiboot Specification version 1.6 -- Page 12
// http://nongnu.askapache.com/grub/phcoder/multiboot.pdf
pub enum MemoryAreaType {
    Usable           = 1,
    ACPI             = 3,
    HibernatePersist = 4,
}
impl core::fmt::Debug for MemoryAreaType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            MemoryAreaType::Usable             => { write!(f, "Usable")  }
            MemoryAreaType::ACPI               => { write!(f, "ACPI")    }
            MemoryAreaType::HibernatePersist   => { write!(f, "HibPers") }
        }
    }
}
impl core::fmt::Display for MemoryAreaType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
#[repr(C)]
pub struct MemoryArea {
    base_addr: u64,
    length: u64,
    typ: MemoryAreaType,
    _reserved: u32,
}

impl MemoryArea {
    pub fn start_address(&self) -> usize {
        self.base_addr as usize
    }

    pub fn end_address(&self) -> usize {
        (self.base_addr + self.length) as usize
    }

    pub fn size(&self) -> usize {
        self.length as usize
    }

    pub fn area_type(&self) -> MemoryAreaType {
        self.typ.clone()
    }
}

#[derive(Clone, Debug)]
pub struct MemoryAreaIter {
    current_area: u64,
    last_area: u64,
    entry_size: u32,
}

impl Iterator for MemoryAreaIter {
    type Item = &'static MemoryArea;
    fn next(&mut self) -> Option<&'static MemoryArea> {
        if self.current_area > self.last_area {
            None
        } else {
            let area = unsafe{&*(self.current_area as *const MemoryArea)};
            self.current_area = self.current_area + (self.entry_size as u64);
            Some(area)
        }
    }
}
