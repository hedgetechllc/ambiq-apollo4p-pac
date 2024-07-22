#[doc = "Register `STATXIPDMA` reader"]
pub type R = crate::R<StatxipdmaSpec>;
#[doc = "Register `STATXIPDMA` writer"]
pub type W = crate::W<StatxipdmaSpec>;
#[doc = "XIP/DMA module debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Fld32 {
    #[doc = "2048: This bit indicates the idle status of XIPDMA."]
    Xipdmaidle = 2048,
}
impl From<Fld32> for u32 {
    #[inline(always)]
    fn from(variant: Fld32) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fld32 {
    type Ux = u32;
}
impl crate::IsEnum for Fld32 {}
#[doc = "Field `FLD32` reader - XIP/DMA module debug"]
pub type Fld32R = crate::FieldReader<Fld32>;
impl Fld32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fld32> {
        match self.bits {
            2048 => Some(Fld32::Xipdmaidle),
            _ => None,
        }
    }
    #[doc = "This bit indicates the idle status of XIPDMA."]
    #[inline(always)]
    pub fn is_xipdmaidle(&self) -> bool {
        *self == Fld32::Xipdmaidle
    }
}
#[doc = "Field `FLD32` writer - XIP/DMA module debug"]
pub type Fld32W<'a, REG> = crate::FieldWriter<'a, REG, 32, Fld32>;
impl<'a, REG> Fld32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "This bit indicates the idle status of XIPDMA."]
    #[inline(always)]
    pub fn xipdmaidle(self) -> &'a mut crate::W<REG> {
        self.variant(Fld32::Xipdmaidle)
    }
}
impl R {
    #[doc = "Bits 0:31 - XIP/DMA module debug"]
    #[inline(always)]
    pub fn fld32(&self) -> Fld32R {
        Fld32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - XIP/DMA module debug"]
    #[inline(always)]
    #[must_use]
    pub fn fld32(&mut self) -> Fld32W<StatxipdmaSpec> {
        Fld32W::new(self, 0)
    }
}
#[doc = "Debug XIP DMA State\n\nYou can [`read`](crate::Reg::read) this register and get [`statxipdma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statxipdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatxipdmaSpec;
impl crate::RegisterSpec for StatxipdmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statxipdma::R`](R) reader structure"]
impl crate::Readable for StatxipdmaSpec {}
#[doc = "`write(|w| ..)` method takes [`statxipdma::W`](W) writer structure"]
impl crate::Writable for StatxipdmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATXIPDMA to value 0"]
impl crate::Resettable for StatxipdmaSpec {
    const RESET_VALUE: u32 = 0;
}
