#[doc = "Register `DEV0SCRAMBLING` reader"]
pub type R = crate::R<Dev0scramblingSpec>;
#[doc = "Register `DEV0SCRAMBLING` writer"]
pub type W = crate::W<Dev0scramblingSpec>;
#[doc = "Field `SCRSTART0` reader - Scrambling region start address \\[25:16\\]
(64K block granularity). The START block is the FIRST block included in the scrambled address range."]
pub type Scrstart0R = crate::FieldReader<u16>;
#[doc = "Field `SCRSTART0` writer - Scrambling region start address \\[25:16\\]
(64K block granularity). The START block is the FIRST block included in the scrambled address range."]
pub type Scrstart0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SCREND0` reader - Scrambling region end address \\[25:16\\]
(64K block granularity). The END block is the LAST block included in the scrambled address range."]
pub type Scrend0R = crate::FieldReader<u16>;
#[doc = "Field `SCREND0` writer - Scrambling region end address \\[25:16\\]
(64K block granularity). The END block is the LAST block included in the scrambled address range."]
pub type Scrend0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SCRENABLE0` reader - Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
pub type Screnable0R = crate::BitReader;
#[doc = "Field `SCRENABLE0` writer - Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
pub type Screnable0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Scrambling region start address \\[25:16\\]
(64K block granularity). The START block is the FIRST block included in the scrambled address range."]
    #[inline(always)]
    pub fn scrstart0(&self) -> Scrstart0R {
        Scrstart0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Scrambling region end address \\[25:16\\]
(64K block granularity). The END block is the LAST block included in the scrambled address range."]
    #[inline(always)]
    pub fn scrend0(&self) -> Scrend0R {
        Scrend0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
    #[inline(always)]
    pub fn screnable0(&self) -> Screnable0R {
        Screnable0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Scrambling region start address \\[25:16\\]
(64K block granularity). The START block is the FIRST block included in the scrambled address range."]
    #[inline(always)]
    #[must_use]
    pub fn scrstart0(&mut self) -> Scrstart0W<Dev0scramblingSpec> {
        Scrstart0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Scrambling region end address \\[25:16\\]
(64K block granularity). The END block is the LAST block included in the scrambled address range."]
    #[inline(always)]
    #[must_use]
    pub fn scrend0(&mut self) -> Scrend0W<Dev0scramblingSpec> {
        Scrend0W::new(self, 16)
    }
    #[doc = "Bit 31 - Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
    #[inline(always)]
    #[must_use]
    pub fn screnable0(&mut self) -> Screnable0W<Dev0scramblingSpec> {
        Screnable0W::new(self, 31)
    }
}
#[doc = "Enables data scrambling for the specified range external flash addresses. Scrambling does not impact flash access performance.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0scrambling::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0scrambling::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev0scramblingSpec;
impl crate::RegisterSpec for Dev0scramblingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev0scrambling::R`](R) reader structure"]
impl crate::Readable for Dev0scramblingSpec {}
#[doc = "`write(|w| ..)` method takes [`dev0scrambling::W`](W) writer structure"]
impl crate::Writable for Dev0scramblingSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV0SCRAMBLING to value 0"]
impl crate::Resettable for Dev0scramblingSpec {
    const RESET_VALUE: u32 = 0;
}
