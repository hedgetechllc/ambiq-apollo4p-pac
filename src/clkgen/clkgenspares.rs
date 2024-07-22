#[doc = "Register `CLKGENSPARES` reader"]
pub type R = crate::R<ClkgensparesSpec>;
#[doc = "Register `CLKGENSPARES` writer"]
pub type W = crate::W<ClkgensparesSpec>;
#[doc = "Field `CLKGENSPARES` reader - Placeholer spare registes that can be used as needed for future use"]
pub type ClkgensparesR = crate::FieldReader<u32>;
#[doc = "Field `CLKGENSPARES` writer - Placeholer spare registes that can be used as needed for future use"]
pub type ClkgensparesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Placeholer spare registes that can be used as needed for future use"]
    #[inline(always)]
    pub fn clkgenspares(&self) -> ClkgensparesR {
        ClkgensparesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Placeholer spare registes that can be used as needed for future use"]
    #[inline(always)]
    #[must_use]
    pub fn clkgenspares(&mut self) -> ClkgensparesW<ClkgensparesSpec> {
        ClkgensparesW::new(self, 0)
    }
}
#[doc = "CLKGEN Spare Regs\n\nYou can [`read`](crate::Reg::read) this register and get [`clkgenspares::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkgenspares::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgensparesSpec;
impl crate::RegisterSpec for ClkgensparesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgenspares::R`](R) reader structure"]
impl crate::Readable for ClkgensparesSpec {}
#[doc = "`write(|w| ..)` method takes [`clkgenspares::W`](W) writer structure"]
impl crate::Writable for ClkgensparesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGENSPARES to value 0"]
impl crate::Resettable for ClkgensparesSpec {
    const RESET_VALUE: u32 = 0;
}
