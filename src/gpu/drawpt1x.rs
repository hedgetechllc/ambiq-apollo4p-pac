#[doc = "Register `DRAWPT1X` reader"]
pub type R = crate::R<Drawpt1xSpec>;
#[doc = "Register `DRAWPT1X` writer"]
pub type W = crate::W<Drawpt1xSpec>;
#[doc = "Field `DRAW1X` reader - X coordinate"]
pub type Draw1xR = crate::FieldReader<u32>;
#[doc = "Field `DRAW1X` writer - X coordinate"]
pub type Draw1xW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - X coordinate"]
    #[inline(always)]
    pub fn draw1x(&self) -> Draw1xR {
        Draw1xR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - X coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn draw1x(&mut self) -> Draw1xW<Drawpt1xSpec> {
        Draw1xW::new(self, 0)
    }
}
#[doc = "X coordinate of Vertex 1 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt1x::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt1x::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt1xSpec;
impl crate::RegisterSpec for Drawpt1xSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt1x::R`](R) reader structure"]
impl crate::Readable for Drawpt1xSpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt1x::W`](W) writer structure"]
impl crate::Writable for Drawpt1xSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT1X to value 0"]
impl crate::Resettable for Drawpt1xSpec {
    const RESET_VALUE: u32 = 0;
}
