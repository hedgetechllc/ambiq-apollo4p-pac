#[doc = "Register `DRAWPT1Y` reader"]
pub type R = crate::R<Drawpt1ySpec>;
#[doc = "Register `DRAWPT1Y` writer"]
pub type W = crate::W<Drawpt1ySpec>;
#[doc = "Field `DRAW1Y` reader - Y coordinate"]
pub type Draw1yR = crate::FieldReader<u32>;
#[doc = "Field `DRAW1Y` writer - Y coordinate"]
pub type Draw1yW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Y coordinate"]
    #[inline(always)]
    pub fn draw1y(&self) -> Draw1yR {
        Draw1yR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Y coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn draw1y(&mut self) -> Draw1yW<Drawpt1ySpec> {
        Draw1yW::new(self, 0)
    }
}
#[doc = "Y coordinate of Vertex 1 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt1y::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt1y::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt1ySpec;
impl crate::RegisterSpec for Drawpt1ySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt1y::R`](R) reader structure"]
impl crate::Readable for Drawpt1ySpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt1y::W`](W) writer structure"]
impl crate::Writable for Drawpt1ySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT1Y to value 0"]
impl crate::Resettable for Drawpt1ySpec {
    const RESET_VALUE: u32 = 0;
}
