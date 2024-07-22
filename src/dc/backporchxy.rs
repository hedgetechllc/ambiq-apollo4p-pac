#[doc = "Register `BACKPORCHXY` reader"]
pub type R = crate::R<BackporchxySpec>;
#[doc = "Register `BACKPORCHXY` writer"]
pub type W = crate::W<BackporchxySpec>;
#[doc = "Field `BLINES` reader - Specify the number of lines for the back porch Y dimension"]
pub type BlinesR = crate::FieldReader<u16>;
#[doc = "Field `BLINES` writer - Specify the number of lines for the back porch Y dimension"]
pub type BlinesW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BPCLKCYCLES` reader - Specify the pixel clock cycles for the back porch X dimension"]
pub type BpclkcyclesR = crate::FieldReader<u16>;
#[doc = "Field `BPCLKCYCLES` writer - Specify the pixel clock cycles for the back porch X dimension"]
pub type BpclkcyclesW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the number of lines for the back porch Y dimension"]
    #[inline(always)]
    pub fn blines(&self) -> BlinesR {
        BlinesR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the pixel clock cycles for the back porch X dimension"]
    #[inline(always)]
    pub fn bpclkcycles(&self) -> BpclkcyclesR {
        BpclkcyclesR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the number of lines for the back porch Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn blines(&mut self) -> BlinesW<BackporchxySpec> {
        BlinesW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the pixel clock cycles for the back porch X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn bpclkcycles(&mut self) -> BpclkcyclesW<BackporchxySpec> {
        BpclkcyclesW::new(self, 16)
    }
}
#[doc = "Specifies the X and Y dimensions for the Back Porch.\n\nYou can [`read`](crate::Reg::read) this register and get [`backporchxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backporchxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BackporchxySpec;
impl crate::RegisterSpec for BackporchxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backporchxy::R`](R) reader structure"]
impl crate::Readable for BackporchxySpec {}
#[doc = "`write(|w| ..)` method takes [`backporchxy::W`](W) writer structure"]
impl crate::Writable for BackporchxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BACKPORCHXY to value 0"]
impl crate::Resettable for BackporchxySpec {
    const RESET_VALUE: u32 = 0;
}
