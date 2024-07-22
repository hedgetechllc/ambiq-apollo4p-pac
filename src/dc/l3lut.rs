#[doc = "Register `L3LUT` reader"]
pub type R = crate::R<L3lutSpec>;
#[doc = "Register `L3LUT` writer"]
pub type W = crate::W<L3lutSpec>;
#[doc = "Field `L3LUT0GAMRAMPB` reader - Gamma ramp blue bits"]
pub type L3lut0gamrampbR = crate::FieldReader;
#[doc = "Field `L3LUT0GAMRAMPB` writer - Gamma ramp blue bits"]
pub type L3lut0gamrampbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `L3LUT0GAMRAMPG` reader - Gamma ramp green bits"]
pub type L3lut0gamrampgR = crate::FieldReader;
#[doc = "Field `L3LUT0GAMRAMPG` writer - Gamma ramp green bits"]
pub type L3lut0gamrampgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `L3LUT0GAMRAMPR` reader - Gamma ramp red bits"]
pub type L3lut0gamramprR = crate::FieldReader;
#[doc = "Field `L3LUT0GAMRAMPR` writer - Gamma ramp red bits"]
pub type L3lut0gamramprW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `L3LUT0GAMRAMPA` reader - Gamma ramp alpha bits"]
pub type L3lut0gamrampaR = crate::FieldReader;
#[doc = "Field `L3LUT0GAMRAMPA` writer - Gamma ramp alpha bits"]
pub type L3lut0gamrampaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Gamma ramp blue bits"]
    #[inline(always)]
    pub fn l3lut0gamrampb(&self) -> L3lut0gamrampbR {
        L3lut0gamrampbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Gamma ramp green bits"]
    #[inline(always)]
    pub fn l3lut0gamrampg(&self) -> L3lut0gamrampgR {
        L3lut0gamrampgR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Gamma ramp red bits"]
    #[inline(always)]
    pub fn l3lut0gamrampr(&self) -> L3lut0gamramprR {
        L3lut0gamramprR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Gamma ramp alpha bits"]
    #[inline(always)]
    pub fn l3lut0gamrampa(&self) -> L3lut0gamrampaR {
        L3lut0gamrampaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Gamma ramp blue bits"]
    #[inline(always)]
    #[must_use]
    pub fn l3lut0gamrampb(&mut self) -> L3lut0gamrampbW<L3lutSpec> {
        L3lut0gamrampbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Gamma ramp green bits"]
    #[inline(always)]
    #[must_use]
    pub fn l3lut0gamrampg(&mut self) -> L3lut0gamrampgW<L3lutSpec> {
        L3lut0gamrampgW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Gamma ramp red bits"]
    #[inline(always)]
    #[must_use]
    pub fn l3lut0gamrampr(&mut self) -> L3lut0gamramprW<L3lutSpec> {
        L3lut0gamramprW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Gamma ramp alpha bits"]
    #[inline(always)]
    #[must_use]
    pub fn l3lut0gamrampa(&mut self) -> L3lut0gamrampaW<L3lutSpec> {
        L3lut0gamrampaW::new(self, 24)
    }
}
#[doc = "A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 3 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L3LUT(n) (*((volatile uint32_t*)(&amp;L3LUT + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`l3lut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l3lut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L3lutSpec;
impl crate::RegisterSpec for L3lutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l3lut::R`](R) reader structure"]
impl crate::Readable for L3lutSpec {}
#[doc = "`write(|w| ..)` method takes [`l3lut::W`](W) writer structure"]
impl crate::Writable for L3lutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L3LUT to value 0"]
impl crate::Resettable for L3lutSpec {
    const RESET_VALUE: u32 = 0;
}
