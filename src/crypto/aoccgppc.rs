#[doc = "Register `AOCCGPPC` reader"]
pub type R = crate::R<AoccgppcSpec>;
#[doc = "Register `AOCCGPPC` writer"]
pub type W = crate::W<AoccgppcSpec>;
#[doc = "Field `AOCCGPPC` reader - The AO_CC_GPPC value"]
pub type AoccgppcR = crate::FieldReader;
#[doc = "Field `AOCCGPPC` writer - The AO_CC_GPPC value"]
pub type AoccgppcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The AO_CC_GPPC value"]
    #[inline(always)]
    pub fn aoccgppc(&self) -> AoccgppcR {
        AoccgppcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The AO_CC_GPPC value"]
    #[inline(always)]
    #[must_use]
    pub fn aoccgppc(&mut self) -> AoccgppcW<AoccgppcSpec> {
        AoccgppcW::new(self, 0)
    }
}
#[doc = "holds the AO_CC_GPPC value from AONote: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aoccgppc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoccgppc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AoccgppcSpec;
impl crate::RegisterSpec for AoccgppcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoccgppc::R`](R) reader structure"]
impl crate::Readable for AoccgppcSpec {}
#[doc = "`write(|w| ..)` method takes [`aoccgppc::W`](W) writer structure"]
impl crate::Writable for AoccgppcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AOCCGPPC to value 0"]
impl crate::Resettable for AoccgppcSpec {
    const RESET_VALUE: u32 = 0;
}
