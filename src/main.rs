use std::env::args;
use std::process::exit;

fn print_help() {
    println!("Usage:");
    println!("      pci-range <cell0> <cell1> .. <cell7>");
}

#[derive(Debug)]
struct PCIMap {
    high: Option<u32>,
    mid: Option<u32>,
    low: Option<u32>,
    phy_high: Option<u32>,
    phy_low: Option<u32>,
    size_high: Option<u32>,
    size_low: Option<u32>,
}

#[derive(Debug)]
enum PciAddress {
    IO(u32),
    Config(u16),
    MMIO32(u32),
    MMIO64(u64),
}

impl std::fmt::Display for PciAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PciAddress::*;

        match self {
            IO(io) => write!(f, "IO(0x{:08x})", io),
            Config(bdf) => write!(f, "Config(Bus(0x{:02x}), Device(0x{:02x}), Func(0x{:01x}))", 
                                    bdf >> 8, (bdf & 0xff) >> 3, bdf & 0x8),
            MMIO32(a32) => write!(f, "MMIO32(0x{:08x})", a32),
            MMIO64(a64) => write!(f, "MMIO64(0x{:016x})", a64),
        }
    }
}


impl PCIMap {
    pub fn address(&self) -> Option<PciAddress> {
        use PciAddress::*;

        let high = self.high?;

        Some(match (high >> 24) & 0x3 {
            0 => Config(((high >> 8) & 0xffff) as u16),
            1 => IO(self.low?),
            2 => MMIO32(self.low?),
            3 => MMIO64(((self.mid? as u64) << 32) + self.low? as u64),
            _ => panic!("unknow pci address type")
        })
    }

    fn check_bit(i: u32, n: usize) -> bool {
        (i >> n) & 0x1 != 0
    }

    pub fn relocatable(&self) -> Option<bool> {
        Some(Self::check_bit(self.high?, 31))
    }

    pub fn prefetchable(&self) -> Option<bool> {
        Some(Self::check_bit(self.high?, 30))
    }

    pub fn aliased(&self) -> Option<bool> {
        Some(Self::check_bit(self.high?, 29))
    }

    fn to_u64(h: u32, l: u32) -> u64 {
        ((h as u64) << 32) + l as u64 
    }

    pub fn physical_addr(&self) -> Option<u64> {
        Some(Self::to_u64(self.phy_high?, self.phy_low?))
    }

    pub fn size(&self) -> Option<u64> {
        Some(Self::to_u64(self.size_high?, self.size_low?))
    }
}

fn main() {
    if args().len() != 8 {
        print_help();
        exit(1);
    }

    let args: Vec<Option<u32>> = args().skip(1).map(|s|
        u32::from_str_radix(s.trim_start_matches("0x"), 16).ok()
    ).chain(std::iter::repeat(None)).take(7).collect();

    let map = PCIMap {
            high: args[0],
            mid: args[1],
            low: args[2],
            phy_high: args[3],
            phy_low: args[4],
            size_high: args[5],
            size_low: args[6] 
    };

    println!("PciAddress: {}", map.address().unwrap());
    println!("PhysicalAddress: 0x{:016x}", map.physical_addr().unwrap());
    println!("Size: 0x{:x}", map.size().unwrap());
    println!("relocatable: {}, prefetchable: {}, aliased: {}", 
            map.relocatable().unwrap(), map.prefetchable().unwrap(), map.aliased().unwrap());
}
