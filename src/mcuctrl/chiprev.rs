#[doc = "Register `CHIPREV` reader"]
pub type R = crate::R<ChiprevSpec>;
#[doc = "Register `CHIPREV` writer"]
pub type W = crate::W<ChiprevSpec>;
#[doc = "Minor Revision ID.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revmin {
    #[doc = "3: Apollo4 minor rev 2."]
    Rev2 = 3,
    #[doc = "2: Apollo4 minor rev 1."]
    Rev1 = 2,
    #[doc = "1: Apollo4 minor rev 0. Minor revision value, succeeding minor revisions will increment from this value."]
    Rev0 = 1,
}
impl From<Revmin> for u8 {
    #[inline(always)]
    fn from(variant: Revmin) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revmin {
    type Ux = u8;
}
impl crate::IsEnum for Revmin {}
#[doc = "Field `REVMIN` reader - Minor Revision ID."]
pub type RevminR = crate::FieldReader<Revmin>;
impl RevminR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revmin> {
        match self.bits {
            3 => Some(Revmin::Rev2),
            2 => Some(Revmin::Rev1),
            1 => Some(Revmin::Rev0),
            _ => None,
        }
    }
    #[doc = "Apollo4 minor rev 2."]
    #[inline(always)]
    pub fn is_rev2(&self) -> bool {
        *self == Revmin::Rev2
    }
    #[doc = "Apollo4 minor rev 1."]
    #[inline(always)]
    pub fn is_rev1(&self) -> bool {
        *self == Revmin::Rev1
    }
    #[doc = "Apollo4 minor rev 0. Minor revision value, succeeding minor revisions will increment from this value."]
    #[inline(always)]
    pub fn is_rev0(&self) -> bool {
        *self == Revmin::Rev0
    }
}
#[doc = "Field `REVMIN` writer - Minor Revision ID."]
pub type RevminW<'a, REG> = crate::FieldWriter<'a, REG, 4, Revmin>;
impl<'a, REG> RevminW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Apollo4 minor rev 2."]
    #[inline(always)]
    pub fn rev2(self) -> &'a mut crate::W<REG> {
        self.variant(Revmin::Rev2)
    }
    #[doc = "Apollo4 minor rev 1."]
    #[inline(always)]
    pub fn rev1(self) -> &'a mut crate::W<REG> {
        self.variant(Revmin::Rev1)
    }
    #[doc = "Apollo4 minor rev 0. Minor revision value, succeeding minor revisions will increment from this value."]
    #[inline(always)]
    pub fn rev0(self) -> &'a mut crate::W<REG> {
        self.variant(Revmin::Rev0)
    }
}
#[doc = "Major Revision ID.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revmaj {
    #[doc = "3: Apollo4 revision C"]
    C = 3,
    #[doc = "2: Apollo4 revision B"]
    B = 2,
    #[doc = "1: Apollo4 revision A"]
    A = 1,
}
impl From<Revmaj> for u8 {
    #[inline(always)]
    fn from(variant: Revmaj) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revmaj {
    type Ux = u8;
}
impl crate::IsEnum for Revmaj {}
#[doc = "Field `REVMAJ` reader - Major Revision ID."]
pub type RevmajR = crate::FieldReader<Revmaj>;
impl RevmajR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revmaj> {
        match self.bits {
            3 => Some(Revmaj::C),
            2 => Some(Revmaj::B),
            1 => Some(Revmaj::A),
            _ => None,
        }
    }
    #[doc = "Apollo4 revision C"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == Revmaj::C
    }
    #[doc = "Apollo4 revision B"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Revmaj::B
    }
    #[doc = "Apollo4 revision A"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Revmaj::A
    }
}
#[doc = "Field `REVMAJ` writer - Major Revision ID."]
pub type RevmajW<'a, REG> = crate::FieldWriter<'a, REG, 4, Revmaj>;
impl<'a, REG> RevmajW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Apollo4 revision C"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(Revmaj::C)
    }
    #[doc = "Apollo4 revision B"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Revmaj::B)
    }
    #[doc = "Apollo4 revision A"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Revmaj::A)
    }
}
#[doc = "Field `SIPART` reader - Silicon Part ID"]
pub type SipartR = crate::FieldReader<u16>;
#[doc = "Field `SIPART` writer - Silicon Part ID"]
pub type SipartW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - Minor Revision ID."]
    #[inline(always)]
    pub fn revmin(&self) -> RevminR {
        RevminR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major Revision ID."]
    #[inline(always)]
    pub fn revmaj(&self) -> RevmajR {
        RevmajR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:19 - Silicon Part ID"]
    #[inline(always)]
    pub fn sipart(&self) -> SipartR {
        SipartR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Minor Revision ID."]
    #[inline(always)]
    #[must_use]
    pub fn revmin(&mut self) -> RevminW<ChiprevSpec> {
        RevminW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Major Revision ID."]
    #[inline(always)]
    #[must_use]
    pub fn revmaj(&mut self) -> RevmajW<ChiprevSpec> {
        RevmajW::new(self, 4)
    }
    #[doc = "Bits 8:19 - Silicon Part ID"]
    #[inline(always)]
    #[must_use]
    pub fn sipart(&mut self) -> SipartW<ChiprevSpec> {
        SipartW::new(self, 8)
    }
}
#[doc = "Chip Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`chiprev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chiprev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChiprevSpec;
impl crate::RegisterSpec for ChiprevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chiprev::R`](R) reader structure"]
impl crate::Readable for ChiprevSpec {}
#[doc = "`write(|w| ..)` method takes [`chiprev::W`](W) writer structure"]
impl crate::Writable for ChiprevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIPREV to value 0x11"]
impl crate::Resettable for ChiprevSpec {
    const RESET_VALUE: u32 = 0x11;
}
