#[doc = "Register `REGACCINTEN` reader"]
pub type R = crate::R<RegaccintenSpec>;
#[doc = "Register `REGACCINTEN` writer"]
pub type W = crate::W<RegaccintenSpec>;
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
    pub fn regacc(&mut self) -> RegaccW<RegaccintenSpec> {
        RegaccW::new(self, 0)
    }
}
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`regaccinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regaccinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegaccintenSpec;
impl crate::RegisterSpec for RegaccintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regaccinten::R`](R) reader structure"]
impl crate::Readable for RegaccintenSpec {}
#[doc = "`write(|w| ..)` method takes [`regaccinten::W`](W) writer structure"]
impl crate::Writable for RegaccintenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGACCINTEN to value 0"]
impl crate::Resettable for RegaccintenSpec {
    const RESET_VALUE: u32 = 0;
}
