#[doc = "Register `ZXLIM` reader"]
pub type R = crate::R<ZxlimSpec>;
#[doc = "Register `ZXLIM` writer"]
pub type W = crate::W<ZxlimSpec>;
#[doc = "Field `LZXC` reader - Sets the lower integer sample limit for the ZX comparator. Note that these values are raw AUDADC values whose bounds are specified by PRMODE but not maniupulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value. Samples must enter the range between UZXC and LZXC in order for a zero crossing to be recognized."]
pub type LzxcR = crate::FieldReader<u16>;
#[doc = "Field `LZXC` writer - Sets the lower integer sample limit for the ZX comparator. Note that these values are raw AUDADC values whose bounds are specified by PRMODE but not maniupulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value. Samples must enter the range between UZXC and LZXC in order for a zero crossing to be recognized."]
pub type LzxcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `UZXC` reader - Sets the upper integer sample limit for the ZX comparator. Note that these values are raw AUDADC values whose bounds are specified by PRMODE but not maniupulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value. Samples must enter the range between UZXC and LZXC in order for a zero crossing to be recognized."]
pub type UzxcR = crate::FieldReader<u16>;
#[doc = "Field `UZXC` writer - Sets the upper integer sample limit for the ZX comparator. Note that these values are raw AUDADC values whose bounds are specified by PRMODE but not maniupulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value. Samples must enter the range between UZXC and LZXC in order for a zero crossing to be recognized."]
pub type UzxcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Sets the lower integer sample limit for the ZX comparator. Note that these values are raw AUDADC values whose bounds are specified by PRMODE but not maniupulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value. Samples must enter the range between UZXC and LZXC in order for a zero crossing to be recognized."]
    #[inline(always)]
    pub fn lzxc(&self) -> LzxcR {
        LzxcR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Sets the upper integer sample limit for the ZX comparator. Note that these values are raw AUDADC values whose bounds are specified by PRMODE but not maniupulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value. Samples must enter the range between UZXC and LZXC in order for a zero crossing to be recognized."]
    #[inline(always)]
    pub fn uzxc(&self) -> UzxcR {
        UzxcR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Sets the lower integer sample limit for the ZX comparator. Note that these values are raw AUDADC values whose bounds are specified by PRMODE but not maniupulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value. Samples must enter the range between UZXC and LZXC in order for a zero crossing to be recognized."]
    #[inline(always)]
    #[must_use]
    pub fn lzxc(&mut self) -> LzxcW<ZxlimSpec> {
        LzxcW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Sets the upper integer sample limit for the ZX comparator. Note that these values are raw AUDADC values whose bounds are specified by PRMODE but not maniupulated by accumulate/divide logic. Therefore, there is no oversampling and no binary point in this value. Samples must enter the range between UZXC and LZXC in order for a zero crossing to be recognized."]
    #[inline(always)]
    #[must_use]
    pub fn uzxc(&mut self) -> UzxcW<ZxlimSpec> {
        UzxcW::new(self, 16)
    }
}
#[doc = "Zero Crossing Comparator Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`zxlim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zxlim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZxlimSpec;
impl crate::RegisterSpec for ZxlimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zxlim::R`](R) reader structure"]
impl crate::Readable for ZxlimSpec {}
#[doc = "`write(|w| ..)` method takes [`zxlim::W`](W) writer structure"]
impl crate::Writable for ZxlimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ZXLIM to value 0"]
impl crate::Resettable for ZxlimSpec {
    const RESET_VALUE: u32 = 0;
}
