#[doc = "Register `DRAWPT0Y` reader"]
pub type R = crate::R<Drawpt0ySpec>;
#[doc = "Register `DRAWPT0Y` writer"]
pub type W = crate::W<Drawpt0ySpec>;
#[doc = "Field `DRAW0Y` reader - Y coordinate"]
pub type Draw0yR = crate::FieldReader<u32>;
#[doc = "Field `DRAW0Y` writer - Y coordinate"]
pub type Draw0yW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Y coordinate"]
    #[inline(always)]
    pub fn draw0y(&self) -> Draw0yR {
        Draw0yR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Y coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn draw0y(&mut self) -> Draw0yW<Drawpt0ySpec> {
        Draw0yW::new(self, 0)
    }
}
#[doc = "Y coordinate of Vertex 0 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt0y::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt0y::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt0ySpec;
impl crate::RegisterSpec for Drawpt0ySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt0y::R`](R) reader structure"]
impl crate::Readable for Drawpt0ySpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt0y::W`](W) writer structure"]
impl crate::Writable for Drawpt0ySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT0Y to value 0"]
impl crate::Resettable for Drawpt0ySpec {
    const RESET_VALUE: u32 = 0;
}
