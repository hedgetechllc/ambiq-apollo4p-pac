#[doc = "Register `TEX1COLOR` reader"]
pub type R = crate::R<Tex1colorSpec>;
#[doc = "Register `TEX1COLOR` writer"]
pub type W = crate::W<Tex1colorSpec>;
#[doc = "Field `RED` reader - red value"]
pub type RedR = crate::FieldReader;
#[doc = "Field `RED` writer - red value"]
pub type RedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GREEN` reader - green value"]
pub type GreenR = crate::FieldReader;
#[doc = "Field `GREEN` writer - green value"]
pub type GreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLUE` reader - blue value"]
pub type BlueR = crate::FieldReader;
#[doc = "Field `BLUE` writer - blue value"]
pub type BlueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ALPHA` reader - alpha value"]
pub type AlphaR = crate::FieldReader;
#[doc = "Field `ALPHA` writer - alpha value"]
pub type AlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - red value"]
    #[inline(always)]
    pub fn red(&self) -> RedR {
        RedR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - green value"]
    #[inline(always)]
    pub fn green(&self) -> GreenR {
        GreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - blue value"]
    #[inline(always)]
    pub fn blue(&self) -> BlueR {
        BlueR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - alpha value"]
    #[inline(always)]
    pub fn alpha(&self) -> AlphaR {
        AlphaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - red value"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RedW<Tex1colorSpec> {
        RedW::new(self, 0)
    }
    #[doc = "Bits 8:15 - green value"]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GreenW<Tex1colorSpec> {
        GreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - blue value"]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BlueW<Tex1colorSpec> {
        BlueW::new(self, 16)
    }
    #[doc = "Bits 24:31 - alpha value"]
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> AlphaW<Tex1colorSpec> {
        AlphaW::new(self, 24)
    }
}
#[doc = "Texture maps default color.Used with luminance and alpha-only color formats.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex1color::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex1color::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tex1colorSpec;
impl crate::RegisterSpec for Tex1colorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tex1color::R`](R) reader structure"]
impl crate::Readable for Tex1colorSpec {}
#[doc = "`write(|w| ..)` method takes [`tex1color::W`](W) writer structure"]
impl crate::Writable for Tex1colorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEX1COLOR to value 0"]
impl crate::Resettable for Tex1colorSpec {
    const RESET_VALUE: u32 = 0;
}
