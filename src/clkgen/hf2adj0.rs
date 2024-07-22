#[doc = "Register `HF2ADJ0` reader"]
pub type R = crate::R<Hf2adj0Spec>;
#[doc = "Register `HF2ADJ0` writer"]
pub type W = crate::W<Hf2adj0Spec>;
#[doc = "HF2ADJ control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hf2adjen {
    #[doc = "0: HF2ADJ disable"]
    Dis = 0,
    #[doc = "1: HF2ADJ enable"]
    En = 1,
}
impl From<Hf2adjen> for bool {
    #[inline(always)]
    fn from(variant: Hf2adjen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HF2ADJEN` reader - HF2ADJ control"]
pub type Hf2adjenR = crate::BitReader<Hf2adjen>;
impl Hf2adjenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hf2adjen {
        match self.bits {
            false => Hf2adjen::Dis,
            true => Hf2adjen::En,
        }
    }
    #[doc = "HF2ADJ disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hf2adjen::Dis
    }
    #[doc = "HF2ADJ enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Hf2adjen::En
    }
}
#[doc = "Field `HF2ADJEN` writer - HF2ADJ control"]
pub type Hf2adjenW<'a, REG> = crate::BitWriter<'a, REG, Hf2adjen>;
impl<'a, REG> Hf2adjenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HF2ADJ disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjen::Dis)
    }
    #[doc = "HF2ADJ enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjen::En)
    }
}
#[doc = "Fast_start_delay control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hf2adjfaststren {
    #[doc = "0: Fast_start_delay disable"]
    Dis = 0,
    #[doc = "1: Fast_start_delay enable"]
    En = 1,
}
impl From<Hf2adjfaststren> for bool {
    #[inline(always)]
    fn from(variant: Hf2adjfaststren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HF2ADJFASTSTREN` reader - Fast_start_delay control"]
pub type Hf2adjfaststrenR = crate::BitReader<Hf2adjfaststren>;
impl Hf2adjfaststrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hf2adjfaststren {
        match self.bits {
            false => Hf2adjfaststren::Dis,
            true => Hf2adjfaststren::En,
        }
    }
    #[doc = "Fast_start_delay disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hf2adjfaststren::Dis
    }
    #[doc = "Fast_start_delay enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Hf2adjfaststren::En
    }
}
#[doc = "Field `HF2ADJFASTSTREN` writer - Fast_start_delay control"]
pub type Hf2adjfaststrenW<'a, REG> = crate::BitWriter<'a, REG, Hf2adjfaststren>;
impl<'a, REG> Hf2adjfaststrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast_start_delay disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjfaststren::Dis)
    }
    #[doc = "Fast_start_delay enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjfaststren::En)
    }
}
#[doc = "Field `HF2ADJFASTSTRDLY` reader - Fast_start_delay value setting"]
pub type Hf2adjfaststrdlyR = crate::FieldReader<u16>;
#[doc = "Field `HF2ADJFASTSTRDLY` writer - Fast_start_delay value setting"]
pub type Hf2adjfaststrdlyW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `HF2ADJCNTINOFFSET` reader - Counter input offset"]
pub type Hf2adjcntinoffsetR = crate::FieldReader<u16>;
#[doc = "Field `HF2ADJCNTINOFFSET` writer - Counter input offset"]
pub type Hf2adjcntinoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `HF2ADJXTHSMUXSEL` reader - 0=XTHS 1=EXTREF select"]
pub type Hf2adjxthsmuxselR = crate::BitReader;
#[doc = "Field `HF2ADJXTHSMUXSEL` writer - 0=XTHS 1=EXTREF select"]
pub type Hf2adjxthsmuxselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HF2ADJ control"]
    #[inline(always)]
    pub fn hf2adjen(&self) -> Hf2adjenR {
        Hf2adjenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast_start_delay control"]
    #[inline(always)]
    pub fn hf2adjfaststren(&self) -> Hf2adjfaststrenR {
        Hf2adjfaststrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:14 - Fast_start_delay value setting"]
    #[inline(always)]
    pub fn hf2adjfaststrdly(&self) -> Hf2adjfaststrdlyR {
        Hf2adjfaststrdlyR::new(((self.bits >> 2) & 0x1fff) as u16)
    }
    #[doc = "Bits 15:28 - Counter input offset"]
    #[inline(always)]
    pub fn hf2adjcntinoffset(&self) -> Hf2adjcntinoffsetR {
        Hf2adjcntinoffsetR::new(((self.bits >> 15) & 0x3fff) as u16)
    }
    #[doc = "Bit 29 - 0=XTHS 1=EXTREF select"]
    #[inline(always)]
    pub fn hf2adjxthsmuxsel(&self) -> Hf2adjxthsmuxselR {
        Hf2adjxthsmuxselR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HF2ADJ control"]
    #[inline(always)]
    #[must_use]
    pub fn hf2adjen(&mut self) -> Hf2adjenW<Hf2adj0Spec> {
        Hf2adjenW::new(self, 0)
    }
    #[doc = "Bit 1 - Fast_start_delay control"]
    #[inline(always)]
    #[must_use]
    pub fn hf2adjfaststren(&mut self) -> Hf2adjfaststrenW<Hf2adj0Spec> {
        Hf2adjfaststrenW::new(self, 1)
    }
    #[doc = "Bits 2:14 - Fast_start_delay value setting"]
    #[inline(always)]
    #[must_use]
    pub fn hf2adjfaststrdly(&mut self) -> Hf2adjfaststrdlyW<Hf2adj0Spec> {
        Hf2adjfaststrdlyW::new(self, 2)
    }
    #[doc = "Bits 15:28 - Counter input offset"]
    #[inline(always)]
    #[must_use]
    pub fn hf2adjcntinoffset(&mut self) -> Hf2adjcntinoffsetW<Hf2adj0Spec> {
        Hf2adjcntinoffsetW::new(self, 15)
    }
    #[doc = "Bit 29 - 0=XTHS 1=EXTREF select"]
    #[inline(always)]
    #[must_use]
    pub fn hf2adjxthsmuxsel(&mut self) -> Hf2adjxthsmuxselW<Hf2adj0Spec> {
        Hf2adjxthsmuxselW::new(self, 29)
    }
}
#[doc = "This register controls hf2adj enable, fast_start enable, fast_start_delay setting and counter input offset.\n\nYou can [`read`](crate::Reg::read) this register and get [`hf2adj0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hf2adj0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hf2adj0Spec;
impl crate::RegisterSpec for Hf2adj0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hf2adj0::R`](R) reader structure"]
impl crate::Readable for Hf2adj0Spec {}
#[doc = "`write(|w| ..)` method takes [`hf2adj0::W`](W) writer structure"]
impl crate::Writable for Hf2adj0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HF2ADJ0 to value 0x3ffe"]
impl crate::Resettable for Hf2adj0Spec {
    const RESET_VALUE: u32 = 0x3ffe;
}
