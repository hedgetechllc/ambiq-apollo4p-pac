#[doc = "Register `BGCOLOR` reader"]
pub type R = crate::R<BgcolorSpec>;
#[doc = "Register `BGCOLOR` writer"]
pub type W = crate::W<BgcolorSpec>;
#[doc = "Field `ALPHACOLOR` reader - Color alpha is used as background color"]
pub type AlphacolorR = crate::FieldReader;
#[doc = "Field `ALPHACOLOR` writer - Color alpha is used as background color"]
pub type AlphacolorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLUECOLOR` reader - Color blue is used as background color"]
pub type BluecolorR = crate::FieldReader;
#[doc = "Field `BLUECOLOR` writer - Color blue is used as background color"]
pub type BluecolorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GREENCOLOR` reader - Color green is used as background color"]
pub type GreencolorR = crate::FieldReader;
#[doc = "Field `GREENCOLOR` writer - Color green is used as background color"]
pub type GreencolorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REDCOLOR` reader - Color red is used as background color"]
pub type RedcolorR = crate::FieldReader;
#[doc = "Field `REDCOLOR` writer - Color red is used as background color"]
pub type RedcolorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color alpha is used as background color"]
    #[inline(always)]
    pub fn alphacolor(&self) -> AlphacolorR {
        AlphacolorR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Color blue is used as background color"]
    #[inline(always)]
    pub fn bluecolor(&self) -> BluecolorR {
        BluecolorR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Color green is used as background color"]
    #[inline(always)]
    pub fn greencolor(&self) -> GreencolorR {
        GreencolorR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Color red is used as background color"]
    #[inline(always)]
    pub fn redcolor(&self) -> RedcolorR {
        RedcolorR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color alpha is used as background color"]
    #[inline(always)]
    #[must_use]
    pub fn alphacolor(&mut self) -> AlphacolorW<BgcolorSpec> {
        AlphacolorW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Color blue is used as background color"]
    #[inline(always)]
    #[must_use]
    pub fn bluecolor(&mut self) -> BluecolorW<BgcolorSpec> {
        BluecolorW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Color green is used as background color"]
    #[inline(always)]
    #[must_use]
    pub fn greencolor(&mut self) -> GreencolorW<BgcolorSpec> {
        GreencolorW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Color red is used as background color"]
    #[inline(always)]
    #[must_use]
    pub fn redcolor(&mut self) -> RedcolorW<BgcolorSpec> {
        RedcolorW::new(self, 24)
    }
}
#[doc = "Specifies the main background color.\n\nYou can [`read`](crate::Reg::read) this register and get [`bgcolor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcolor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgcolorSpec;
impl crate::RegisterSpec for BgcolorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgcolor::R`](R) reader structure"]
impl crate::Readable for BgcolorSpec {}
#[doc = "`write(|w| ..)` method takes [`bgcolor::W`](W) writer structure"]
impl crate::Writable for BgcolorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BGCOLOR to value 0"]
impl crate::Resettable for BgcolorSpec {
    const RESET_VALUE: u32 = 0;
}
