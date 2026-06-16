pub struct ProgramHeader {
    pub e_type: u32,
    pub flags: u32,
    pub offset: u64,
    pub vaddr: u64,
    pub filesz: u64,
    pub memsz: u64,
}

pub struct ElfHeader {
    pub e_ident: [u8; 4],  //00
    pub class: u8,         //04
    pub data: u8,          //05
    pub version: [u8; 10], //06
    pub e_type: u16,       //10
    pub e_machine: u16,    //12
    pub e_version: u32,    //14
    pub e_entry: u64,      //18
    pub e_phoff: u64,      //20
    pub e_shoff: u64,      //28
    pub e_flags: u32,      //30
    pub e_ehsize: u16,     //34
    pub e_phentsize: u16,  //36
    pub e_phnum: u16,
    pub e_shentsize: u16, //3A
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

pub fn parse(path: &str) {
    let contents: Vec<u8> = std::fs::read(&path).expect("failed to read the file");
    /*if contents[0..4] != [0x7f, 0x45, 0x4c, 0x46] {
        panic!("it is not an ELF");
    }*/
    println!("{}", contents[0x4]);
}
