#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `STATNOTBLANK` reader - Indicates that the controller is not in active vertical blanking"]
pub type StatnotblankR = crate::BitReader;
#[doc = "Field `STATNOTBLANK` writer - Indicates that the controller is not in active vertical blanking"]
pub type StatnotblankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATDE` reader - Indicates the DE signal status (0 or 1) at the current time of reading"]
pub type StatdeR = crate::BitReader;
#[doc = "Field `STATDE` writer - Indicates the DE signal status (0 or 1) at the current time of reading"]
pub type StatdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATHSYNC` reader - Indicates the HSYNC signal status (0 or 1) at the current time of reading"]
pub type StathsyncR = crate::BitReader;
#[doc = "Field `STATHSYNC` writer - Indicates the HSYNC signal status (0 or 1) at the current time of reading"]
pub type StathsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATVSYNC` reader - Indicates the VSYNC signal status and the tearing e?ect signal status (0 or 1) at the current time of reading"]
pub type StatvsyncR = crate::BitReader;
#[doc = "Field `STATVSYNC` writer - Indicates the VSYNC signal status and the tearing e?ect signal status (0 or 1) at the current time of reading"]
pub type StatvsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATCSYNC` reader - Indicates the CSYNC signal status (0 or 1) at the current time of reading"]
pub type StatcsyncR = crate::BitReader;
#[doc = "Field `STATCSYNC` writer - Indicates the CSYNC signal status (0 or 1) at the current time of reading"]
pub type StatcsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATLAST` reader - Indicates that the last row is currently displayed"]
pub type StatlastR = crate::BitReader;
#[doc = "Field `STATLAST` writer - Indicates that the last row is currently displayed"]
pub type StatlastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUF` reader - Indicates current underflow"]
pub type StatufR = crate::BitReader;
#[doc = "Field `STATUF` writer - Indicates current underflow"]
pub type StatufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATSTICKY` reader - Indicates sticky underflow. This bit clears when interrupt register is written"]
pub type StatstickyR = crate::BitReader;
#[doc = "Field `STATSTICKY` writer - Indicates sticky underflow. This bit clears when interrupt register is written"]
pub type StatstickyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATTEAR` reader - Indicates DBI Type-B tearing effect"]
pub type StattearR = crate::BitReader;
#[doc = "Field `STATTEAR` writer - Indicates DBI Type-B tearing effect"]
pub type StattearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATDBIRGB` reader - Indicates pending RGB data in DBI interface"]
pub type StatdbirgbR = crate::BitReader;
#[doc = "Field `STATDBIRGB` writer - Indicates pending RGB data in DBI interface"]
pub type StatdbirgbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATDBIPENDCOM` reader - Indicates pending commands in DBI interface"]
pub type StatdbipendcomR = crate::BitReader;
#[doc = "Field `STATDBIPENDCOM` writer - Indicates pending commands in DBI interface"]
pub type StatdbipendcomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATDBIPENDTRANS` reader - Indicates pending output transaction in DBI interface"]
pub type StatdbipendtransR = crate::BitReader;
#[doc = "Field `STATDBIPENDTRANS` writer - Indicates pending output transaction in DBI interface"]
pub type StatdbipendtransW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that the controller is not in active vertical blanking"]
    #[inline(always)]
    pub fn statnotblank(&self) -> StatnotblankR {
        StatnotblankR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the DE signal status (0 or 1) at the current time of reading"]
    #[inline(always)]
    pub fn statde(&self) -> StatdeR {
        StatdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the HSYNC signal status (0 or 1) at the current time of reading"]
    #[inline(always)]
    pub fn stathsync(&self) -> StathsyncR {
        StathsyncR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates the VSYNC signal status and the tearing e?ect signal status (0 or 1) at the current time of reading"]
    #[inline(always)]
    pub fn statvsync(&self) -> StatvsyncR {
        StatvsyncR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates the CSYNC signal status (0 or 1) at the current time of reading"]
    #[inline(always)]
    pub fn statcsync(&self) -> StatcsyncR {
        StatcsyncR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that the last row is currently displayed"]
    #[inline(always)]
    pub fn statlast(&self) -> StatlastR {
        StatlastR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates current underflow"]
    #[inline(always)]
    pub fn statuf(&self) -> StatufR {
        StatufR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates sticky underflow. This bit clears when interrupt register is written"]
    #[inline(always)]
    pub fn statsticky(&self) -> StatstickyR {
        StatstickyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates DBI Type-B tearing effect"]
    #[inline(always)]
    pub fn stattear(&self) -> StattearR {
        StattearR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates pending RGB data in DBI interface"]
    #[inline(always)]
    pub fn statdbirgb(&self) -> StatdbirgbR {
        StatdbirgbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates pending commands in DBI interface"]
    #[inline(always)]
    pub fn statdbipendcom(&self) -> StatdbipendcomR {
        StatdbipendcomR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates pending output transaction in DBI interface"]
    #[inline(always)]
    pub fn statdbipendtrans(&self) -> StatdbipendtransR {
        StatdbipendtransR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that the controller is not in active vertical blanking"]
    #[inline(always)]
    #[must_use]
    pub fn statnotblank(&mut self) -> StatnotblankW<StatusSpec> {
        StatnotblankW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates the DE signal status (0 or 1) at the current time of reading"]
    #[inline(always)]
    #[must_use]
    pub fn statde(&mut self) -> StatdeW<StatusSpec> {
        StatdeW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates the HSYNC signal status (0 or 1) at the current time of reading"]
    #[inline(always)]
    #[must_use]
    pub fn stathsync(&mut self) -> StathsyncW<StatusSpec> {
        StathsyncW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates the VSYNC signal status and the tearing e?ect signal status (0 or 1) at the current time of reading"]
    #[inline(always)]
    #[must_use]
    pub fn statvsync(&mut self) -> StatvsyncW<StatusSpec> {
        StatvsyncW::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates the CSYNC signal status (0 or 1) at the current time of reading"]
    #[inline(always)]
    #[must_use]
    pub fn statcsync(&mut self) -> StatcsyncW<StatusSpec> {
        StatcsyncW::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates that the last row is currently displayed"]
    #[inline(always)]
    #[must_use]
    pub fn statlast(&mut self) -> StatlastW<StatusSpec> {
        StatlastW::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates current underflow"]
    #[inline(always)]
    #[must_use]
    pub fn statuf(&mut self) -> StatufW<StatusSpec> {
        StatufW::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates sticky underflow. This bit clears when interrupt register is written"]
    #[inline(always)]
    #[must_use]
    pub fn statsticky(&mut self) -> StatstickyW<StatusSpec> {
        StatstickyW::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates DBI Type-B tearing effect"]
    #[inline(always)]
    #[must_use]
    pub fn stattear(&mut self) -> StattearW<StatusSpec> {
        StattearW::new(self, 8)
    }
    #[doc = "Bit 10 - Indicates pending RGB data in DBI interface"]
    #[inline(always)]
    #[must_use]
    pub fn statdbirgb(&mut self) -> StatdbirgbW<StatusSpec> {
        StatdbirgbW::new(self, 10)
    }
    #[doc = "Bit 11 - Indicates pending commands in DBI interface"]
    #[inline(always)]
    #[must_use]
    pub fn statdbipendcom(&mut self) -> StatdbipendcomW<StatusSpec> {
        StatdbipendcomW::new(self, 11)
    }
    #[doc = "Bit 12 - Indicates pending output transaction in DBI interface"]
    #[inline(always)]
    #[must_use]
    pub fn statdbipendtrans(&mut self) -> StatdbipendtransW<StatusSpec> {
        StatdbipendtransW::new(self, 12)
    }
}
#[doc = "DSI Status register (interrupt and pending activity)\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
