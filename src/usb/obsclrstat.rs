#[doc = "Register `OBSCLRSTAT` reader"]
pub type R = crate::R<ObsclrstatSpec>;
#[doc = "Register `OBSCLRSTAT` writer"]
pub type W = crate::W<ObsclrstatSpec>;
#[doc = "Field `CLRSTAT` reader - Writing a 1 to this bit clears all bits in the UTMISTICKYSTATUS register."]
pub type ClrstatR = crate::BitReader;
#[doc = "Field `CLRSTAT` writer - Writing a 1 to this bit clears all bits in the UTMISTICKYSTATUS register."]
pub type ClrstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing a 1 to this bit clears all bits in the UTMISTICKYSTATUS register."]
    #[inline(always)]
    pub fn clrstat(&self) -> ClrstatR {
        ClrstatR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bit clears all bits in the UTMISTICKYSTATUS register."]
    #[inline(always)]
    #[must_use]
    pub fn clrstat(&mut self) -> ClrstatW<ObsclrstatSpec> {
        ClrstatW::new(self, 0)
    }
}
#[doc = "Clears all bits in the sticky obs status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`obsclrstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obsclrstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ObsclrstatSpec;
impl crate::RegisterSpec for ObsclrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obsclrstat::R`](R) reader structure"]
impl crate::Readable for ObsclrstatSpec {}
#[doc = "`write(|w| ..)` method takes [`obsclrstat::W`](W) writer structure"]
impl crate::Writable for ObsclrstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OBSCLRSTAT to value 0"]
impl crate::Resettable for ObsclrstatSpec {
    const RESET_VALUE: u32 = 0;
}
