#[doc = "Register `HF2ADJ2` reader"]
pub type R = crate::R<Hf2adj2Spec>;
#[doc = "Register `HF2ADJ2` writer"]
pub type W = crate::W<Hf2adj2Spec>;
#[doc = "XTAL32MHz divider ratio for HF2ADJ.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hf2adjxtaldivratio {
    #[doc = "0: XTAL32MHz"]
    Xtal32m = 0,
    #[doc = "1: XTAL32MHz / 2"]
    Xtal32mDiv2 = 1,
    #[doc = "2: XTAL32MHz / 4"]
    Xtal32mDiv4 = 2,
    #[doc = "3: XTAL32MHz / 8"]
    Xtal32mDiv8 = 3,
}
impl From<Hf2adjxtaldivratio> for u8 {
    #[inline(always)]
    fn from(variant: Hf2adjxtaldivratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hf2adjxtaldivratio {
    type Ux = u8;
}
impl crate::IsEnum for Hf2adjxtaldivratio {}
#[doc = "Field `HF2ADJXTALDIVRATIO` reader - XTAL32MHz divider ratio for HF2ADJ."]
pub type Hf2adjxtaldivratioR = crate::FieldReader<Hf2adjxtaldivratio>;
impl Hf2adjxtaldivratioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hf2adjxtaldivratio {
        match self.bits {
            0 => Hf2adjxtaldivratio::Xtal32m,
            1 => Hf2adjxtaldivratio::Xtal32mDiv2,
            2 => Hf2adjxtaldivratio::Xtal32mDiv4,
            3 => Hf2adjxtaldivratio::Xtal32mDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "XTAL32MHz"]
    #[inline(always)]
    pub fn is_xtal32m(&self) -> bool {
        *self == Hf2adjxtaldivratio::Xtal32m
    }
    #[doc = "XTAL32MHz / 2"]
    #[inline(always)]
    pub fn is_xtal32m_div2(&self) -> bool {
        *self == Hf2adjxtaldivratio::Xtal32mDiv2
    }
    #[doc = "XTAL32MHz / 4"]
    #[inline(always)]
    pub fn is_xtal32m_div4(&self) -> bool {
        *self == Hf2adjxtaldivratio::Xtal32mDiv4
    }
    #[doc = "XTAL32MHz / 8"]
    #[inline(always)]
    pub fn is_xtal32m_div8(&self) -> bool {
        *self == Hf2adjxtaldivratio::Xtal32mDiv8
    }
}
#[doc = "Field `HF2ADJXTALDIVRATIO` writer - XTAL32MHz divider ratio for HF2ADJ."]
pub type Hf2adjxtaldivratioW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Hf2adjxtaldivratio, crate::Safe>;
impl<'a, REG> Hf2adjxtaldivratioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XTAL32MHz"]
    #[inline(always)]
    pub fn xtal32m(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjxtaldivratio::Xtal32m)
    }
    #[doc = "XTAL32MHz / 2"]
    #[inline(always)]
    pub fn xtal32m_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjxtaldivratio::Xtal32mDiv2)
    }
    #[doc = "XTAL32MHz / 4"]
    #[inline(always)]
    pub fn xtal32m_div4(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjxtaldivratio::Xtal32mDiv4)
    }
    #[doc = "XTAL32MHz / 8"]
    #[inline(always)]
    pub fn xtal32m_div8(self) -> &'a mut crate::W<REG> {
        self.variant(Hf2adjxtaldivratio::Xtal32mDiv8)
    }
}
#[doc = "Field `HF2ADJRATIO` reader - HF2ADJ ratio setting."]
pub type Hf2adjratioR = crate::FieldReader<u32>;
#[doc = "Field `HF2ADJRATIO` writer - HF2ADJ ratio setting."]
pub type Hf2adjratioW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:1 - XTAL32MHz divider ratio for HF2ADJ."]
    #[inline(always)]
    pub fn hf2adjxtaldivratio(&self) -> Hf2adjxtaldivratioR {
        Hf2adjxtaldivratioR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:30 - HF2ADJ ratio setting."]
    #[inline(always)]
    pub fn hf2adjratio(&self) -> Hf2adjratioR {
        Hf2adjratioR::new((self.bits >> 2) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - XTAL32MHz divider ratio for HF2ADJ."]
    #[inline(always)]
    #[must_use]
    pub fn hf2adjxtaldivratio(&mut self) -> Hf2adjxtaldivratioW<Hf2adj2Spec> {
        Hf2adjxtaldivratioW::new(self, 0)
    }
    #[doc = "Bits 2:30 - HF2ADJ ratio setting."]
    #[inline(always)]
    #[must_use]
    pub fn hf2adjratio(&mut self) -> Hf2adjratioW<Hf2adj2Spec> {
        Hf2adjratioW::new(self, 2)
    }
}
#[doc = "This register controls xtal32m divider ratio and HF2ADJ ration setting.\n\nYou can [`read`](crate::Reg::read) this register and get [`hf2adj2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hf2adj2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hf2adj2Spec;
impl crate::RegisterSpec for Hf2adj2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hf2adj2::R`](R) reader structure"]
impl crate::Readable for Hf2adj2Spec {}
#[doc = "`write(|w| ..)` method takes [`hf2adj2::W`](W) writer structure"]
impl crate::Writable for Hf2adj2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HF2ADJ2 to value 0x0018_9376"]
impl crate::Resettable for Hf2adj2Spec {
    const RESET_VALUE: u32 = 0x0018_9376;
}
