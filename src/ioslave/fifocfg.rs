#[doc = "Register `FIFOCFG` reader"]
pub type R = crate::R<FifocfgSpec>;
#[doc = "Register `FIFOCFG` writer"]
pub type W = crate::W<FifocfgSpec>;
#[doc = "Field `FIFOBASE` reader - These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
pub type FifobaseR = crate::FieldReader;
#[doc = "Field `FIFOBASE` writer - These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
pub type FifobaseW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FIFOMAX` reader - These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
pub type FifomaxR = crate::FieldReader;
#[doc = "Field `FIFOMAX` writer - These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
pub type FifomaxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ROBASE` reader - Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOBASE*8-1)"]
pub type RobaseR = crate::FieldReader;
#[doc = "Field `ROBASE` writer - Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOBASE*8-1)"]
pub type RobaseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:4 - These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
    #[inline(always)]
    pub fn fifobase(&self) -> FifobaseR {
        FifobaseR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
    #[inline(always)]
    pub fn fifomax(&self) -> FifomaxR {
        FifomaxR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOBASE*8-1)"]
    #[inline(always)]
    pub fn robase(&self) -> RobaseR {
        RobaseR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
    #[inline(always)]
    #[must_use]
    pub fn fifobase(&mut self) -> FifobaseW<FifocfgSpec> {
        FifobaseW::new(self, 0)
    }
    #[doc = "Bits 8:13 - These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
    #[inline(always)]
    #[must_use]
    pub fn fifomax(&mut self) -> FifomaxW<FifocfgSpec> {
        FifomaxW::new(self, 8)
    }
    #[doc = "Bits 24:29 - Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOBASE*8-1)"]
    #[inline(always)]
    #[must_use]
    pub fn robase(&mut self) -> RobaseW<FifocfgSpec> {
        RobaseW::new(self, 24)
    }
}
#[doc = "FIFO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`fifocfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifocfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifocfgSpec;
impl crate::RegisterSpec for FifocfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifocfg::R`](R) reader structure"]
impl crate::Readable for FifocfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fifocfg::W`](W) writer structure"]
impl crate::Writable for FifocfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOCFG to value 0x2000_0000"]
impl crate::Resettable for FifocfgSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
