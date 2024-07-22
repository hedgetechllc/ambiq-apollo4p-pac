#[doc = "Register `HF2ADJ1` reader"]
pub type R = crate::R<Hf2adj1Spec>;
#[doc = "Register `HF2ADJ1` writer"]
pub type W = crate::W<Hf2adj1Spec>;
#[doc = "HF2ADJ output selection\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hf2adjtrimen {
    #[doc = "0: 0"]
    TrimEn0 = 0,
    #[doc = "1: HF2ADJTRIMOUT"]
    TrimEn1 = 1,
    #[doc = "2: HF2ADJTRIMOFFSET"]
    TrimEn2 = 2,
    #[doc = "3: HF2ADJTRIMOUT + HF2ADJTRIMOFFSET"]
    TrimEn3 = 3,
    #[doc = "4: HF2TUNE"]
    TrimEn4 = 4,
    #[doc = "5: HF2ADJTRIMOUT + HF2TUNE"]
    TrimEn5 = 5,
    #[doc = "6: HF2ADJTRIMOFFSET + HF2TUNE"]
    TrimEn6 = 6,
    #[doc = "7: HF2ADJTRIMOUT + HF2ADJTRIMOFFSET + HF2TUNE"]
    TrimEn7 = 7,
}
impl From<Hf2adjtrimen> for u8 {
    #[inline(always)]
    fn from(variant: Hf2adjtrimen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hf2adjtrimen {
    type Ux = u8;
}
impl crate::IsEnum for Hf2adjtrimen {}
#[doc = "Field `HF2ADJTRIMEN` reader - HF2ADJ output selection"]
pub type Hf2adjtrimenR = crate::FieldReader<Hf2adjtrimen>;
impl Hf2adjtrimenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hf2adjtrimen {
        match self.bits {
            0 => Hf2adjtrimen::TrimEn0,
            1 => Hf2adjtrimen::TrimEn1,
            2 => Hf2adjtrimen::TrimEn2,
            3 => Hf2adjtrimen::TrimEn3,
            4 => Hf2adjtrimen::TrimEn4,
            5 => Hf2adjtrimen::TrimEn5,
            6 => Hf2adjtrimen::TrimEn6,
            7 => Hf2adjtrimen::TrimEn7,
            _ => unreachable!(),
        }
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_trim_en0(&self) -> bool {
        *self == Hf2adjtrimen::TrimEn0
    }
    #[doc = "HF2ADJTRIMOUT"]
    #[inline(always)]
    pub fn is_trim_en1(&self) -> bool {
        *self == Hf2adjtrimen::TrimEn1
    }
    #[doc = "HF2ADJTRIMOFFSET"]
    #[inline(always)]
    pub fn is_trim_en2(&self) -> bool {
        *self == Hf2adjtrimen::TrimEn2
    }
    #[doc = "HF2ADJTRIMOUT + HF2ADJTRIMOFFSET"]
    #[inline(always)]
    pub fn is_trim_en3(&self) -> bool {
        *self == Hf2adjtrimen::TrimEn3
    }
    #[doc = "HF2TUNE"]
    #[inline(always)]
    pub fn is_trim_en4(&self) -> bool {
        *self == Hf2adjtrimen::TrimEn4
    }
    #[doc = "HF2ADJTRIMOUT + HF2TUNE"]
    #[inline(always)]
    pub fn is_trim_en5(&self) -> bool {
        *self == Hf2adjtrimen::TrimEn5
    }
    #[doc = "HF2ADJTRIMOFFSET + HF2TUNE"]
    #[inline(always)]
    pub fn is_trim_en6(&self) -> bool {
        *self == Hf2adjtrimen::TrimEn6
    }
    #[doc = "HF2ADJTRIMOUT + HF2ADJTRIMOFFSET + HF2TUNE"]
    #[inline(always)]
    pub fn is_trim_en7(&self) -> bool {
        *self == Hf2adjtrimen::TrimEn7
    }
}
#[doc = "Field `HF2ADJTRIMEN` writer - HF2ADJ output selection"]
pub type Hf2adjtrimenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hf2adjtrimen, crate::Safe>;
impl<'a, REG> Hf2adjtrimenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0"]
    #[inline(always)]
    pub fn trim_en0(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjtrimen::TrimEn0)
    }
    #[doc = "HF2ADJTRIMOUT"]
    #[inline(always)]
    pub fn trim_en1(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjtrimen::TrimEn1)
    }
    #[doc = "HF2ADJTRIMOFFSET"]
    #[inline(always)]
    pub fn trim_en2(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjtrimen::TrimEn2)
    }
    #[doc = "HF2ADJTRIMOUT + HF2ADJTRIMOFFSET"]
    #[inline(always)]
    pub fn trim_en3(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjtrimen::TrimEn3)
    }
    #[doc = "HF2TUNE"]
    #[inline(always)]
    pub fn trim_en4(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjtrimen::TrimEn4)
    }
    #[doc = "HF2ADJTRIMOUT + HF2TUNE"]
    #[inline(always)]
    pub fn trim_en5(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjtrimen::TrimEn5)
    }
    #[doc = "HF2ADJTRIMOFFSET + HF2TUNE"]
    #[inline(always)]
    pub fn trim_en6(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjtrimen::TrimEn6)
    }
    #[doc = "HF2ADJTRIMOUT + HF2ADJTRIMOFFSET + HF2TUNE"]
    #[inline(always)]
    pub fn trim_en7(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjtrimen::TrimEn7)
    }
}
#[doc = "Field `HF2ADJTRIMOFFSET` reader - HF2ADJ trimming offset. (signed number)"]
pub type Hf2adjtrimoffsetR = crate::FieldReader<u16>;
#[doc = "Field `HF2ADJTRIMOFFSET` writer - HF2ADJ trimming offset. (signed number)"]
pub type Hf2adjtrimoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:2 - HF2ADJ output selection"]
    #[inline(always)]
    pub fn hf2adjtrimen(&self) -> Hf2adjtrimenR {
        Hf2adjtrimenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - HF2ADJ trimming offset. (signed number)"]
    #[inline(always)]
    pub fn hf2adjtrimoffset(&self) -> Hf2adjtrimoffsetR {
        Hf2adjtrimoffsetR::new(((self.bits >> 3) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - HF2ADJ output selection"]
    #[inline(always)]
    #[must_use]
    pub fn hf2adjtrimen(&mut self) -> Hf2adjtrimenW<Hf2adj1Spec> {
        Hf2adjtrimenW::new(self, 0)
    }
    #[doc = "Bits 3:13 - HF2ADJ trimming offset. (signed number)"]
    #[inline(always)]
    #[must_use]
    pub fn hf2adjtrimoffset(&mut self) -> Hf2adjtrimoffsetW<Hf2adj1Spec> {
        Hf2adjtrimoffsetW::new(self, 3)
    }
}
#[doc = "This register controls hf2adj trimming enable and trimming offset.\n\nYou can [`read`](crate::Reg::read) this register and get [`hf2adj1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hf2adj1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hf2adj1Spec;
impl crate::RegisterSpec for Hf2adj1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hf2adj1::R`](R) reader structure"]
impl crate::Readable for Hf2adj1Spec {}
#[doc = "`write(|w| ..)` method takes [`hf2adj1::W`](W) writer structure"]
impl crate::Writable for Hf2adj1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HF2ADJ1 to value 0x07"]
impl crate::Resettable for Hf2adj1Spec {
    const RESET_VALUE: u32 = 0x07;
}
