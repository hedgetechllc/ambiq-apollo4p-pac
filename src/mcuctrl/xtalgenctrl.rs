#[doc = "Register `XTALGENCTRL` reader"]
pub type R = crate::R<XtalgenctrlSpec>;
#[doc = "Register `XTALGENCTRL` writer"]
pub type W = crate::W<XtalgenctrlSpec>;
#[doc = "Auto-calibration delay control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acwarmup {
    #[doc = "0: Warmup period of 1-2 seconds"]
    Sec1 = 0,
    #[doc = "1: Warmup period of 2-4 seconds"]
    Sec2 = 1,
    #[doc = "2: Warmup period of 4-8 seconds"]
    Sec4 = 2,
    #[doc = "3: Warmup period of 8-16 seconds"]
    Sec8 = 3,
}
impl From<Acwarmup> for u8 {
    #[inline(always)]
    fn from(variant: Acwarmup) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acwarmup {
    type Ux = u8;
}
impl crate::IsEnum for Acwarmup {}
#[doc = "Field `ACWARMUP` reader - Auto-calibration delay control"]
pub type AcwarmupR = crate::FieldReader<Acwarmup>;
impl AcwarmupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acwarmup {
        match self.bits {
            0 => Acwarmup::Sec1,
            1 => Acwarmup::Sec2,
            2 => Acwarmup::Sec4,
            3 => Acwarmup::Sec8,
            _ => unreachable!(),
        }
    }
    #[doc = "Warmup period of 1-2 seconds"]
    #[inline(always)]
    pub fn is_sec1(&self) -> bool {
        *self == Acwarmup::Sec1
    }
    #[doc = "Warmup period of 2-4 seconds"]
    #[inline(always)]
    pub fn is_sec2(&self) -> bool {
        *self == Acwarmup::Sec2
    }
    #[doc = "Warmup period of 4-8 seconds"]
    #[inline(always)]
    pub fn is_sec4(&self) -> bool {
        *self == Acwarmup::Sec4
    }
    #[doc = "Warmup period of 8-16 seconds"]
    #[inline(always)]
    pub fn is_sec8(&self) -> bool {
        *self == Acwarmup::Sec8
    }
}
#[doc = "Field `ACWARMUP` writer - Auto-calibration delay control"]
pub type AcwarmupW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acwarmup, crate::Safe>;
impl<'a, REG> AcwarmupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Warmup period of 1-2 seconds"]
    #[inline(always)]
    pub fn sec1(self) -> &'a mut crate::W<REG> {
        self.variant(Acwarmup::Sec1)
    }
    #[doc = "Warmup period of 2-4 seconds"]
    #[inline(always)]
    pub fn sec2(self) -> &'a mut crate::W<REG> {
        self.variant(Acwarmup::Sec2)
    }
    #[doc = "Warmup period of 4-8 seconds"]
    #[inline(always)]
    pub fn sec4(self) -> &'a mut crate::W<REG> {
        self.variant(Acwarmup::Sec4)
    }
    #[doc = "Warmup period of 8-16 seconds"]
    #[inline(always)]
    pub fn sec8(self) -> &'a mut crate::W<REG> {
        self.variant(Acwarmup::Sec8)
    }
}
#[doc = "Field `XTALBIASTRIM` reader - XTAL BIAS trim"]
pub type XtalbiastrimR = crate::FieldReader;
#[doc = "Field `XTALBIASTRIM` writer - XTAL BIAS trim"]
pub type XtalbiastrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `XTALKSBIASTRIM` reader - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
pub type XtalksbiastrimR = crate::FieldReader;
#[doc = "Field `XTALKSBIASTRIM` writer - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
pub type XtalksbiastrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Auto-calibration delay control"]
    #[inline(always)]
    pub fn acwarmup(&self) -> AcwarmupR {
        AcwarmupR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - XTAL BIAS trim"]
    #[inline(always)]
    pub fn xtalbiastrim(&self) -> XtalbiastrimR {
        XtalbiastrimR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
    #[inline(always)]
    pub fn xtalksbiastrim(&self) -> XtalksbiastrimR {
        XtalksbiastrimR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Auto-calibration delay control"]
    #[inline(always)]
    #[must_use]
    pub fn acwarmup(&mut self) -> AcwarmupW<XtalgenctrlSpec> {
        AcwarmupW::new(self, 0)
    }
    #[doc = "Bits 2:7 - XTAL BIAS trim"]
    #[inline(always)]
    #[must_use]
    pub fn xtalbiastrim(&mut self) -> XtalbiastrimW<XtalgenctrlSpec> {
        XtalbiastrimW::new(self, 2)
    }
    #[doc = "Bits 8:13 - XTAL IBIAS Kick start trim. This trim value is used during the startup process to enable a faster lock."]
    #[inline(always)]
    #[must_use]
    pub fn xtalksbiastrim(&mut self) -> XtalksbiastrimW<XtalgenctrlSpec> {
        XtalksbiastrimW::new(self, 8)
    }
}
#[doc = "XTAL Oscillator General Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalgenctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalgenctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalgenctrlSpec;
impl crate::RegisterSpec for XtalgenctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalgenctrl::R`](R) reader structure"]
impl crate::Readable for XtalgenctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalgenctrl::W`](W) writer structure"]
impl crate::Writable for XtalgenctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTALGENCTRL to value 0x0100"]
impl crate::Resettable for XtalgenctrlSpec {
    const RESET_VALUE: u32 = 0x0100;
}
