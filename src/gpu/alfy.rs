#[doc = "Register `ALFY` reader"]
pub type R = crate::R<AlfySpec>;
#[doc = "Register `ALFY` writer"]
pub type W = crate::W<AlfySpec>;
#[doc = "Field `ALFY` reader - Added alfa value for each step at y-axis"]
pub type AlfyR = crate::FieldReader<u32>;
#[doc = "Field `ALFY` writer - Added alfa value for each step at y-axis"]
pub type AlfyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added alfa value for each step at y-axis"]
    #[inline(always)]
    pub fn alfy(&self) -> AlfyR {
        AlfyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added alfa value for each step at y-axis"]
    #[inline(always)]
    #[must_use]
    pub fn alfy(&mut self) -> AlfyW<AlfySpec> {
        AlfyW::new(self, 0)
    }
}
#[doc = "Added alfa value for each step at y-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`alfy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alfy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlfySpec;
impl crate::RegisterSpec for AlfySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alfy::R`](R) reader structure"]
impl crate::Readable for AlfySpec {}
#[doc = "`write(|w| ..)` method takes [`alfy::W`](W) writer structure"]
impl crate::Writable for AlfySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALFY to value 0"]
impl crate::Resettable for AlfySpec {
    const RESET_VALUE: u32 = 0;
}
