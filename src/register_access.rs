use crate::{Error, Si4703};
use embedded_hal::blocking::i2c;

const DEVICE_ADDRESS: u8 = 0x10;

pub struct Register;
impl Register {
    pub const DEVICE_ID: usize = 0x0;
    pub const CHIP_ID: usize = 0x1;
    pub const POWERCFG: usize = 0x2;
    pub const CHANNEL: usize = 0x3;
    pub const SYSCONFIG1: usize = 0x4;
    pub const SYSCONFIG2: usize = 0x5;
    pub const SYSCONFIG3: usize = 0x6;
    pub const TEST1: usize = 0x7;
    pub const STATUSRSSI: usize = 0xA;
    pub const READCHAN: usize = 0xB;
    pub const RDSA: usize = 0xC;
    pub const RDSB: usize = 0xD;
    pub const RDSC: usize = 0xE;
    pub const RDSD: usize = 0xF;
}

pub struct BitFlags;
impl BitFlags {
    pub const DSMUTE: u16 = 1 << 15;
    pub const DMUTE: u16 = 1 << 14;
    pub const MONO: u16 = 1 << 13;
    pub const RDSR: u16 = 1 << 15;
    pub const STC: u16 = 1 << 14;
    pub const SF_BL: u16 = 1 << 13;
    pub const AFCRL: u16 = 1 << 12;
    pub const RDSS: u16 = 1 << 11;
    pub const ST: u16 = 1 << 8;
    pub const DE: u16 = 1 << 11;
    pub const SKMODE: u16 = 1 << 10;
    pub const SEEKUP: u16 = 1 << 9;
    pub const SEEK: u16 = 1 << 8;
    pub const ENABLE: u16 = 1;
    pub const DISABLE: u16 = 1 << 6;
    pub const RDSIEN: u16 = 1 << 15;
    pub const STCIEN: u16 = 1 << 14;
    pub const RDS: u16 = 1 << 12;
    pub const AGCD: u16 = 1 << 10;
    pub const RDSM: u16 = 1 << 11;
    pub const VOLEXT: u16 = 1 << 8;
    pub const XOSCEN: u16 = 1 << 15;
    pub const AHIZEN: u16 = 1 << 14;
    pub const TUNE: u16 = 1 << 15;
    pub const BLERA1: u16 = 1 << 10;
    pub const BLERA0: u16 = 1 << 9;
    pub const BLERB1: u16 = 1 << 15;
    pub const BLERB0: u16 = 1 << 14;
    pub const BLERC1: u16 = 1 << 13;
    pub const BLERC0: u16 = 1 << 12;
    pub const BLERD1: u16 = 1 << 11;
    pub const BLERD0: u16 = 1 << 10;
}

impl<I2C, E, IC> Si4703<I2C, IC>
where
    I2C: i2c::Write<Error = E> + i2c::Read<Error = E>,
{
    pub(crate) fn read_status(&mut self) -> Result<u16, Error<E>> {
        let mut data = [0; 4];
        self.i2c
            .read(DEVICE_ADDRESS, &mut data)
            .map_err(Error::I2C)?;
        Ok(u16::from(data[0]) << 8 | u16::from(data[1]))
    }

    pub(crate) fn read_rds(&mut self) -> Result<[u16; 16], Error<E>> {
        self.read_some_registers_bare_err(6).map_err(Error::I2C)
    }

    pub(crate) fn read_powercfg(&mut self) -> Result<u16, Error<E>> {
        self.read_powercfg_bare_err().map_err(Error::I2C)
    }

    pub(crate) fn read_powercfg_bare_err(&mut self) -> Result<u16, E> {
        let registers = self.read_some_registers_bare_err(9)?;
        Ok(registers[Register::POWERCFG])
    }

    pub(crate) fn read_some_registers_bare_err(&mut self, count: usize) -> Result<[u16; 16], E> {
        const OFFSET: usize = 0xA;
        let mut data = [0; 32];
        self.i2c.read(DEVICE_ADDRESS, &mut data[..count * 2])?;
        Ok(to_registers(data, OFFSET))
    }

    pub(crate) fn read_registers(&mut self) -> Result<[u16; 16], Error<E>> {
        self.read_registers_bare_err().map_err(Error::I2C)
    }

    pub(crate) fn read_registers_bare_err(&mut self) -> Result<[u16; 16], E> {
        const OFFSET: usize = 0xA;
        let mut data = [0; 32];
        self.i2c.read(DEVICE_ADDRESS, &mut data)?;
        let registers = to_registers(data, OFFSET);
        Ok(registers)
    }

    pub(crate) fn write_powercfg(&mut self, value: u16) -> Result<(), Error<E>> {
        self.write_powercfg_bare_err(value).map_err(Error::I2C)
    }

    pub(crate) fn write_powercfg_bare_err(&mut self, value: u16) -> Result<(), E> {
        let data = [(value >> 8) as u8, value as u8];
        self.i2c.write(DEVICE_ADDRESS, &data)
    }

    pub(crate) fn write_registers(&mut self, registers: &[u16]) -> Result<(), Error<E>> {
        self.write_registers_bare_err(registers).map_err(Error::I2C)
    }

    pub(crate) fn write_registers_bare_err(&mut self, registers: &[u16]) -> Result<(), E> {
        const OFFSET: usize = 0x2;
        let data = from_registers(registers, OFFSET);
        self.i2c
            .write(DEVICE_ADDRESS, &data[..((registers.len() - OFFSET) * 2)])
    }
}

fn to_registers(data: [u8; 32], offset: usize) -> [u16; 16] {
    let mut registers = [0; 16];
    for i in 0..registers.len() {
        registers[(i + offset) % registers.len()] =
            u16::from(data[2 * i]) << 8 | u16::from(data[2 * i + 1])
    }
    registers
}

fn from_registers(registers: &[u16], offset: usize) -> [u8; 32] {
    let mut data = [0; 32];
    for i in 0..registers.len() {
        let reg = registers[(i + offset) % registers.len()];
        data[2 * i] = (reg >> 8) as u8;
        data[2 * i + 1] = reg as u8
    }
    data
}

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;
    use super::*;
    const DATA: [u8; 32] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0x10, 0x11, 0x12, 0x13, 0x14,
        0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    ];
    const REGS: [u16; 16] = [
        1, 0x203, 0x405, 0x607, 0x809, 0xA0B, 0xC0D, 0xE0F, 0x1011, 0x1213, 0x1415, 0x1617, 0x1819,
        0x1A1B, 0x1C1D, 0x1E1F,
    ];
    #[test]
    fn can_convert_to_registers() {
        let registers = to_registers(DATA, 0xA);
        const SHIFTED_REGS: [u16; 16] = [
            0xC0D, 0xE0F, 0x1011, 0x1213, 0x1415, 0x1617, 0x1819, 0x1A1B, 0x1C1D, 0x1E1F, 1, 0x203,
            0x405, 0x607, 0x809, 0xA0B,
        ];
        assert_eq!(registers, SHIFTED_REGS)
    }

    #[test]
    fn can_convert_from_registers() {
        let data = from_registers(&REGS, 0x2);
        const SHIFTED_DATA: [u8; 32] = [
            4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0, 1, 2, 3,
        ];
        assert_eq!(data, SHIFTED_DATA)
    }
}
