#[doc = "Register `DBICMD` reader"]
pub type R = crate::R<DbicmdSpec>;
#[doc = "Register `DBICMD` writer"]
pub type W = crate::W<DbicmdSpec>;
#[doc = "Field `DATA2DBI` reader - Data to send to the DBI interface"]
pub type Data2dbiR = crate::FieldReader<u16>;
#[doc = "Field `DATA2DBI` writer - Data to send to the DBI interface"]
pub type Data2dbiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RSVD0` reader - This field is reserved."]
pub type Rsvd0R = crate::FieldReader<u16>;
#[doc = "Field `RSVD0` writer - This field is reserved."]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `LOCALSTORE` reader - When set to 1, bits \\[15:0\\]
are locally stored as base address of the horizontal line; it is used along with the DBI_CFG\\[17:16\\]
register bits for the SPI interface"]
pub type LocalstoreR = crate::BitReader;
#[doc = "Field `LOCALSTORE` writer - When set to 1, bits \\[15:0\\]
are locally stored as base address of the horizontal line; it is used along with the DBI_CFG\\[17:16\\]
register bits for the SPI interface"]
pub type LocalstoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READDBI` reader - Read from DBI interface"]
pub type ReaddbiR = crate::BitReader;
#[doc = "Field `READDBI` writer - Read from DBI interface"]
pub type ReaddbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD1` reader - This field is reserved."]
pub type Rsvd1R = crate::BitReader;
#[doc = "Field `RSVD1` writer - This field is reserved."]
pub type Rsvd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRECTDATA` reader - Send direct data of type 'command' to the DBI interface"]
pub type DirectdataR = crate::BitReader;
#[doc = "Field `DIRECTDATA` writer - Send direct data of type 'command' to the DBI interface"]
pub type DirectdataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD2` reader - This field is reserved."]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD2` writer - This field is reserved."]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Data to send to the DBI interface"]
    #[inline(always)]
    pub fn data2dbi(&self) -> Data2dbiR {
        Data2dbiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - When set to 1, bits \\[15:0\\]
are locally stored as base address of the horizontal line; it is used along with the DBI_CFG\\[17:16\\]
register bits for the SPI interface"]
    #[inline(always)]
    pub fn localstore(&self) -> LocalstoreR {
        LocalstoreR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Read from DBI interface"]
    #[inline(always)]
    pub fn readdbi(&self) -> ReaddbiR {
        ReaddbiR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Send direct data of type 'command' to the DBI interface"]
    #[inline(always)]
    pub fn directdata(&self) -> DirectdataR {
        DirectdataR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data to send to the DBI interface"]
    #[inline(always)]
    #[must_use]
    pub fn data2dbi(&mut self) -> Data2dbiW<DbicmdSpec> {
        Data2dbiW::new(self, 0)
    }
    #[doc = "Bits 16:26 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> Rsvd0W<DbicmdSpec> {
        Rsvd0W::new(self, 16)
    }
    #[doc = "Bit 27 - When set to 1, bits \\[15:0\\]
are locally stored as base address of the horizontal line; it is used along with the DBI_CFG\\[17:16\\]
register bits for the SPI interface"]
    #[inline(always)]
    #[must_use]
    pub fn localstore(&mut self) -> LocalstoreW<DbicmdSpec> {
        LocalstoreW::new(self, 27)
    }
    #[doc = "Bit 28 - Read from DBI interface"]
    #[inline(always)]
    #[must_use]
    pub fn readdbi(&mut self) -> ReaddbiW<DbicmdSpec> {
        ReaddbiW::new(self, 28)
    }
    #[doc = "Bit 29 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> Rsvd1W<DbicmdSpec> {
        Rsvd1W::new(self, 29)
    }
    #[doc = "Bit 30 - Send direct data of type 'command' to the DBI interface"]
    #[inline(always)]
    #[must_use]
    pub fn directdata(&mut self) -> DirectdataW<DbicmdSpec> {
        DirectdataW::new(self, 30)
    }
    #[doc = "Bit 31 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd2(&mut self) -> Rsvd2W<DbicmdSpec> {
        Rsvd2W::new(self, 31)
    }
}
#[doc = "Register to read/write commands from/to DBI Type-B interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbicmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbicmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbicmdSpec;
impl crate::RegisterSpec for DbicmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbicmd::R`](R) reader structure"]
impl crate::Readable for DbicmdSpec {}
#[doc = "`write(|w| ..)` method takes [`dbicmd::W`](W) writer structure"]
impl crate::Writable for DbicmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBICMD to value 0"]
impl crate::Resettable for DbicmdSpec {
    const RESET_VALUE: u32 = 0;
}
