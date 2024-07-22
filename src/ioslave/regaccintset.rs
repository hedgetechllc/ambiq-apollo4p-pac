#[doc = "Register `REGACCINTSET` reader"]
pub type R = crate::R<RegaccintsetSpec>;
#[doc = "Register `REGACCINTSET` writer"]
pub type W = crate::W<RegaccintsetSpec>;
#[doc = "Field `REGACC` reader - Register access interrupts."]
pub type RegaccR = crate::FieldReader<u32>;
#[doc = "Field `REGACC` writer - Register access interrupts."]
pub type RegaccW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register access interrupts."]
    #[inline(always)]
    pub fn regacc(&self) -> RegaccR {
        RegaccR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register access interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn regacc(&mut self) -> RegaccW<RegaccintsetSpec> {
        RegaccW::new(self, 0)
    }
}
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`regaccintset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regaccintset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegaccintsetSpec;
impl crate::RegisterSpec for RegaccintsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regaccintset::R`](R) reader structure"]
impl crate::Readable for RegaccintsetSpec {}
#[doc = "`write(|w| ..)` method takes [`regaccintset::W`](W) writer structure"]
impl crate::Writable for RegaccintsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGACCINTSET to value 0"]
impl crate::Resettable for RegaccintsetSpec {
    const RESET_VALUE: u32 = 0;
}
