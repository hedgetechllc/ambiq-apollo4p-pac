#[doc = "Register `DRAWPT0Z` reader"]
pub type R = crate::R<Drawpt0zSpec>;
#[doc = "Register `DRAWPT0Z` writer"]
pub type W = crate::W<Drawpt0zSpec>;
#[doc = "Field `DRAW0Z` reader - This bitfield is reserved."]
pub type Draw0zR = crate::FieldReader<u32>;
#[doc = "Field `DRAW0Z` writer - This bitfield is reserved."]
pub type Draw0zW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bitfield is reserved."]
    #[inline(always)]
    pub fn draw0z(&self) -> Draw0zR {
        Draw0zR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn draw0z(&mut self) -> Draw0zW<Drawpt0zSpec> {
        Draw0zW::new(self, 0)
    }
}
#[doc = "DRAWPTOX register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt0z::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt0z::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt0zSpec;
impl crate::RegisterSpec for Drawpt0zSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt0z::R`](R) reader structure"]
impl crate::Readable for Drawpt0zSpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt0z::W`](W) writer structure"]
impl crate::Writable for Drawpt0zSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT0Z to value 0"]
impl crate::Resettable for Drawpt0zSpec {
    const RESET_VALUE: u32 = 0;
}
