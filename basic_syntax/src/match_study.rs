#[derive(Debug)]
pub enum CpuArchitecture {
    X86,
    X64,
    M1,
}

#[derive(Debug)]
pub enum Computer {
    X86_LAPTOP(CpuArchitecture),
    M1_LAPTOP(CpuArchitecture),
}

#[derive(Debug)]
pub enum ComputerBrand {
    HP(Computer),
    APPLE(Computer),
}

pub fn print_computer(computer: ComputerBrand) {
    match computer {
        ComputerBrand::HP(computer) => {
            println!("{:?}", computer);
        }
        ComputerBrand::APPLE(computer) => {
            println!("{:?}", computer);
        }
    }
}
