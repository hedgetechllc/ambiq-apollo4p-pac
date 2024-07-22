#[doc = "Register `DRAWPT2Z` reader"]
pub type R = crate::R<Drawpt2zSpec>;
#[doc = "Register `DRAWPT2Z` writer"]
pub type W = crate::W<Drawpt2zSpec>;
#[doc = "Field `RSVD` reader - This bitfield is reserved."]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - This bitfield is reserved."]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bitfield is reserved."]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<Drawpt2zSpec> {
        RsvdW::new(self, 0)
    }
}
#[doc = "DRAWPT2Z register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt2z::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt2z::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt2zSpec;
impl crate::RegisterSpec for Drawpt2zSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt2z::R`](R) reader structure"]
impl crate::Readable for Drawpt2zSpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt2z::W`](W) writer structure"]
impl crate::Writable for Drawpt2zSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT2Z to value 0"]
impl crate::Resettable for Drawpt2zSpec {
    const RESET_VALUE: u32 = 0;
}
