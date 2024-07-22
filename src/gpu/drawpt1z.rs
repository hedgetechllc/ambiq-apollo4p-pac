#[doc = "Register `DRAWPT1Z` reader"]
pub type R = crate::R<Drawpt1zSpec>;
#[doc = "Register `DRAWPT1Z` writer"]
pub type W = crate::W<Drawpt1zSpec>;
#[doc = "Field `DRAW1Z` reader - This bitfield is reserved."]
pub type Draw1zR = crate::FieldReader<u32>;
#[doc = "Field `DRAW1Z` writer - This bitfield is reserved."]
pub type Draw1zW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bitfield is reserved."]
    #[inline(always)]
    pub fn draw1z(&self) -> Draw1zR {
        Draw1zR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn draw1z(&mut self) -> Draw1zW<Drawpt1zSpec> {
        Draw1zW::new(self, 0)
    }
}
#[doc = "DRAWPT1Z register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt1z::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt1z::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt1zSpec;
impl crate::RegisterSpec for Drawpt1zSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt1z::R`](R) reader structure"]
impl crate::Readable for Drawpt1zSpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt1z::W`](W) writer structure"]
impl crate::Writable for Drawpt1zSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT1Z to value 0"]
impl crate::Resettable for Drawpt1zSpec {
    const RESET_VALUE: u32 = 0;
}
