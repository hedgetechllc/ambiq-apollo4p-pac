#[doc = "Register `DOUTFIFOEMPTY` reader"]
pub type R = crate::R<DoutfifoemptySpec>;
#[doc = "Register `DOUTFIFOEMPTY` writer"]
pub type W = crate::W<DoutfifoemptySpec>;
#[doc = "DOUT FIFO empty status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutfifoempty {
    #[doc = "0: DOUT FIFO is not empty"]
    FifoNe = 0,
    #[doc = "1: FIFO is empty"]
    FifoEdout = 1,
}
impl From<Doutfifoempty> for bool {
    #[inline(always)]
    fn from(variant: Doutfifoempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTFIFOEMPTY` reader - DOUT FIFO empty status."]
pub type DoutfifoemptyR = crate::BitReader<Doutfifoempty>;
impl DoutfifoemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doutfifoempty {
        match self.bits {
            false => Doutfifoempty::FifoNe,
            true => Doutfifoempty::FifoEdout,
        }
    }
    #[doc = "DOUT FIFO is not empty"]
    #[inline(always)]
    pub fn is_fifo_ne(&self) -> bool {
        *self == Doutfifoempty::FifoNe
    }
    #[doc = "FIFO is empty"]
    #[inline(always)]
    pub fn is_fifo_edout(&self) -> bool {
        *self == Doutfifoempty::FifoEdout
    }
}
#[doc = "Field `DOUTFIFOEMPTY` writer - DOUT FIFO empty status."]
pub type DoutfifoemptyW<'a, REG> = crate::BitWriter<'a, REG, Doutfifoempty>;
impl<'a, REG> DoutfifoemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DOUT FIFO is not empty"]
    #[inline(always)]
    pub fn fifo_ne(self) -> &'a mut crate::W<REG> {
        self.variant(Doutfifoempty::FifoNe)
    }
    #[doc = "FIFO is empty"]
    #[inline(always)]
    pub fn fifo_edout(self) -> &'a mut crate::W<REG> {
        self.variant(Doutfifoempty::FifoEdout)
    }
}
impl R {
    #[doc = "Bit 0 - DOUT FIFO empty status."]
    #[inline(always)]
    pub fn doutfifoempty(&self) -> DoutfifoemptyR {
        DoutfifoemptyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DOUT FIFO empty status."]
    #[inline(always)]
    #[must_use]
    pub fn doutfifoempty(&mut self) -> DoutfifoemptyW<DoutfifoemptySpec> {
        DoutfifoemptyW::new(self, 0)
    }
}
#[doc = "DOUT_FIFO_EMPTY Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`doutfifoempty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutfifoempty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutfifoemptySpec;
impl crate::RegisterSpec for DoutfifoemptySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutfifoempty::R`](R) reader structure"]
impl crate::Readable for DoutfifoemptySpec {}
#[doc = "`write(|w| ..)` method takes [`doutfifoempty::W`](W) writer structure"]
impl crate::Writable for DoutfifoemptySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTFIFOEMPTY to value 0x01"]
impl crate::Resettable for DoutfifoemptySpec {
    const RESET_VALUE: u32 = 0x01;
}
