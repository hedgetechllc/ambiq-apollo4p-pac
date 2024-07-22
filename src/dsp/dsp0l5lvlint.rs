#[doc = "Register `DSP0L5LVLINT` reader"]
pub type R = crate::R<Dsp0l5lvlintSpec>;
#[doc = "Register `DSP0L5LVLINT` writer"]
pub type W = crate::W<Dsp0l5lvlintSpec>;
#[doc = "Field `DSP0L5LVLINT` reader - DSP 0 L5 Level Interrupt Mux"]
pub type Dsp0l5lvlintR = crate::FieldReader<u32>;
#[doc = "Field `DSP0L5LVLINT` writer - DSP 0 L5 Level Interrupt Mux"]
pub type Dsp0l5lvlintW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - DSP 0 L5 Level Interrupt Mux"]
    #[inline(always)]
    pub fn dsp0l5lvlint(&self) -> Dsp0l5lvlintR {
        Dsp0l5lvlintR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - DSP 0 L5 Level Interrupt Mux"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0l5lvlint(&mut self) -> Dsp0l5lvlintW<Dsp0l5lvlintSpec> {
        Dsp0l5lvlintW::new(self, 0)
    }
}
#[doc = "DSP 0 L5 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0l5lvlint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0l5lvlint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0l5lvlintSpec;
impl crate::RegisterSpec for Dsp0l5lvlintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0l5lvlint::R`](R) reader structure"]
impl crate::Readable for Dsp0l5lvlintSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0l5lvlint::W`](W) writer structure"]
impl crate::Writable for Dsp0l5lvlintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0L5LVLINT to value 0"]
impl crate::Resettable for Dsp0l5lvlintSpec {
    const RESET_VALUE: u32 = 0;
}
