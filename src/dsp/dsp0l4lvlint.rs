#[doc = "Register `DSP0L4LVLINT` reader"]
pub type R = crate::R<Dsp0l4lvlintSpec>;
#[doc = "Register `DSP0L4LVLINT` writer"]
pub type W = crate::W<Dsp0l4lvlintSpec>;
#[doc = "Field `DSP0L4LVLINT` reader - DSP 0 L4 Level Interrupt Mux"]
pub type Dsp0l4lvlintR = crate::FieldReader<u32>;
#[doc = "Field `DSP0L4LVLINT` writer - DSP 0 L4 Level Interrupt Mux"]
pub type Dsp0l4lvlintW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - DSP 0 L4 Level Interrupt Mux"]
    #[inline(always)]
    pub fn dsp0l4lvlint(&self) -> Dsp0l4lvlintR {
        Dsp0l4lvlintR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - DSP 0 L4 Level Interrupt Mux"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0l4lvlint(&mut self) -> Dsp0l4lvlintW<Dsp0l4lvlintSpec> {
        Dsp0l4lvlintW::new(self, 0)
    }
}
#[doc = "DSP 0 L4 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0l4lvlint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0l4lvlint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0l4lvlintSpec;
impl crate::RegisterSpec for Dsp0l4lvlintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0l4lvlint::R`](R) reader structure"]
impl crate::Readable for Dsp0l4lvlintSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0l4lvlint::W`](W) writer structure"]
impl crate::Writable for Dsp0l4lvlintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0L4LVLINT to value 0"]
impl crate::Resettable for Dsp0l4lvlintSpec {
    const RESET_VALUE: u32 = 0;
}
