#[doc = "Register `DRAWPT0X` reader"]
pub type R = crate::R<Drawpt0xSpec>;
#[doc = "Register `DRAWPT0X` writer"]
pub type W = crate::W<Drawpt0xSpec>;
#[doc = "Field `DRAW0X` reader - X coordinate"]
pub type Draw0xR = crate::FieldReader<u32>;
#[doc = "Field `DRAW0X` writer - X coordinate"]
pub type Draw0xW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - X coordinate"]
    #[inline(always)]
    pub fn draw0x(&self) -> Draw0xR {
        Draw0xR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - X coordinate"]
    #[inline(always)]
    #[must_use]
    pub fn draw0x(&mut self) -> Draw0xW<Drawpt0xSpec> {
        Draw0xW::new(self, 0)
    }
}
#[doc = "X coordinate of Vertex 0 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt0x::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt0x::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Drawpt0xSpec;
impl crate::RegisterSpec for Drawpt0xSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drawpt0x::R`](R) reader structure"]
impl crate::Readable for Drawpt0xSpec {}
#[doc = "`write(|w| ..)` method takes [`drawpt0x::W`](W) writer structure"]
impl crate::Writable for Drawpt0xSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRAWPT0X to value 0"]
impl crate::Resettable for Drawpt0xSpec {
    const RESET_VALUE: u32 = 0;
}
