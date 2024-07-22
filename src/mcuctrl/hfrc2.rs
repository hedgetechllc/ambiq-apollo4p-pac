#[doc = "Register `HFRC2` reader"]
pub type R = crate::R<Hfrc2Spec>;
#[doc = "Register `HFRC2` writer"]
pub type W = crate::W<Hfrc2Spec>;
#[doc = "Field `HF2TUNE` reader - Default HFRC2 frequency tune value"]
pub type Hf2tuneR = crate::FieldReader<u16>;
#[doc = "Field `HF2TUNE` writer - Default HFRC2 frequency tune value"]
pub type Hf2tuneW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 11:21 - Default HFRC2 frequency tune value"]
    #[inline(always)]
    pub fn hf2tune(&self) -> Hf2tuneR {
        Hf2tuneR::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 11:21 - Default HFRC2 frequency tune value"]
    #[inline(always)]
    #[must_use]
    pub fn hf2tune(&mut self) -> Hf2tuneW<Hfrc2Spec> {
        Hf2tuneW::new(self, 11)
    }
}
#[doc = "HFRC2 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfrc2Spec;
impl crate::RegisterSpec for Hfrc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfrc2::R`](R) reader structure"]
impl crate::Readable for Hfrc2Spec {}
#[doc = "`write(|w| ..)` method takes [`hfrc2::W`](W) writer structure"]
impl crate::Writable for Hfrc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFRC2 to value 0"]
impl crate::Resettable for Hfrc2Spec {
    const RESET_VALUE: u32 = 0;
}
