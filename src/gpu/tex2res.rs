#[doc = "Register `TEX2RES` reader"]
pub type R = crate::R<Tex2resSpec>;
#[doc = "Register `TEX2RES` writer"]
pub type W = crate::W<Tex2resSpec>;
#[doc = "Field `RESX` reader - resolution X size"]
pub type ResxR = crate::FieldReader<u16>;
#[doc = "Field `RESX` writer - resolution X size"]
pub type ResxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESY` reader - resolution Y size"]
pub type ResyR = crate::FieldReader<u16>;
#[doc = "Field `RESY` writer - resolution Y size"]
pub type ResyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - resolution X size"]
    #[inline(always)]
    pub fn resx(&self) -> ResxR {
        ResxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - resolution Y size"]
    #[inline(always)]
    pub fn resy(&self) -> ResyR {
        ResyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - resolution X size"]
    #[inline(always)]
    #[must_use]
    pub fn resx(&mut self) -> ResxW<Tex2resSpec> {
        ResxW::new(self, 0)
    }
    #[doc = "Bits 16:31 - resolution Y size"]
    #[inline(always)]
    #[must_use]
    pub fn resy(&mut self) -> ResyW<Tex2resSpec> {
        ResyW::new(self, 16)
    }
}
#[doc = "Image 2 resolution.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex2res::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex2res::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tex2resSpec;
impl crate::RegisterSpec for Tex2resSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tex2res::R`](R) reader structure"]
impl crate::Readable for Tex2resSpec {}
#[doc = "`write(|w| ..)` method takes [`tex2res::W`](W) writer structure"]
impl crate::Writable for Tex2resSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEX2RES to value 0"]
impl crate::Resettable for Tex2resSpec {
    const RESET_VALUE: u32 = 0;
}
