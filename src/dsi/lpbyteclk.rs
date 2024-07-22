#[doc = "Register `LPBYTECLK` reader"]
pub type R = crate::R<LpbyteclkSpec>;
#[doc = "Register `LPBYTECLK` writer"]
pub type W = crate::W<LpbyteclkSpec>;
#[doc = "Field `VALBYTECLK` reader - The value programmed in this register is equal to the number of byte clocks occupied in one low power clock; this value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc)"]
pub type ValbyteclkR = crate::FieldReader<u16>;
#[doc = "Field `VALBYTECLK` writer - The value programmed in this register is equal to the number of byte clocks occupied in one low power clock; this value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc)"]
pub type ValbyteclkW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The value programmed in this register is equal to the number of byte clocks occupied in one low power clock; this value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc)"]
    #[inline(always)]
    pub fn valbyteclk(&self) -> ValbyteclkR {
        ValbyteclkR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value programmed in this register is equal to the number of byte clocks occupied in one low power clock; this value is based on the byte clock (txbyteclkhs) and low power clock frequency (txclkesc)"]
    #[inline(always)]
    #[must_use]
    pub fn valbyteclk(&mut self) -> ValbyteclkW<LpbyteclkSpec> {
        ValbyteclkW::new(self, 0)
    }
}
#[doc = "Low power clock equivalence in terms of byte clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbyteclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbyteclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpbyteclkSpec;
impl crate::RegisterSpec for LpbyteclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpbyteclk::R`](R) reader structure"]
impl crate::Readable for LpbyteclkSpec {}
#[doc = "`write(|w| ..)` method takes [`lpbyteclk::W`](W) writer structure"]
impl crate::Writable for LpbyteclkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPBYTECLK to value 0x01"]
impl crate::Resettable for LpbyteclkSpec {
    const RESET_VALUE: u32 = 0x01;
}
