#[doc = "Register `CHACHABYTEWORDORDERCNTLREG` reader"]
pub type R = crate::R<ChachabytewordordercntlregSpec>;
#[doc = "Register `CHACHABYTEWORDORDERCNTLREG` writer"]
pub type W = crate::W<ChachabytewordordercntlregSpec>;
#[doc = "Change the words order of the input data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chachadinwordorder {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. (reverse each word in 128 bit input ( w0->w3, w1->w2, w2->w1,w3-w0))"]
    Enable = 1,
}
impl From<Chachadinwordorder> for bool {
    #[inline(always)]
    fn from(variant: Chachadinwordorder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHADINWORDORDER` reader - Change the words order of the input data."]
pub type ChachadinwordorderR = crate::BitReader<Chachadinwordorder>;
impl ChachadinwordorderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chachadinwordorder {
        match self.bits {
            false => Chachadinwordorder::Disable,
            true => Chachadinwordorder::Enable,
        }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chachadinwordorder::Disable
    }
    #[doc = "enable. (reverse each word in 128 bit input ( w0->w3, w1->w2, w2->w1,w3-w0))"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chachadinwordorder::Enable
    }
}
#[doc = "Field `CHACHADINWORDORDER` writer - Change the words order of the input data."]
pub type ChachadinwordorderW<'a, REG> = crate::BitWriter<'a, REG, Chachadinwordorder>;
impl<'a, REG> ChachadinwordorderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachadinwordorder::Disable)
    }
    #[doc = "enable. (reverse each word in 128 bit input ( w0->w3, w1->w2, w2->w1,w3-w0))"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachadinwordorder::Enable)
    }
}
#[doc = "Change the byte order of the input data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chachadinbyteorder {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. (reverse each byte in each word input (b0->b3, b1->b2, b2->b1,b3->b0))"]
    Enable = 1,
}
impl From<Chachadinbyteorder> for bool {
    #[inline(always)]
    fn from(variant: Chachadinbyteorder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHADINBYTEORDER` reader - Change the byte order of the input data."]
pub type ChachadinbyteorderR = crate::BitReader<Chachadinbyteorder>;
impl ChachadinbyteorderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chachadinbyteorder {
        match self.bits {
            false => Chachadinbyteorder::Disable,
            true => Chachadinbyteorder::Enable,
        }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chachadinbyteorder::Disable
    }
    #[doc = "enable. (reverse each byte in each word input (b0->b3, b1->b2, b2->b1,b3->b0))"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chachadinbyteorder::Enable
    }
}
#[doc = "Field `CHACHADINBYTEORDER` writer - Change the byte order of the input data."]
pub type ChachadinbyteorderW<'a, REG> = crate::BitWriter<'a, REG, Chachadinbyteorder>;
impl<'a, REG> ChachadinbyteorderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachadinbyteorder::Disable)
    }
    #[doc = "enable. (reverse each byte in each word input (b0->b3, b1->b2, b2->b1,b3->b0))"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachadinbyteorder::Enable)
    }
}
#[doc = "Change the quarter of a matrix order in core\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chachacorematrixlbeorder {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. (reverse each quarter of a matrix (m\\[0-127\\]->m\\[384-511\\], m\\[128-255\\]->m\\[256-383\\], m\\[256-383\\]->m\\[128-255\\], m\\[384-511\\]->m\\[0-127\\]))"]
    Enable = 1,
}
impl From<Chachacorematrixlbeorder> for bool {
    #[inline(always)]
    fn from(variant: Chachacorematrixlbeorder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHACOREMATRIXLBEORDER` reader - Change the quarter of a matrix order in core"]
pub type ChachacorematrixlbeorderR = crate::BitReader<Chachacorematrixlbeorder>;
impl ChachacorematrixlbeorderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chachacorematrixlbeorder {
        match self.bits {
            false => Chachacorematrixlbeorder::Disable,
            true => Chachacorematrixlbeorder::Enable,
        }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chachacorematrixlbeorder::Disable
    }
    #[doc = "enable. (reverse each quarter of a matrix (m\\[0-127\\]->m\\[384-511\\], m\\[128-255\\]->m\\[256-383\\], m\\[256-383\\]->m\\[128-255\\], m\\[384-511\\]->m\\[0-127\\]))"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chachacorematrixlbeorder::Enable
    }
}
#[doc = "Field `CHACHACOREMATRIXLBEORDER` writer - Change the quarter of a matrix order in core"]
pub type ChachacorematrixlbeorderW<'a, REG> = crate::BitWriter<'a, REG, Chachacorematrixlbeorder>;
impl<'a, REG> ChachacorematrixlbeorderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachacorematrixlbeorder::Disable)
    }
    #[doc = "enable. (reverse each quarter of a matrix (m\\[0-127\\]->m\\[384-511\\], m\\[128-255\\]->m\\[256-383\\], m\\[256-383\\]->m\\[128-255\\], m\\[384-511\\]->m\\[0-127\\]))"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachacorematrixlbeorder::Enable)
    }
}
#[doc = "Change the words order of the output data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chachadoutwordorder {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. (reverse each word in 128 bit output ( w0->w3, w1->w2, w2->w1,w3-w0))"]
    Enable = 1,
}
impl From<Chachadoutwordorder> for bool {
    #[inline(always)]
    fn from(variant: Chachadoutwordorder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHADOUTWORDORDER` reader - Change the words order of the output data."]
pub type ChachadoutwordorderR = crate::BitReader<Chachadoutwordorder>;
impl ChachadoutwordorderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chachadoutwordorder {
        match self.bits {
            false => Chachadoutwordorder::Disable,
            true => Chachadoutwordorder::Enable,
        }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chachadoutwordorder::Disable
    }
    #[doc = "enable. (reverse each word in 128 bit output ( w0->w3, w1->w2, w2->w1,w3-w0))"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chachadoutwordorder::Enable
    }
}
#[doc = "Field `CHACHADOUTWORDORDER` writer - Change the words order of the output data."]
pub type ChachadoutwordorderW<'a, REG> = crate::BitWriter<'a, REG, Chachadoutwordorder>;
impl<'a, REG> ChachadoutwordorderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachadoutwordorder::Disable)
    }
    #[doc = "enable. (reverse each word in 128 bit output ( w0->w3, w1->w2, w2->w1,w3-w0))"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachadoutwordorder::Enable)
    }
}
#[doc = "Change the byte order of the output data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chachadoutbyteorder {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. (reverse each byte in each word output (b0->b3, b1->b2, b2->b1,b3->b0))"]
    Enable = 1,
}
impl From<Chachadoutbyteorder> for bool {
    #[inline(always)]
    fn from(variant: Chachadoutbyteorder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHADOUTBYTEORDER` reader - Change the byte order of the output data."]
pub type ChachadoutbyteorderR = crate::BitReader<Chachadoutbyteorder>;
impl ChachadoutbyteorderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chachadoutbyteorder {
        match self.bits {
            false => Chachadoutbyteorder::Disable,
            true => Chachadoutbyteorder::Enable,
        }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chachadoutbyteorder::Disable
    }
    #[doc = "enable. (reverse each byte in each word output (b0->b3, b1->b2, b2->b1,b3->b0))"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chachadoutbyteorder::Enable
    }
}
#[doc = "Field `CHACHADOUTBYTEORDER` writer - Change the byte order of the output data."]
pub type ChachadoutbyteorderW<'a, REG> = crate::BitWriter<'a, REG, Chachadoutbyteorder>;
impl<'a, REG> ChachadoutbyteorderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachadoutbyteorder::Disable)
    }
    #[doc = "enable. (reverse each byte in each word output (b0->b3, b1->b2, b2->b1,b3->b0))"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachadoutbyteorder::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Change the words order of the input data."]
    #[inline(always)]
    pub fn chachadinwordorder(&self) -> ChachadinwordorderR {
        ChachadinwordorderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Change the byte order of the input data."]
    #[inline(always)]
    pub fn chachadinbyteorder(&self) -> ChachadinbyteorderR {
        ChachadinbyteorderR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Change the quarter of a matrix order in core"]
    #[inline(always)]
    pub fn chachacorematrixlbeorder(&self) -> ChachacorematrixlbeorderR {
        ChachacorematrixlbeorderR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Change the words order of the output data."]
    #[inline(always)]
    pub fn chachadoutwordorder(&self) -> ChachadoutwordorderR {
        ChachadoutwordorderR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Change the byte order of the output data."]
    #[inline(always)]
    pub fn chachadoutbyteorder(&self) -> ChachadoutbyteorderR {
        ChachadoutbyteorderR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Change the words order of the input data."]
    #[inline(always)]
    #[must_use]
    pub fn chachadinwordorder(&mut self) -> ChachadinwordorderW<ChachabytewordordercntlregSpec> {
        ChachadinwordorderW::new(self, 0)
    }
    #[doc = "Bit 1 - Change the byte order of the input data."]
    #[inline(always)]
    #[must_use]
    pub fn chachadinbyteorder(&mut self) -> ChachadinbyteorderW<ChachabytewordordercntlregSpec> {
        ChachadinbyteorderW::new(self, 1)
    }
    #[doc = "Bit 2 - Change the quarter of a matrix order in core"]
    #[inline(always)]
    #[must_use]
    pub fn chachacorematrixlbeorder(
        &mut self,
    ) -> ChachacorematrixlbeorderW<ChachabytewordordercntlregSpec> {
        ChachacorematrixlbeorderW::new(self, 2)
    }
    #[doc = "Bit 3 - Change the words order of the output data."]
    #[inline(always)]
    #[must_use]
    pub fn chachadoutwordorder(&mut self) -> ChachadoutwordorderW<ChachabytewordordercntlregSpec> {
        ChachadoutwordorderW::new(self, 3)
    }
    #[doc = "Bit 4 - Change the byte order of the output data."]
    #[inline(always)]
    #[must_use]
    pub fn chachadoutbyteorder(&mut self) -> ChachadoutbyteorderW<ChachabytewordordercntlregSpec> {
        ChachadoutbyteorderW::new(self, 4)
    }
}
#[doc = "CHACHA_SALSA DATA ORDER configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachabytewordordercntlreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachabytewordordercntlreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachabytewordordercntlregSpec;
impl crate::RegisterSpec for ChachabytewordordercntlregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachabytewordordercntlreg::R`](R) reader structure"]
impl crate::Readable for ChachabytewordordercntlregSpec {}
#[doc = "`write(|w| ..)` method takes [`chachabytewordordercntlreg::W`](W) writer structure"]
impl crate::Writable for ChachabytewordordercntlregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHABYTEWORDORDERCNTLREG to value 0"]
impl crate::Resettable for ChachabytewordordercntlregSpec {
    const RESET_VALUE: u32 = 0;
}
