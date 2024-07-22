#[doc = "Register `CONTEXTID` reader"]
pub type R = crate::R<ContextidSpec>;
#[doc = "Register `CONTEXTID` writer"]
pub type W = crate::W<ContextidSpec>;
#[doc = "Field `CONTEXTID` reader - Context ID"]
pub type ContextidR = crate::FieldReader;
#[doc = "Field `CONTEXTID` writer - Context ID"]
pub type ContextidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Context ID"]
    #[inline(always)]
    pub fn contextid(&self) -> ContextidR {
        ContextidR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Context ID"]
    #[inline(always)]
    #[must_use]
    pub fn contextid(&mut self) -> ContextidW<ContextidSpec> {
        ContextidW::new(self, 0)
    }
}
#[doc = "A general RD_WR register. For Firmware use.\n\nYou can [`read`](crate::Reg::read) this register and get [`contextid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`contextid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ContextidSpec;
impl crate::RegisterSpec for ContextidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`contextid::R`](R) reader structure"]
impl crate::Readable for ContextidSpec {}
#[doc = "`write(|w| ..)` method takes [`contextid::W`](W) writer structure"]
impl crate::Writable for ContextidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTEXTID to value 0"]
impl crate::Resettable for ContextidSpec {
    const RESET_VALUE: u32 = 0;
}
