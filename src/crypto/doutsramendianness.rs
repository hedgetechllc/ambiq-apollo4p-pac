#[doc = "Register `DOUTSRAMENDIANNESS` reader"]
pub type R = crate::R<DoutsramendiannessSpec>;
#[doc = "Register `DOUTSRAMENDIANNESS` writer"]
pub type W = crate::W<DoutsramendiannessSpec>;
#[doc = "Defines the endianness of DOUT interface from SRAM:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutsramendianness {
    #[doc = "1: big-endianness"]
    Be = 1,
    #[doc = "0: little endianness"]
    Le = 0,
}
impl From<Doutsramendianness> for bool {
    #[inline(always)]
    fn from(variant: Doutsramendianness) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSRAMENDIANNESS` reader - Defines the endianness of DOUT interface from SRAM:"]
pub type DoutsramendiannessR = crate::BitReader<Doutsramendianness>;
impl DoutsramendiannessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doutsramendianness {
        match self.bits {
            true => Doutsramendianness::Be,
            false => Doutsramendianness::Le,
        }
    }
    #[doc = "big-endianness"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == Doutsramendianness::Be
    }
    #[doc = "little endianness"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == Doutsramendianness::Le
    }
}
#[doc = "Field `DOUTSRAMENDIANNESS` writer - Defines the endianness of DOUT interface from SRAM:"]
pub type DoutsramendiannessW<'a, REG> = crate::BitWriter<'a, REG, Doutsramendianness>;
impl<'a, REG> DoutsramendiannessW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "big-endianness"]
    #[inline(always)]
    pub fn be(self) -> &'a mut crate::W<REG> {
        self.variant(Doutsramendianness::Be)
    }
    #[doc = "little endianness"]
    #[inline(always)]
    pub fn le(self) -> &'a mut crate::W<REG> {
        self.variant(Doutsramendianness::Le)
    }
}
impl R {
    #[doc = "Bit 0 - Defines the endianness of DOUT interface from SRAM:"]
    #[inline(always)]
    pub fn doutsramendianness(&self) -> DoutsramendiannessR {
        DoutsramendiannessR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Defines the endianness of DOUT interface from SRAM:"]
    #[inline(always)]
    #[must_use]
    pub fn doutsramendianness(&mut self) -> DoutsramendiannessW<DoutsramendiannessSpec> {
        DoutsramendiannessW::new(self, 0)
    }
}
#[doc = "This register defines the endianness of the DOUT interface from SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`doutsramendianness::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutsramendianness::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutsramendiannessSpec;
impl crate::RegisterSpec for DoutsramendiannessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutsramendianness::R`](R) reader structure"]
impl crate::Readable for DoutsramendiannessSpec {}
#[doc = "`write(|w| ..)` method takes [`doutsramendianness::W`](W) writer structure"]
impl crate::Writable for DoutsramendiannessSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTSRAMENDIANNESS to value 0"]
impl crate::Resettable for DoutsramendiannessSpec {
    const RESET_VALUE: u32 = 0;
}
