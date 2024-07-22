#[doc = "Register `DINSRAMENDIANNESS` reader"]
pub type R = crate::R<DinsramendiannessSpec>;
#[doc = "Register `DINSRAMENDIANNESS` writer"]
pub type W = crate::W<DinsramendiannessSpec>;
#[doc = "Defines the endianness of DIN interface to SRAM:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sramdinendianness {
    #[doc = "1: big-endianness"]
    Be = 1,
    #[doc = "0: little endianness"]
    Le = 0,
}
impl From<Sramdinendianness> for bool {
    #[inline(always)]
    fn from(variant: Sramdinendianness) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMDINENDIANNESS` reader - Defines the endianness of DIN interface to SRAM:"]
pub type SramdinendiannessR = crate::BitReader<Sramdinendianness>;
impl SramdinendiannessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramdinendianness {
        match self.bits {
            true => Sramdinendianness::Be,
            false => Sramdinendianness::Le,
        }
    }
    #[doc = "big-endianness"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == Sramdinendianness::Be
    }
    #[doc = "little endianness"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == Sramdinendianness::Le
    }
}
#[doc = "Field `SRAMDINENDIANNESS` writer - Defines the endianness of DIN interface to SRAM:"]
pub type SramdinendiannessW<'a, REG> = crate::BitWriter<'a, REG, Sramdinendianness>;
impl<'a, REG> SramdinendiannessW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "big-endianness"]
    #[inline(always)]
    pub fn be(self) -> &'a mut crate::W<REG> {
        self.variant(Sramdinendianness::Be)
    }
    #[doc = "little endianness"]
    #[inline(always)]
    pub fn le(self) -> &'a mut crate::W<REG> {
        self.variant(Sramdinendianness::Le)
    }
}
impl R {
    #[doc = "Bit 0 - Defines the endianness of DIN interface to SRAM:"]
    #[inline(always)]
    pub fn sramdinendianness(&self) -> SramdinendiannessR {
        SramdinendiannessR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Defines the endianness of DIN interface to SRAM:"]
    #[inline(always)]
    #[must_use]
    pub fn sramdinendianness(&mut self) -> SramdinendiannessW<DinsramendiannessSpec> {
        SramdinendiannessW::new(self, 0)
    }
}
#[doc = "This register defines the endianness of the DIN interface to SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinsramendianness::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinsramendianness::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinsramendiannessSpec;
impl crate::RegisterSpec for DinsramendiannessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinsramendianness::R`](R) reader structure"]
impl crate::Readable for DinsramendiannessSpec {}
#[doc = "`write(|w| ..)` method takes [`dinsramendianness::W`](W) writer structure"]
impl crate::Writable for DinsramendiannessSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINSRAMENDIANNESS to value 0"]
impl crate::Resettable for DinsramendiannessSpec {
    const RESET_VALUE: u32 = 0;
}
