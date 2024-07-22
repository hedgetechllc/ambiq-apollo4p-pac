#[doc = "Register `L2LUT0` reader"]
pub type R = crate::R<L2lut0Spec>;
#[doc = "Register `L2LUT0` writer"]
pub type W = crate::W<L2lut0Spec>;
#[doc = "Field `L2LUT0GAMRAMPB` reader - Gamma ramp blue bits"]
pub type L2lut0gamrampbR = crate::FieldReader;
#[doc = "Field `L2LUT0GAMRAMPB` writer - Gamma ramp blue bits"]
pub type L2lut0gamrampbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `L2LUT0GAMRAMPG` reader - Gamma ramp green bits"]
pub type L2lut0gamrampgR = crate::FieldReader;
#[doc = "Field `L2LUT0GAMRAMPG` writer - Gamma ramp green bits"]
pub type L2lut0gamrampgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `L2LUT0GAMRAMPR` reader - Gamma ramp red bits"]
pub type L2lut0gamramprR = crate::FieldReader;
#[doc = "Field `L2LUT0GAMRAMPR` writer - Gamma ramp red bits"]
pub type L2lut0gamramprW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `L2LUT0GAMRAMPA` reader - Gamma ramp alpha bits"]
pub type L2lut0gamrampaR = crate::FieldReader;
#[doc = "Field `L2LUT0GAMRAMPA` writer - Gamma ramp alpha bits"]
pub type L2lut0gamrampaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Gamma ramp blue bits"]
    #[inline(always)]
    pub fn l2lut0gamrampb(&self) -> L2lut0gamrampbR {
        L2lut0gamrampbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Gamma ramp green bits"]
    #[inline(always)]
    pub fn l2lut0gamrampg(&self) -> L2lut0gamrampgR {
        L2lut0gamrampgR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Gamma ramp red bits"]
    #[inline(always)]
    pub fn l2lut0gamrampr(&self) -> L2lut0gamramprR {
        L2lut0gamramprR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Gamma ramp alpha bits"]
    #[inline(always)]
    pub fn l2lut0gamrampa(&self) -> L2lut0gamrampaR {
        L2lut0gamrampaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Gamma ramp blue bits"]
    #[inline(always)]
    #[must_use]
    pub fn l2lut0gamrampb(&mut self) -> L2lut0gamrampbW<L2lut0Spec> {
        L2lut0gamrampbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Gamma ramp green bits"]
    #[inline(always)]
    #[must_use]
    pub fn l2lut0gamrampg(&mut self) -> L2lut0gamrampgW<L2lut0Spec> {
        L2lut0gamrampgW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Gamma ramp red bits"]
    #[inline(always)]
    #[must_use]
    pub fn l2lut0gamrampr(&mut self) -> L2lut0gamramprW<L2lut0Spec> {
        L2lut0gamramprW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Gamma ramp alpha bits"]
    #[inline(always)]
    #[must_use]
    pub fn l2lut0gamrampa(&mut self) -> L2lut0gamrampaW<L2lut0Spec> {
        L2lut0gamrampaW::new(self, 24)
    }
}
#[doc = "A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 2 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L2LUT(n) (*((volatile uint32_t*)(&amp;L2LUT + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`l2lut0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2lut0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2lut0Spec;
impl crate::RegisterSpec for L2lut0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2lut0::R`](R) reader structure"]
impl crate::Readable for L2lut0Spec {}
#[doc = "`write(|w| ..)` method takes [`l2lut0::W`](W) writer structure"]
impl crate::Writable for L2lut0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2LUT0 to value 0"]
impl crate::Resettable for L2lut0Spec {
    const RESET_VALUE: u32 = 0;
}
