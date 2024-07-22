#[doc = "Register `DRAWPT2Y` reader"]
pub type R = crate::R<Drawpt2ySpec>;
#[doc = "Register `DRAWPT2Y` writer"]
pub type W = crate::W<Drawpt2ySpec>;
#[doc = "Field `DRAW2Y` reader - Y coordinate"]
pub type Draw2yR = crate::FieldReader<u32>;
#[doc = "Field `DRAW2Y` writer - Y coordinate"]
pub type Draw2yW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Y coordinate"]
    #[inline(always)]
    pub fn draw2y(&self) -> Draw2yR {
        Draw2yR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Y coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn draw2y(&mut self) -> Draw2yW<Drawpt2ySpec> {
        Draw2yW::new(self, 0)
    }
}
#[doc = "Y coordinate of Vertex 2 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt2y::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt2y::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt2ySpec;
impl crate::RegisterSpec for Drawpt2ySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt2y::R`](R) reader structure"]
impl crate::Readable for Drawpt2ySpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt2y::W`](W) writer structure"]
impl crate::Writable for Drawpt2ySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT2Y to value 0"]
impl crate::Resettable for Drawpt2ySpec {
    const RESET_VALUE: u32 = 0;
}
