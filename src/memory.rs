use std::io::Error;
use std::fs;

pub struct AddressSpace {
	pub rom: Vec<u8>,
	pub vram: [u8; 0x2000], // VRAM locking is not emulated as there is not PPU present.
	pub sram: Vec<[u8; 0x2000]>,
	pub wram: [u8; 0x1000 * 8],
	// Accessing echo ram will throw a warning.
	pub oam: [u8; 0x100], // This includes the 105 unused bytes of OAM; they will throw a warning.
	// All MMIO registers are special-cased; many serve no function.
}

impl AddressSpace {
	pub fn read(&self, address: u16) -> u8 {
		let address = address as usize;
		match address {
			0x0000..=0x3FFF => self.rom[address],
			_ => panic!("Unimplemented address range for {address}")
		}
	}

	pub fn write(&mut self, address: u16, value: u8) {
		let address = address as usize;
		match address {
			0x0000..=0x3FFF => self.rom[address] = value,
			_ => panic!("Unimplemented address range for {address}")
		};
	}

	pub fn open(path: &String) -> Result<AddressSpace, Error> {
		let mut rom = fs::read(path)?;
		if rom.len() < 0x4000 {
			rom.resize_with(0x4000, || {0xFF} );
		}
		Ok(AddressSpace{
			rom,
			vram: [0; 0x2000],
			sram: vec!(),
			wram: [0; 0x1000 * 8],
			oam: [0; 0x100],
		})
	}
}