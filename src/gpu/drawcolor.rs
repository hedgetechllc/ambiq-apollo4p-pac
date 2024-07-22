#[doc = "Register `DRAWCOLOR` reader"]
pub type R = crate::R<DrawcolorSpec>;
#[doc = "Register `DRAWCOLOR` writer"]
pub type W = crate::W<DrawcolorSpec>;
#[doc = "Field `RASTPRIM` reader - Rasterizer drawing"]
pub type RastprimR = crate::FieldReader<u32>;
#[doc = "Field `RASTPRIM` writer - Rasterizer drawing"]
pub type RastprimW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rasterizer drawing"]
    #[inline(always)]
    pub fn rastprim(&self) -> RastprimR {
        RastprimR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rasterizer drawing"]
    #[inline(always)]
    #[must_use]
    pub fn rastprim(&mut self) -> RastprimW<DrawcolorSpec> {
        RastprimW::new(self, 0)
    }
}
#[doc = "DRAWCOLOR register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawcolor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawcolor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrawcolorSpec;
impl crate::RegisterSpec for DrawcolorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawcolor::R`](R) reader structure"]
impl crate::Readable for DrawcolorSpec {}
#[doc = "`write(|w| ..)` method takes [`drawcolor::W`](W) writer structure"]
impl crate::Writable for DrawcolorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWCOLOR to value 0"]
impl crate::Resettable for DrawcolorSpec {
    const RESET_VALUE: u32 = 0;
}
