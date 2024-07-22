#[doc = "Register `GLLUT` reader"]
pub type R = crate::R<GllutSpec>;
#[doc = "Register `GLLUT` writer"]
pub type W = crate::W<GllutSpec>;
#[doc = "Field `GLLUT0GAMRAMPB` reader - Gamma ramp blue bits"]
pub type Gllut0gamrampbR = crate::FieldReader;
#[doc = "Field `GLLUT0GAMRAMPB` writer - Gamma ramp blue bits"]
pub type Gllut0gamrampbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GLLUT0GAMRAMPG` reader - Gamma ramp green bits"]
pub type Gllut0gamrampgR = crate::FieldReader;
#[doc = "Field `GLLUT0GAMRAMPG` writer - Gamma ramp green bits"]
pub type Gllut0gamrampgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GLLUT0GAMRAMPR` reader - Gamma ramp red bits"]
pub type Gllut0gamramprR = crate::FieldReader;
#[doc = "Field `GLLUT0GAMRAMPR` writer - Gamma ramp red bits"]
pub type Gllut0gamramprW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Gamma ramp blue bits"]
    #[inline(always)]
    pub fn gllut0gamrampb(&self) -> Gllut0gamrampbR {
        Gllut0gamrampbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Gamma ramp green bits"]
    #[inline(always)]
    pub fn gllut0gamrampg(&self) -> Gllut0gamrampgR {
        Gllut0gamrampgR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Gamma ramp red bits"]
    #[inline(always)]
    pub fn gllut0gamrampr(&self) -> Gllut0gamramprR {
        Gllut0gamramprR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Gamma ramp blue bits"]
    #[inline(always)]
    #[must_use]
    pub fn gllut0gamrampb(&mut self) -> Gllut0gamrampbW<GllutSpec> {
        Gllut0gamrampbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Gamma ramp green bits"]
    #[inline(always)]
    #[must_use]
    pub fn gllut0gamrampg(&mut self) -> Gllut0gamrampgW<GllutSpec> {
        Gllut0gamrampgW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Gamma ramp red bits"]
    #[inline(always)]
    #[must_use]
    pub fn gllut0gamrampr(&mut self) -> Gllut0gamramprW<GllutSpec> {
        Gllut0gamramprW::new(self, 16)
    }
}
#[doc = "R\\[0\\]G\\[0\\]B\\[0\\]
thru R\\[255\\]G\\[255\\]B\\[255\\]
Global palette, gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L0LUT(n) (*((volatile uint32_t*)(&amp;L0LUT + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`gllut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gllut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GllutSpec;
impl crate::RegisterSpec for GllutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gllut::R`](R) reader structure"]
impl crate::Readable for GllutSpec {}
#[doc = "`write(|w| ..)` method takes [`gllut::W`](W) writer structure"]
impl crate::Writable for GllutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLLUT to value 0"]
impl crate::Resettable for GllutSpec {
    const RESET_VALUE: u32 = 0;
}
