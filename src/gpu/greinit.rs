#[doc = "Register `GREINIT` reader"]
pub type R = crate::R<GreinitSpec>;
#[doc = "Register `GREINIT` writer"]
pub type W = crate::W<GreinitSpec>;
#[doc = "Field `GREENXY` reader - Green value of STARTXY pixel"]
pub type GreenxyR = crate::FieldReader<u32>;
#[doc = "Field `GREENXY` writer - Green value of STARTXY pixel"]
pub type GreenxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Green value of STARTXY pixel"]
    #[inline(always)]
    pub fn greenxy(&self) -> GreenxyR {
        GreenxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Green value of STARTXY pixel"]
    #[inline(always)]
    #[must_use]
    pub fn greenxy(&mut self) -> GreenxyW<GreinitSpec> {
        GreenxyW::new(self, 0)
    }
}
#[doc = "Green value of STARTXY pixel, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`greinit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`greinit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GreinitSpec;
impl crate::RegisterSpec for GreinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`greinit::R`](R) reader structure"]
impl crate::Readable for GreinitSpec {}
#[doc = "`write(|w| ..)` method takes [`greinit::W`](W) writer structure"]
impl crate::Writable for GreinitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GREINIT to value 0"]
impl crate::Resettable for GreinitSpec {
    const RESET_VALUE: u32 = 0;
}
