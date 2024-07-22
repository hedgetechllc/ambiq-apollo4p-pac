#[doc = "Register `DSP1L3LVLINT` reader"]
pub type R = crate::R<Dsp1l3lvlintSpec>;
#[doc = "Register `DSP1L3LVLINT` writer"]
pub type W = crate::W<Dsp1l3lvlintSpec>;
#[doc = "Field `DSP1L3LVLINT` reader - DSP 1 L3 Level Interrupt Mux"]
pub type Dsp1l3lvlintR = crate::FieldReader<u32>;
#[doc = "Field `DSP1L3LVLINT` writer - DSP 1 L3 Level Interrupt Mux"]
pub type Dsp1l3lvlintW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - DSP 1 L3 Level Interrupt Mux"]
    #[inline(always)]
    pub fn dsp1l3lvlint(&self) -> Dsp1l3lvlintR {
        Dsp1l3lvlintR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - DSP 1 L3 Level Interrupt Mux"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1l3lvlint(&mut self) -> Dsp1l3lvlintW<Dsp1l3lvlintSpec> {
        Dsp1l3lvlintW::new(self, 0)
    }
}
#[doc = "DSP 1 L3 Level Interrupt Mux\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1l3lvlint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1l3lvlint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1l3lvlintSpec;
impl crate::RegisterSpec for Dsp1l3lvlintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1l3lvlint::R`](R) reader structure"]
impl crate::Readable for Dsp1l3lvlintSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1l3lvlint::W`](W) writer structure"]
impl crate::Writable for Dsp1l3lvlintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1L3LVLINT to value 0"]
impl crate::Resettable for Dsp1l3lvlintSpec {
    const RESET_VALUE: u32 = 0;
}
