#[doc = "Register `ADMA` reader"]
pub type R = crate::R<AdmaSpec>;
#[doc = "Register `ADMA` writer"]
pub type W = crate::W<AdmaSpec>;
#[doc = "This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates 10 because ADMA never stops in this state. D01 - D00 : ADMA Error State when error occurred Contents of SYS_SDR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Admaerrorstate {
    #[doc = "0: ST_STOP (Stop DMA) Points to next of the error descriptor"]
    Stdma = 0,
    #[doc = "1: ST_FDS (Fetch Descriptor) Points to the error descriptor"]
    Fetchdesc = 1,
    #[doc = "2: Never set this state (Not used)"]
    Invalid = 2,
    #[doc = "3: ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    Xferdata = 3,
}
impl From<Admaerrorstate> for u8 {
    #[inline(always)]
    fn from(variant: Admaerrorstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Admaerrorstate {
    type Ux = u8;
}
impl crate::IsEnum for Admaerrorstate {}
#[doc = "Field `ADMAERRORSTATE` reader - This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates 10 because ADMA never stops in this state. D01 - D00 : ADMA Error State when error occurred Contents of SYS_SDR register"]
pub type AdmaerrorstateR = crate::FieldReader<Admaerrorstate>;
impl AdmaerrorstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admaerrorstate {
        match self.bits {
            0 => Admaerrorstate::Stdma,
            1 => Admaerrorstate::Fetchdesc,
            2 => Admaerrorstate::Invalid,
            3 => Admaerrorstate::Xferdata,
            _ => unreachable!(),
        }
    }
    #[doc = "ST_STOP (Stop DMA) Points to next of the error descriptor"]
    #[inline(always)]
    pub fn is_stdma(&self) -> bool {
        *self == Admaerrorstate::Stdma
    }
    #[doc = "ST_FDS (Fetch Descriptor) Points to the error descriptor"]
    #[inline(always)]
    pub fn is_fetchdesc(&self) -> bool {
        *self == Admaerrorstate::Fetchdesc
    }
    #[doc = "Never set this state (Not used)"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == Admaerrorstate::Invalid
    }
    #[doc = "ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    #[inline(always)]
    pub fn is_xferdata(&self) -> bool {
        *self == Admaerrorstate::Xferdata
    }
}
#[doc = "Field `ADMAERRORSTATE` writer - This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates 10 because ADMA never stops in this state. D01 - D00 : ADMA Error State when error occurred Contents of SYS_SDR register"]
pub type AdmaerrorstateW<'a, REG> = crate::FieldWriter<'a, REG, 2, Admaerrorstate, crate::Safe>;
impl<'a, REG> AdmaerrorstateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ST_STOP (Stop DMA) Points to next of the error descriptor"]
    #[inline(always)]
    pub fn stdma(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerrorstate::Stdma)
    }
    #[doc = "ST_FDS (Fetch Descriptor) Points to the error descriptor"]
    #[inline(always)]
    pub fn fetchdesc(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerrorstate::Fetchdesc)
    }
    #[doc = "Never set this state (Not used)"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerrorstate::Invalid)
    }
    #[doc = "ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    #[inline(always)]
    pub fn xferdata(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerrorstate::Xferdata)
    }
}
#[doc = "This error occurs in the following 2 cases. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admalenmismatcherr {
    #[doc = "1: Error"]
    Error = 1,
    #[doc = "0: No error"]
    Noerror = 0,
}
impl From<Admalenmismatcherr> for bool {
    #[inline(always)]
    fn from(variant: Admalenmismatcherr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMALENMISMATCHERR` reader - This error occurs in the following 2 cases. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length."]
pub type AdmalenmismatcherrR = crate::BitReader<Admalenmismatcherr>;
impl AdmalenmismatcherrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admalenmismatcherr {
        match self.bits {
            true => Admalenmismatcherr::Error,
            false => Admalenmismatcherr::Noerror,
        }
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Admalenmismatcherr::Error
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == Admalenmismatcherr::Noerror
    }
}
#[doc = "Field `ADMALENMISMATCHERR` writer - This error occurs in the following 2 cases. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length."]
pub type AdmalenmismatcherrW<'a, REG> = crate::BitWriter<'a, REG, Admalenmismatcherr>;
impl<'a, REG> AdmalenmismatcherrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Admalenmismatcherr::Error)
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut crate::W<REG> {
        self.variant(Admalenmismatcherr::Noerror)
    }
}
impl R {
    #[doc = "Bits 0:1 - This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates 10 because ADMA never stops in this state. D01 - D00 : ADMA Error State when error occurred Contents of SYS_SDR register"]
    #[inline(always)]
    pub fn admaerrorstate(&self) -> AdmaerrorstateR {
        AdmaerrorstateR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This error occurs in the following 2 cases. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length."]
    #[inline(always)]
    pub fn admalenmismatcherr(&self) -> AdmalenmismatcherrR {
        AdmalenmismatcherrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates 10 because ADMA never stops in this state. D01 - D00 : ADMA Error State when error occurred Contents of SYS_SDR register"]
    #[inline(always)]
    #[must_use]
    pub fn admaerrorstate(&mut self) -> AdmaerrorstateW<AdmaSpec> {
        AdmaerrorstateW::new(self, 0)
    }
    #[doc = "Bit 2 - This error occurs in the following 2 cases. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length."]
    #[inline(always)]
    #[must_use]
    pub fn admalenmismatcherr(&mut self) -> AdmalenmismatcherrW<AdmaSpec> {
        AdmalenmismatcherrW::new(self, 2)
    }
}
#[doc = "ADMA error status\n\nYou can [`read`](crate::Reg::read) this register and get [`adma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdmaSpec;
impl crate::RegisterSpec for AdmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adma::R`](R) reader structure"]
impl crate::Readable for AdmaSpec {}
#[doc = "`write(|w| ..)` method takes [`adma::W`](W) writer structure"]
impl crate::Writable for AdmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADMA to value 0"]
impl crate::Resettable for AdmaSpec {
    const RESET_VALUE: u32 = 0;
}
