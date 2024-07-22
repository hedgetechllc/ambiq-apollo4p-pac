#[doc = "Register `POLARITY` reader"]
pub type R = crate::R<PolaritySpec>;
#[doc = "Register `POLARITY` writer"]
pub type W = crate::W<PolaritySpec>;
#[doc = "Polarity bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pbits {
    #[doc = "0: polarity for Vsync"]
    Polv = 0,
    #[doc = "1: Polarity for Hsync"]
    Polh = 1,
    #[doc = "2: Polarity for shut down"]
    Polsd = 2,
    #[doc = "3: Polarity for Color mode"]
    Polcm = 3,
}
impl From<Pbits> for u8 {
    #[inline(always)]
    fn from(variant: Pbits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pbits {
    type Ux = u8;
}
impl crate::IsEnum for Pbits {}
#[doc = "Field `PBITS` reader - Polarity bits"]
pub type PbitsR = crate::FieldReader<Pbits>;
impl PbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pbits> {
        match self.bits {
            0 => Some(Pbits::Polv),
            1 => Some(Pbits::Polh),
            2 => Some(Pbits::Polsd),
            3 => Some(Pbits::Polcm),
            _ => None,
        }
    }
    #[doc = "polarity for Vsync"]
    #[inline(always)]
    pub fn is_polv(&self) -> bool {
        *self == Pbits::Polv
    }
    #[doc = "Polarity for Hsync"]
    #[inline(always)]
    pub fn is_polh(&self) -> bool {
        *self == Pbits::Polh
    }
    #[doc = "Polarity for shut down"]
    #[inline(always)]
    pub fn is_polsd(&self) -> bool {
        *self == Pbits::Polsd
    }
    #[doc = "Polarity for Color mode"]
    #[inline(always)]
    pub fn is_polcm(&self) -> bool {
        *self == Pbits::Polcm
    }
}
#[doc = "Field `PBITS` writer - Polarity bits"]
pub type PbitsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pbits>;
impl<'a, REG> PbitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "polarity for Vsync"]
    #[inline(always)]
    pub fn polv(self) -> &'a mut crate::W<REG> {
        self.variant(Pbits::Polv)
    }
    #[doc = "Polarity for Hsync"]
    #[inline(always)]
    pub fn polh(self) -> &'a mut crate::W<REG> {
        self.variant(Pbits::Polh)
    }
    #[doc = "Polarity for shut down"]
    #[inline(always)]
    pub fn polsd(self) -> &'a mut crate::W<REG> {
        self.variant(Pbits::Polsd)
    }
    #[doc = "Polarity for Color mode"]
    #[inline(always)]
    pub fn polcm(self) -> &'a mut crate::W<REG> {
        self.variant(Pbits::Polcm)
    }
}
impl R {
    #[doc = "Bits 0:3 - Polarity bits"]
    #[inline(always)]
    pub fn pbits(&self) -> PbitsR {
        PbitsR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Polarity bits"]
    #[inline(always)]
    #[must_use]
    pub fn pbits(&mut self) -> PbitsW<PolaritySpec> {
        PbitsW::new(self, 0)
    }
}
#[doc = "Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PolaritySpec;
impl crate::RegisterSpec for PolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polarity::R`](R) reader structure"]
impl crate::Readable for PolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`polarity::W`](W) writer structure"]
impl crate::Writable for PolaritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLARITY to value 0"]
impl crate::Resettable for PolaritySpec {
    const RESET_VALUE: u32 = 0;
}
