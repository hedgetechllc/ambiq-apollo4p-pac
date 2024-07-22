#[doc = "Register `DSP1L2LVLINT` reader"]
pub type R = crate::R<Dsp1l2lvlintSpec>;
#[doc = "Register `DSP1L2LVLINT` writer"]
pub type W = crate::W<Dsp1l2lvlintSpec>;
#[doc = "Field `DSP1L2LVLINT` reader - DSP 1 L2 Level Interrupt Mux"]
pub type Dsp1l2lvlintR = crate::FieldReader<u32>;
#[doc = "Field `DSP1L2LVLINT` writer - DSP 1 L2 Level Interrupt Mux"]
pub type Dsp1l2lvlintW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - DSP 1 L2 Level Interrupt Mux"]
    #[inline(always)]
    pub fn dsp1l2lvlint(&self) -> Dsp1l2lvlintR {
        Dsp1l2lvlintR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - DSP 1 L2 Level Interrupt Mux"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1l2lvlint(&mut self) -> Dsp1l2lvlintW<Dsp1l2lvlintSpec> {
        Dsp1l2lvlintW::new(self, 0)
    }
}
#[doc = "DSP 1 L2 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1l2lvlint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1l2lvlint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1l2lvlintSpec;
impl crate::RegisterSpec for Dsp1l2lvlintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1l2lvlint::R`](R) reader structure"]
impl crate::Readable for Dsp1l2lvlintSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1l2lvlint::W`](W) writer structure"]
impl crate::Writable for Dsp1l2lvlintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1L2LVLINT to value 0"]
impl crate::Resettable for Dsp1l2lvlintSpec {
    const RESET_VALUE: u32 = 0;
}
