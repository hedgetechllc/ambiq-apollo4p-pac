#[doc = "Register `PRENC` reader"]
pub type R = crate::R<PrencSpec>;
#[doc = "Register `PRENC` writer"]
pub type W = crate::W<PrencSpec>;
#[doc = "Field `PRENC` reader - These bits hold the priority encode of the REGACC interrupts."]
pub type PrencR = crate::FieldReader;
#[doc = "Field `PRENC` writer - These bits hold the priority encode of the REGACC interrupts."]
pub type PrencW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - These bits hold the priority encode of the REGACC interrupts."]
    #[inline(always)]
    pub fn prenc(&self) -> PrencR {
        PrencR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - These bits hold the priority encode of the REGACC interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn prenc(&mut self) -> PrencW<PrencSpec> {
        PrencW::new(self, 0)
    }
}
#[doc = "I/O Slave Interrupt Priority Encode\n\nYou can [`read`](crate::Reg::read) this register and get [`prenc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prenc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrencSpec;
impl crate::RegisterSpec for PrencSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prenc::R`](R) reader structure"]
impl crate::Readable for PrencSpec {}
#[doc = "`write(|w| ..)` method takes [`prenc::W`](W) writer structure"]
impl crate::Writable for PrencSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRENC to value 0"]
impl crate::Resettable for PrencSpec {
    const RESET_VALUE: u32 = 0;
}
