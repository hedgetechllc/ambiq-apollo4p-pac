#[doc = "Register `PWRWEIGHTSLP` reader"]
pub type R = crate::R<PwrweightslpSpec>;
#[doc = "Register `PWRWEIGHTSLP` writer"]
pub type W = crate::W<PwrweightslpSpec>;
#[doc = "Field `WTDSMCU` reader - Weight used for Deep Sleep mode MCU"]
pub type WtdsmcuR = crate::FieldReader;
#[doc = "Field `WTDSMCU` writer - Weight used for Deep Sleep mode MCU"]
pub type WtdsmcuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for Deep Sleep mode MCU"]
    #[inline(always)]
    pub fn wtdsmcu(&self) -> WtdsmcuR {
        WtdsmcuR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for Deep Sleep mode MCU"]
    #[inline(always)]
    #[must_use]
    pub fn wtdsmcu(&mut self) -> WtdsmcuW<PwrweightslpSpec> {
        WtdsmcuW::new(self, 0)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightslp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightslp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrweightslpSpec;
impl crate::RegisterSpec for PwrweightslpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightslp::R`](R) reader structure"]
impl crate::Readable for PwrweightslpSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightslp::W`](W) writer structure"]
impl crate::Writable for PwrweightslpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTSLP to value 0"]
impl crate::Resettable for PwrweightslpSpec {
    const RESET_VALUE: u32 = 0;
}
