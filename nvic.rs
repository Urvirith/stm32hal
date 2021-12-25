
// NVIC DRIVER
// ARM NVIC MODULE
// NVIC Description - is on pg 529
// NVIC Registers (Programming Manual) - is on 178

/* Nested vectored interrupt controller (NVIC) */

use super::pointer;

const U32SIZE:						u32 = 32;

#[repr(C)]
struct NVICReg {
	pub iser:		[u32; 16],		/* Interrupt Set-Enable register */
	pub reserved0:	[u32; 16],
	pub icer:		[u32; 16], 		/* Interrupt Clear-Enable register */
	pub reserved1:	[u32; 16],
	pub ispr:		[u32; 16],    	/* Interrupt Set-Pending Registers */
	pub reserved2:	[u32; 16],
	pub icpr:		[u32; 16],	    /* Interrupt Clear-Pending Registers */
	pub reserved3:	[u32; 16],
	pub iabr:		[u32; 16], 		/* Interrupt Active Bit Registers */
    pub reserved4:	[u32; 16],
	pub itns:		[u32; 16],	    /* Interrupt Target Non-Secure Registers */
	pub reserved5:	[u32; 16],
	pub ipr:		[u8; 480],	    /* Interrupt Priority Registers */
}

pub struct Nvic {
	registers: &'static mut NVICReg
}

impl Nvic {
	pub fn init(base: u32) -> Nvic {
		return Nvic {
			registers:	unsafe{ &mut *(base as *mut NVICReg) }
		};
	}

	pub fn set_interrupt(&mut self, irq_num: u32) {
		if self.get_reg(irq_num) < self.registers.iser.len() {
            pointer::set_ptr_vol_bit_u32(&mut (*self).registers.iser[self.get_reg(irq_num)], self.get_bit(irq_num));
		}
	}

	pub fn set_priority(&mut self, irq_num: u32, priority: u8, sub_priority: u8) {
		if irq_num < self.registers.ipr.len() as u32 {
            pointer::set_ptr_vol_raw_u8(&mut (*self).registers.ipr[irq_num as usize], self.get_priority(priority, sub_priority));
		}
	}

	fn get_reg(&self, irq_num: u32) -> usize {
		return (irq_num / U32SIZE) as usize;
	}

	fn get_bit(&self, irq_num: u32) -> u32 {
		return 1 << (irq_num % U32SIZE);
	}

	fn get_priority(&self, priority: u8, sub_priority: u8) -> u8 {
		return ((priority) << (6)) | (((sub_priority) & (0x3)) << (4));
	}
}