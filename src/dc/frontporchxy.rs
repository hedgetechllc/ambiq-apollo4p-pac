#[doc = "Register `FRONTPORCHXY` reader"]
pub type R = crate::R<FrontporchxySpec>;
#[doc = "Register `FRONTPORCHXY` writer"]
pub type W = crate::W<FrontporchxySpec>;
#[doc = "Field `FLINES` reader - Specify the number of lines for the front porch Y dimension"]
pub type FlinesR = crate::FieldReader<u16>;
#[doc = "Field `FLINES` writer - Specify the number of lines for the front porch Y dimension"]
pub type FlinesW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FPCLKCYCLES` reader - Specify the pixel clock cycles for the front porch X dimension"]
pub type FpclkcyclesR = crate::FieldReader<u16>;
#[doc = "Field `FPCLKCYCLES` writer - Specify the pixel clock cycles for the front porch X dimension"]
pub type FpclkcyclesW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specify the number of lines for the front porch Y dimension"]
    #[inline(always)]
    pub fn flines(&self) -> FlinesR {
        FlinesR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specify the pixel clock cycles for the front porch X dimension"]
    #[inline(always)]
    pub fn fpclkcycles(&self) -> FpclkcyclesR {
        FpclkcyclesR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specify the number of lines for the front porch Y dimension"]
    #[inline(always)]
    #[must_use]
    pub fn flines(&mut self) -> FlinesW<FrontporchxySpec> {
        FlinesW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Specify the pixel clock cycles for the front porch X dimension"]
    #[inline(always)]
    #[must_use]
    pub fn fpclkcycles(&mut self) -> FpclkcyclesW<FrontporchxySpec> {
        FpclkcyclesW::new(self, 16)
    }
}
#[doc = "Specifies the X and Y dimensions for the Front Porch.\n\nYou can [`read`](crate::Reg::read) this register and get [`frontporchxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frontporchxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrontporchxySpec;
impl crate::RegisterSpec for FrontporchxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frontporchxy::R`](R) reader structure"]
impl crate::Readable for FrontporchxySpec {}
#[doc = "`write(|w| ..)` method takes [`frontporchxy::W`](W) writer structure"]
impl crate::Writable for FrontporchxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRONTPORCHXY to value 0"]
impl crate::Resettable for FrontporchxySpec {
    const RESET_VALUE: u32 = 0;
}
