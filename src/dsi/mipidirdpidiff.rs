#[doc = "Register `MIPIDIRDPIDIFF` reader"]
pub type R = crate::R<MipidirdpidiffSpec>;
#[doc = "Register `MIPIDIRDPIDIFF` writer"]
pub type W = crate::W<MipidirdpidiffSpec>;
#[doc = "This field provides the direction of MIPI bus;\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mipidir {
    #[doc = "0: DSI Host has the control over MIPI bus"]
    Control = 0,
    #[doc = "1: DSI Host is in Receive mode"]
    Receive = 1,
}
impl From<Mipidir> for bool {
    #[inline(always)]
    fn from(variant: Mipidir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIPIDIR` reader - This field provides the direction of MIPI bus;"]
pub type MipidirR = crate::BitReader<Mipidir>;
impl MipidirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mipidir {
        match self.bits {
            false => Mipidir::Control,
            true => Mipidir::Receive,
        }
    }
    #[doc = "DSI Host has the control over MIPI bus"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == Mipidir::Control
    }
    #[doc = "DSI Host is in Receive mode"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == Mipidir::Receive
    }
}
#[doc = "Field `MIPIDIR` writer - This field provides the direction of MIPI bus;"]
pub type MipidirW<'a, REG> = crate::BitWriter<'a, REG, Mipidir>;
impl<'a, REG> MipidirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DSI Host has the control over MIPI bus"]
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(Mipidir::Control)
    }
    #[doc = "DSI Host is in Receive mode"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut crate::W<REG> {
        self.variant(Mipidir::Receive)
    }
}
#[doc = "This field provides information to check DPI line time is greater or DSI line time is greater\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpihigh {
    #[doc = "0: one line time in DPI is less than to DSI line time"]
    Lessthan = 0,
    #[doc = "1: one line time in DPI is greater than or equal to DSI line time"]
    Greater = 1,
}
impl From<Dpihigh> for bool {
    #[inline(always)]
    fn from(variant: Dpihigh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPIHIGH` reader - This field provides information to check DPI line time is greater or DSI line time is greater"]
pub type DpihighR = crate::BitReader<Dpihigh>;
impl DpihighR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpihigh {
        match self.bits {
            false => Dpihigh::Lessthan,
            true => Dpihigh::Greater,
        }
    }
    #[doc = "one line time in DPI is less than to DSI line time"]
    #[inline(always)]
    pub fn is_lessthan(&self) -> bool {
        *self == Dpihigh::Lessthan
    }
    #[doc = "one line time in DPI is greater than or equal to DSI line time"]
    #[inline(always)]
    pub fn is_greater(&self) -> bool {
        *self == Dpihigh::Greater
    }
}
#[doc = "Field `DPIHIGH` writer - This field provides information to check DPI line time is greater or DSI line time is greater"]
pub type DpihighW<'a, REG> = crate::BitWriter<'a, REG, Dpihigh>;
impl<'a, REG> DpihighW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "one line time in DPI is less than to DSI line time"]
    #[inline(always)]
    pub fn lessthan(self) -> &'a mut crate::W<REG> {
        self.variant(Dpihigh::Lessthan)
    }
    #[doc = "one line time in DPI is greater than or equal to DSI line time"]
    #[inline(always)]
    pub fn greater(self) -> &'a mut crate::W<REG> {
        self.variant(Dpihigh::Greater)
    }
}
#[doc = "Field `DPIDIFF` reader - This field provides the difference in one line time between DPI and DSI"]
pub type DpidiffR = crate::FieldReader<u16>;
#[doc = "Field `DPIDIFF` writer - This field provides the difference in one line time between DPI and DSI"]
pub type DpidiffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - This field provides the direction of MIPI bus;"]
    #[inline(always)]
    pub fn mipidir(&self) -> MipidirR {
        MipidirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 15 - This field provides information to check DPI line time is greater or DSI line time is greater"]
    #[inline(always)]
    pub fn dpihigh(&self) -> DpihighR {
        DpihighR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - This field provides the difference in one line time between DPI and DSI"]
    #[inline(always)]
    pub fn dpidiff(&self) -> DpidiffR {
        DpidiffR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This field provides the direction of MIPI bus;"]
    #[inline(always)]
    #[must_use]
    pub fn mipidir(&mut self) -> MipidirW<MipidirdpidiffSpec> {
        MipidirW::new(self, 0)
    }
    #[doc = "Bit 15 - This field provides information to check DPI line time is greater or DSI line time is greater"]
    #[inline(always)]
    #[must_use]
    pub fn dpihigh(&mut self) -> DpihighW<MipidirdpidiffSpec> {
        DpihighW::new(self, 15)
    }
    #[doc = "Bits 16:31 - This field provides the difference in one line time between DPI and DSI"]
    #[inline(always)]
    #[must_use]
    pub fn dpidiff(&mut self) -> DpidiffW<MipidirdpidiffSpec> {
        DpidiffW::new(self, 16)
    }
}
#[doc = "Mipi direction DPI difference\n\nYou can [`read`](crate::Reg::read) this register and get [`mipidirdpidiff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipidirdpidiff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipidirdpidiffSpec;
impl crate::RegisterSpec for MipidirdpidiffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipidirdpidiff::R`](R) reader structure"]
impl crate::Readable for MipidirdpidiffSpec {}
#[doc = "`write(|w| ..)` method takes [`mipidirdpidiff::W`](W) writer structure"]
impl crate::Writable for MipidirdpidiffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIPIDIRDPIDIFF to value 0"]
impl crate::Resettable for MipidirdpidiffSpec {
    const RESET_VALUE: u32 = 0;
}
