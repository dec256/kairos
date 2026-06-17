pub struct ProgramHeader {
    pub e_type: u32,
    pub flags: u32,
    pub offset: u64,
    pub vaddr: u64,
    pub filesz: u64,
    pub memsz: u64,
}

pub struct Elf {
    //pub e_ident: [u8; 4],  //00
    //pub class: u8,         //04
    //pub data: u8,          //05
    //pub version: [u8; 10], //06
    //pub e_type: u16,       //10
    pub e_machine: u16, //12
    //pub e_version: u32,    //14
    pub e_entry: u64, //18
    pub e_phoff: u64, //20
    pub e_shoff: u64, //28
    //pub e_flags: u32,      //30
    pub e_ehsize: u16,    //34
    pub e_phentsize: u16, //36
    pub e_phnum: u16,
    pub e_shentsize: u16, //3A
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}
impl Elf {
    pub fn parse(path: &str) -> Result<Self, std::string::ParseError> {
        let contents: Vec<u8> = std::fs::read(&path).expect("failed to read the file");
        if contents[0..4] != [0x7f, 0x45, 0x4c, 0x46] {
            panic!("it is not an ELF");
        }
        Ok(Self {
            e_entry: u64::from_le_bytes(contents[0x18..0x20].try_into().expect("failure parsing")), //e_entry
            e_phoff: u64::from_le_bytes(contents[0x20..0x28].try_into().expect("failure parsing")), //program header offset
            e_shoff: u64::from_le_bytes(contents[0x28..0x30].try_into().expect("failure parsing")), //section header offset
            e_machine: u16::from_le_bytes(
                contents[0x12..0x14].try_into().expect("failure parsing"),
            ),
            e_ehsize: u16::from_le_bytes(contents[0x34..0x36].try_into().expect("failure parsing")),
            e_phentsize: u16::from_le_bytes(
                contents[0x34..0x36].try_into().expect("failure parsing"),
            ),
            e_phnum: u16::from_le_bytes(contents[0x38..0x3A].try_into().expect("failure parsing")),
            e_shentsize: u16::from_le_bytes(
                contents[0x3A..0x3C].try_into().expect("failure parsing"),
            ),
            e_shnum: u16::from_le_bytes(contents[0x3C..0x3E].try_into().expect("failure parsing")),
            e_shstrndx: u16::from_le_bytes(
                contents[0x3E..0x40].try_into().expect("failure parsing"),
            ),
        })
    }
    pub fn entry(&self) -> u64 {
        self.e_entry
    }
}
