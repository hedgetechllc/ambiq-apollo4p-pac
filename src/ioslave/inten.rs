#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `FSIZE` reader - FIFO Size interrupt."]
pub type FsizeR = crate::BitReader;
#[doc = "Field `FSIZE` writer - FIFO Size interrupt."]
pub type FsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOVFL` reader - FIFO Overflow interrupt."]
pub type FovflR = crate::BitReader;
#[doc = "Field `FOVFL` writer - FIFO Overflow interrupt."]
pub type FovflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUNDFL` reader - FIFO Underflow interrupt."]
pub type FundflR = crate::BitReader;
#[doc = "Field `FUNDFL` writer - FIFO Underflow interrupt."]
pub type FundflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRDERR` reader - FIFO Read Error interrupt."]
pub type FrderrR = crate::BitReader;
#[doc = "Field `FRDERR` writer - FIFO Read Error interrupt."]
pub type FrderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENAD` reader - I2C General Address interrupt."]
pub type GenadR = crate::BitReader;
#[doc = "Field `GENAD` writer - I2C General Address interrupt."]
pub type GenadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOINTW` reader - IO Write interrupt."]
pub type IointwR = crate::BitReader;
#[doc = "Field `IOINTW` writer - IO Write interrupt."]
pub type IointwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCMPRF` reader - Transfer complete interrupt, read from FIFO space."]
pub type XcmprfR = crate::BitReader;
#[doc = "Field `XCMPRF` writer - Transfer complete interrupt, read from FIFO space."]
pub type XcmprfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCMPRR` reader - Transfer complete interrupt, read from register space."]
pub type XcmprrR = crate::BitReader;
#[doc = "Field `XCMPRR` writer - Transfer complete interrupt, read from register space."]
pub type XcmprrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCMPWF` reader - Transfer complete interrupt, write to FIFO space."]
pub type XcmpwfR = crate::BitReader;
#[doc = "Field `XCMPWF` writer - Transfer complete interrupt, write to FIFO space."]
pub type XcmpwfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCMPWR` reader - Transfer complete interrupt, write to register space."]
pub type XcmpwrR = crate::BitReader;
#[doc = "Field `XCMPWR` writer - Transfer complete interrupt, write to register space."]
pub type XcmpwrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FIFO Size interrupt."]
    #[inline(always)]
    pub fn fsize(&self) -> FsizeR {
        FsizeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Overflow interrupt."]
    #[inline(always)]
    pub fn fovfl(&self) -> FovflR {
        FovflR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Underflow interrupt."]
    #[inline(always)]
    pub fn fundfl(&self) -> FundflR {
        FundflR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Read Error interrupt."]
    #[inline(always)]
    pub fn frderr(&self) -> FrderrR {
        FrderrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C General Address interrupt."]
    #[inline(always)]
    pub fn genad(&self) -> GenadR {
        GenadR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO Write interrupt."]
    #[inline(always)]
    pub fn iointw(&self) -> IointwR {
        IointwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer complete interrupt, read from FIFO space."]
    #[inline(always)]
    pub fn xcmprf(&self) -> XcmprfR {
        XcmprfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer complete interrupt, read from register space."]
    #[inline(always)]
    pub fn xcmprr(&self) -> XcmprrR {
        XcmprrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transfer complete interrupt, write to FIFO space."]
    #[inline(always)]
    pub fn xcmpwf(&self) -> XcmpwfR {
        XcmpwfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer complete interrupt, write to register space."]
    #[inline(always)]
    pub fn xcmpwr(&self) -> XcmpwrR {
        XcmpwrR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Size interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn fsize(&mut self) -> FsizeW<IntenSpec> {
        FsizeW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn fovfl(&mut self) -> FovflW<IntenSpec> {
        FovflW::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO Underflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn fundfl(&mut self) -> FundflW<IntenSpec> {
        FundflW::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO Read Error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn frderr(&mut self) -> FrderrW<IntenSpec> {
        FrderrW::new(self, 3)
    }
    #[doc = "Bit 4 - I2C General Address interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn genad(&mut self) -> GenadW<IntenSpec> {
        GenadW::new(self, 4)
    }
    #[doc = "Bit 5 - IO Write interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn iointw(&mut self) -> IointwW<IntenSpec> {
        IointwW::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer complete interrupt, read from FIFO space."]
    #[inline(always)]
    #[must_use]
    pub fn xcmprf(&mut self) -> XcmprfW<IntenSpec> {
        XcmprfW::new(self, 6)
    }
    #[doc = "Bit 7 - Transfer complete interrupt, read from register space."]
    #[inline(always)]
    #[must_use]
    pub fn xcmprr(&mut self) -> XcmprrW<IntenSpec> {
        XcmprrW::new(self, 7)
    }
    #[doc = "Bit 8 - Transfer complete interrupt, write to FIFO space."]
    #[inline(always)]
    #[must_use]
    pub fn xcmpwf(&mut self) -> XcmpwfW<IntenSpec> {
        XcmpwfW::new(self, 8)
    }
    #[doc = "Bit 9 - Transfer complete interrupt, write to register space."]
    #[inline(always)]
    #[must_use]
    pub fn xcmpwr(&mut self) -> XcmpwrW<IntenSpec> {
        XcmpwrW::new(self, 9)
    }
}
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
