#[doc = "Register `VSYNCCNT` reader"]
pub type R = crate::R<VsynccntSpec>;
#[doc = "Register `VSYNCCNT` writer"]
pub type W = crate::W<VsynccntSpec>;
#[doc = "Field `VSC` reader - Shows the vertical sync value in terms of lines. Min value - 2 Max value - any 12 bit value greater than 2 based on DPI resolution"]
pub type VscR = crate::FieldReader<u16>;
#[doc = "Field `VSC` writer - Shows the vertical sync value in terms of lines. Min value - 2 Max value - any 12 bit value greater than 2 based on DPI resolution"]
pub type VscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the vertical sync value in terms of lines. Min value - 2 Max value - any 12 bit value greater than 2 based on DPI resolution"]
    #[inline(always)]
    pub fn vsc(&self) -> VscR {
        VscR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shows the vertical sync value in terms of lines. Min value - 2 Max value - any 12 bit value greater than 2 based on DPI resolution"]
    #[inline(always)]
    #[must_use]
    pub fn vsc(&mut self) -> VscW<VsynccntSpec> {
        VscW::new(self, 0)
    }
}
#[doc = "Shows the vertical sync value\n\nYou can [`read`](crate::Reg::read) this register and get [`vsynccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vsynccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VsynccntSpec;
impl crate::RegisterSpec for VsynccntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsynccnt::R`](R) reader structure"]
impl crate::Readable for VsynccntSpec {}
#[doc = "`write(|w| ..)` method takes [`vsynccnt::W`](W) writer structure"]
impl crate::Writable for VsynccntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSYNCCNT to value 0"]
impl crate::Resettable for VsynccntSpec {
    const RESET_VALUE: u32 = 0;
}
