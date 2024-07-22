#[doc = "Register `TEX2BASE` reader"]
pub type R = crate::R<Tex2baseSpec>;
#[doc = "Register `TEX2BASE` writer"]
pub type W = crate::W<Tex2baseSpec>;
#[doc = "Field `Drawing` reader - surface 2 Base address of the drawing surface 2"]
pub type DrawingR = crate::FieldReader<u32>;
#[doc = "Field `Drawing` writer - surface 2 Base address of the drawing surface 2"]
pub type DrawingW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - surface 2 Base address of the drawing surface 2"]
    #[inline(always)]
    pub fn drawing(&self) -> DrawingR {
        DrawingR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - surface 2 Base address of the drawing surface 2"]
    #[inline(always)]
    #[must_use]
    pub fn drawing(&mut self) -> DrawingW<Tex2baseSpec> {
        DrawingW::new(self, 0)
    }
}
#[doc = "Base address of the drawing surface 2 (must be word aligned).\n\nYou can [`read`](crate::Reg::read) this register and get [`tex2base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex2base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tex2baseSpec;
impl crate::RegisterSpec for Tex2baseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tex2base::R`](R) reader structure"]
impl crate::Readable for Tex2baseSpec {}
#[doc = "`write(|w| ..)` method takes [`tex2base::W`](W) writer structure"]
impl crate::Writable for Tex2baseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEX2BASE to value 0"]
impl crate::Resettable for Tex2baseSpec {
    const RESET_VALUE: u32 = 0;
}
