#[doc = "Register `DRAWPT3X` reader"]
pub type R = crate::R<Drawpt3xSpec>;
#[doc = "Register `DRAWPT3X` writer"]
pub type W = crate::W<Drawpt3xSpec>;
#[doc = "Field `DRAW3X` reader - X coordinate"]
pub type Draw3xR = crate::FieldReader<u32>;
#[doc = "Field `DRAW3X` writer - X coordinate"]
pub type Draw3xW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - X coordinate"]
    #[inline(always)]
    pub fn draw3x(&self) -> Draw3xR {
        Draw3xR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - X coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn draw3x(&mut self) -> Draw3xW<Drawpt3xSpec> {
        Draw3xW::new(self, 0)
    }
}
#[doc = "X coordinate of Vertex 3 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt3x::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt3x::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt3xSpec;
impl crate::RegisterSpec for Drawpt3xSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt3x::R`](R) reader structure"]
impl crate::Readable for Drawpt3xSpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt3x::W`](W) writer structure"]
impl crate::Writable for Drawpt3xSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT3X to value 0"]
impl crate::Resettable for Drawpt3xSpec {
    const RESET_VALUE: u32 = 0;
}
