#[doc = "Register `DRAWPT2X` reader"]
pub type R = crate::R<Drawpt2xSpec>;
#[doc = "Register `DRAWPT2X` writer"]
pub type W = crate::W<Drawpt2xSpec>;
#[doc = "Field `DRAW2X` reader - X coordinate"]
pub type Draw2xR = crate::FieldReader<u32>;
#[doc = "Field `DRAW2X` writer - X coordinate"]
pub type Draw2xW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - X coordinate"]
    #[inline(always)]
    pub fn draw2x(&self) -> Draw2xR {
        Draw2xR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - X coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn draw2x(&mut self) -> Draw2xW<Drawpt2xSpec> {
        Draw2xW::new(self, 0)
    }
}
#[doc = "X coordinate of Vertex 2 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt2x::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt2x::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt2xSpec;
impl crate::RegisterSpec for Drawpt2xSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt2x::R`](R) reader structure"]
impl crate::Readable for Drawpt2xSpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt2x::W`](W) writer structure"]
impl crate::Writable for Drawpt2xSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT2X to value 0"]
impl crate::Resettable for Drawpt2xSpec {
    const RESET_VALUE: u32 = 0;
}
