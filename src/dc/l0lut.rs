#[doc = "Register `L0LUT` reader"]
pub type R = crate::R<L0lutSpec>;
#[doc = "Register `L0LUT` writer"]
pub type W = crate::W<L0lutSpec>;
#[doc = "Field `L0LUT0GAMRAMPB` reader - Gamma ramp blue bits"]
pub type L0lut0gamrampbR = crate::FieldReader;
#[doc = "Field `L0LUT0GAMRAMPB` writer - Gamma ramp blue bits"]
pub type L0lut0gamrampbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `L0LUT0GAMRAMPG` reader - Gamma ramp green bits"]
pub type L0lut0gamrampgR = crate::FieldReader;
#[doc = "Field `L0LUT0GAMRAMPG` writer - Gamma ramp green bits"]
pub type L0lut0gamrampgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `L0LUT0GAMRAMPR` reader - Gamma ramp red bits"]
pub type L0lut0gamramprR = crate::FieldReader;
#[doc = "Field `L0LUT0GAMRAMPR` writer - Gamma ramp red bits"]
pub type L0lut0gamramprW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `L0LUT0GAMRAMPA` reader - Gamma ramp alpha bits"]
pub type L0lut0gamrampaR = crate::FieldReader;
#[doc = "Field `L0LUT0GAMRAMPA` writer - Gamma ramp alpha bits"]
pub type L0lut0gamrampaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Gamma ramp blue bits"]
    #[inline(always)]
    pub fn l0lut0gamrampb(&self) -> L0lut0gamrampbR {
        L0lut0gamrampbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Gamma ramp green bits"]
    #[inline(always)]
    pub fn l0lut0gamrampg(&self) -> L0lut0gamrampgR {
        L0lut0gamrampgR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Gamma ramp red bits"]
    #[inline(always)]
    pub fn l0lut0gamrampr(&self) -> L0lut0gamramprR {
        L0lut0gamramprR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Gamma ramp alpha bits"]
    #[inline(always)]
    pub fn l0lut0gamrampa(&self) -> L0lut0gamrampaR {
        L0lut0gamrampaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Gamma ramp blue bits"]
    #[inline(always)]
    #[must_use]
    pub fn l0lut0gamrampb(&mut self) -> L0lut0gamrampbW<L0lutSpec> {
        L0lut0gamrampbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Gamma ramp green bits"]
    #[inline(always)]
    #[must_use]
    pub fn l0lut0gamrampg(&mut self) -> L0lut0gamrampgW<L0lutSpec> {
        L0lut0gamrampgW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Gamma ramp red bits"]
    #[inline(always)]
    #[must_use]
    pub fn l0lut0gamrampr(&mut self) -> L0lut0gamramprW<L0lutSpec> {
        L0lut0gamramprW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Gamma ramp alpha bits"]
    #[inline(always)]
    #[must_use]
    pub fn l0lut0gamrampa(&mut self) -> L0lut0gamrampaW<L0lutSpec> {
        L0lut0gamrampaW::new(self, 24)
    }
}
#[doc = "A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]. Layer 0 palette,gamma correction memory region where x starts at 0 thru 255.\n\nYou can [`read`](crate::Reg::read) this register and get [`l0lut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0lut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0lutSpec;
impl crate::RegisterSpec for L0lutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0lut::R`](R) reader structure"]
impl crate::Readable for L0lutSpec {}
#[doc = "`write(|w| ..)` method takes [`l0lut::W`](W) writer structure"]
impl crate::Writable for L0lutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L0LUT to value 0"]
impl crate::Resettable for L0lutSpec {
    const RESET_VALUE: u32 = 0;
}
