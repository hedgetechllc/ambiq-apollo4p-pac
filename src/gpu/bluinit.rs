#[doc = "Register `BLUINIT` reader"]
pub type R = crate::R<BluinitSpec>;
#[doc = "Register `BLUINIT` writer"]
pub type W = crate::W<BluinitSpec>;
#[doc = "Field `BLUEXY` reader - Blue value of STARTXY pixel"]
pub type BluexyR = crate::FieldReader<u32>;
#[doc = "Field `BLUEXY` writer - Blue value of STARTXY pixel"]
pub type BluexyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Blue value of STARTXY pixel"]
    #[inline(always)]
    pub fn bluexy(&self) -> BluexyR {
        BluexyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Blue value of STARTXY pixel"]
    #[inline(always)]
    #[must_use]
    pub fn bluexy(&mut self) -> BluexyW<BluinitSpec> {
        BluexyW::new(self, 0)
    }
}
#[doc = "Blue value of STARTXY pixel, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`bluinit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bluinit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BluinitSpec;
impl crate::RegisterSpec for BluinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bluinit::R`](R) reader structure"]
impl crate::Readable for BluinitSpec {}
#[doc = "`write(|w| ..)` method takes [`bluinit::W`](W) writer structure"]
impl crate::Writable for BluinitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLUINIT to value 0"]
impl crate::Resettable for BluinitSpec {
    const RESET_VALUE: u32 = 0;
}
