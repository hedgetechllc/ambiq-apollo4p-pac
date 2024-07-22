#[doc = "Register `LCSREG` reader"]
pub type R = crate::R<LcsregSpec>;
#[doc = "Register `LCSREG` writer"]
pub type W = crate::W<LcsregSpec>;
#[doc = "Indicates the LCS (Lifecycle State) value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcsreg {
    #[doc = "0: CM lifecycle state"]
    Cm = 0,
    #[doc = "1: DM lifecycle state"]
    Dm = 1,
    #[doc = "5: SE lifecycle state"]
    Se = 5,
    #[doc = "7: RMA lifecycle state"]
    Rma = 7,
}
impl From<Lcsreg> for u8 {
    #[inline(always)]
    fn from(variant: Lcsreg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcsreg {
    type Ux = u8;
}
impl crate::IsEnum for Lcsreg {}
#[doc = "Field `LCSREG` reader - Indicates the LCS (Lifecycle State) value."]
pub type LcsregR = crate::FieldReader<Lcsreg>;
impl LcsregR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lcsreg> {
        match self.bits {
            0 => Some(Lcsreg::Cm),
            1 => Some(Lcsreg::Dm),
            5 => Some(Lcsreg::Se),
            7 => Some(Lcsreg::Rma),
            _ => None,
        }
    }
    #[doc = "CM lifecycle state"]
    #[inline(always)]
    pub fn is_cm(&self) -> bool {
        *self == Lcsreg::Cm
    }
    #[doc = "DM lifecycle state"]
    #[inline(always)]
    pub fn is_dm(&self) -> bool {
        *self == Lcsreg::Dm
    }
    #[doc = "SE lifecycle state"]
    #[inline(always)]
    pub fn is_se(&self) -> bool {
        *self == Lcsreg::Se
    }
    #[doc = "RMA lifecycle state"]
    #[inline(always)]
    pub fn is_rma(&self) -> bool {
        *self == Lcsreg::Rma
    }
}
#[doc = "Field `LCSREG` writer - Indicates the LCS (Lifecycle State) value."]
pub type LcsregW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcsreg>;
impl<'a, REG> LcsregW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CM lifecycle state"]
    #[inline(always)]
    pub fn cm(self) -> &'a mut crate::W<REG> {
        self.variant(Lcsreg::Cm)
    }
    #[doc = "DM lifecycle state"]
    #[inline(always)]
    pub fn dm(self) -> &'a mut crate::W<REG> {
        self.variant(Lcsreg::Dm)
    }
    #[doc = "SE lifecycle state"]
    #[inline(always)]
    pub fn se(self) -> &'a mut crate::W<REG> {
        self.variant(Lcsreg::Se)
    }
    #[doc = "RMA lifecycle state"]
    #[inline(always)]
    pub fn rma(self) -> &'a mut crate::W<REG> {
        self.variant(Lcsreg::Rma)
    }
}
#[doc = "Field `ERRORKDRZEROCNT` reader - Indication that the number of zeroes in the loaded KDR is not equal to the value set in the manufacture flag."]
pub type ErrorkdrzerocntR = crate::BitReader;
#[doc = "Field `ERRORKDRZEROCNT` writer - Indication that the number of zeroes in the loaded KDR is not equal to the value set in the manufacture flag."]
pub type ErrorkdrzerocntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORPROVZEROCNT` reader - Indication that the number of zeroes in the loaded KCP is not equal to the value set in the OEM flag."]
pub type ErrorprovzerocntR = crate::BitReader;
#[doc = "Field `ERRORPROVZEROCNT` writer - Indication that the number of zeroes in the loaded KCP is not equal to the value set in the OEM flag."]
pub type ErrorprovzerocntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORKCEZEROCNT` reader - Indication that the number of zeroes in the loaded KCE is not equal to the value set in the OEM flag."]
pub type ErrorkcezerocntR = crate::BitReader;
#[doc = "Field `ERRORKCEZEROCNT` writer - Indication that the number of zeroes in the loaded KCE is not equal to the value set in the OEM flag."]
pub type ErrorkcezerocntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORKPICVZEROCNT` reader - Indication that the number of zeroes in the loaded KPICV is not equal to the value set in the manufacture flag."]
pub type ErrorkpicvzerocntR = crate::BitReader;
#[doc = "Field `ERRORKPICVZEROCNT` writer - Indication that the number of zeroes in the loaded KPICV is not equal to the value set in the manufacture flag."]
pub type ErrorkpicvzerocntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORKCEICVZEROCNT` reader - Indication that the number of zeroes in the loaded KCEICV is not equal to the value set in the manufacture flag."]
pub type ErrorkceicvzerocntR = crate::BitReader;
#[doc = "Field `ERRORKCEICVZEROCNT` writer - Indication that the number of zeroes in the loaded KCEICV is not equal to the value set in the manufacture flag."]
pub type ErrorkceicvzerocntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Indicates the LCS (Lifecycle State) value."]
    #[inline(always)]
    pub fn lcsreg(&self) -> LcsregR {
        LcsregR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Indication that the number of zeroes in the loaded KDR is not equal to the value set in the manufacture flag."]
    #[inline(always)]
    pub fn errorkdrzerocnt(&self) -> ErrorkdrzerocntR {
        ErrorkdrzerocntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indication that the number of zeroes in the loaded KCP is not equal to the value set in the OEM flag."]
    #[inline(always)]
    pub fn errorprovzerocnt(&self) -> ErrorprovzerocntR {
        ErrorprovzerocntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indication that the number of zeroes in the loaded KCE is not equal to the value set in the OEM flag."]
    #[inline(always)]
    pub fn errorkcezerocnt(&self) -> ErrorkcezerocntR {
        ErrorkcezerocntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indication that the number of zeroes in the loaded KPICV is not equal to the value set in the manufacture flag."]
    #[inline(always)]
    pub fn errorkpicvzerocnt(&self) -> ErrorkpicvzerocntR {
        ErrorkpicvzerocntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indication that the number of zeroes in the loaded KCEICV is not equal to the value set in the manufacture flag."]
    #[inline(always)]
    pub fn errorkceicvzerocnt(&self) -> ErrorkceicvzerocntR {
        ErrorkceicvzerocntR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Indicates the LCS (Lifecycle State) value."]
    #[inline(always)]
    #[must_use]
    pub fn lcsreg(&mut self) -> LcsregW<LcsregSpec> {
        LcsregW::new(self, 0)
    }
    #[doc = "Bit 8 - Indication that the number of zeroes in the loaded KDR is not equal to the value set in the manufacture flag."]
    #[inline(always)]
    #[must_use]
    pub fn errorkdrzerocnt(&mut self) -> ErrorkdrzerocntW<LcsregSpec> {
        ErrorkdrzerocntW::new(self, 8)
    }
    #[doc = "Bit 9 - Indication that the number of zeroes in the loaded KCP is not equal to the value set in the OEM flag."]
    #[inline(always)]
    #[must_use]
    pub fn errorprovzerocnt(&mut self) -> ErrorprovzerocntW<LcsregSpec> {
        ErrorprovzerocntW::new(self, 9)
    }
    #[doc = "Bit 10 - Indication that the number of zeroes in the loaded KCE is not equal to the value set in the OEM flag."]
    #[inline(always)]
    #[must_use]
    pub fn errorkcezerocnt(&mut self) -> ErrorkcezerocntW<LcsregSpec> {
        ErrorkcezerocntW::new(self, 10)
    }
    #[doc = "Bit 11 - Indication that the number of zeroes in the loaded KPICV is not equal to the value set in the manufacture flag."]
    #[inline(always)]
    #[must_use]
    pub fn errorkpicvzerocnt(&mut self) -> ErrorkpicvzerocntW<LcsregSpec> {
        ErrorkpicvzerocntW::new(self, 11)
    }
    #[doc = "Bit 12 - Indication that the number of zeroes in the loaded KCEICV is not equal to the value set in the manufacture flag."]
    #[inline(always)]
    #[must_use]
    pub fn errorkceicvzerocnt(&mut self) -> ErrorkceicvzerocntW<LcsregSpec> {
        ErrorkceicvzerocntW::new(self, 12)
    }
}
#[doc = "The lifecycle state register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcsregSpec;
impl crate::RegisterSpec for LcsregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsreg::R`](R) reader structure"]
impl crate::Readable for LcsregSpec {}
#[doc = "`write(|w| ..)` method takes [`lcsreg::W`](W) writer structure"]
impl crate::Writable for LcsregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCSREG to value 0"]
impl crate::Resettable for LcsregSpec {
    const RESET_VALUE: u32 = 0;
}
