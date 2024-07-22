#[doc = "Register `DSP1CONTROL` reader"]
pub type R = crate::R<Dsp1controlSpec>;
#[doc = "Register `DSP1CONTROL` writer"]
pub type W = crate::W<Dsp1controlSpec>;
#[doc = "Field `DSP1STATVECSEL` reader - DSP 1 StatVectorSel"]
pub type Dsp1statvecselR = crate::BitReader;
#[doc = "Field `DSP1STATVECSEL` writer - DSP 1 StatVectorSel"]
pub type Dsp1statvecselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP1BRESET` reader - DSP1 BReset. This is the reset used for Xtensa core. S/w must clear this reset to use Dsp."]
pub type Dsp1bresetR = crate::BitReader;
#[doc = "Field `DSP1BRESET` writer - DSP1 BReset. This is the reset used for Xtensa core. S/w must clear this reset to use Dsp."]
pub type Dsp1bresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP1DRESET` reader - DSP1 DReset. This is the reset used for debug functionality like OCD/TRAX etc."]
pub type Dsp1dresetR = crate::BitReader;
#[doc = "Field `DSP1DRESET` writer - DSP1 DReset. This is the reset used for debug functionality like OCD/TRAX etc."]
pub type Dsp1dresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP1RUNSTALL` reader - DSP 1 RunStall. When asserted, DSP 1 will stall until bit is cleared."]
pub type Dsp1runstallR = crate::BitReader;
#[doc = "Field `DSP1RUNSTALL` writer - DSP 1 RunStall. When asserted, DSP 1 will stall until bit is cleared."]
pub type Dsp1runstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DSP 1 IDMA Trigger Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsp1idmatrig {
    #[doc = "3: Trigger is disabled until a cross trigger pulse is asserted. This will allow another source (determined by the XTRIGSRC register) to allow the DMA descriptor chain to proceed."]
    Xtrig = 3,
    #[doc = "2: Trigger is disabled until a trigger pulse (PULSE in IDMATRIG register) is asserted. This will allow a single step in the DMA descriptor chain to be enabled until next completion."]
    Sstep = 2,
    #[doc = "1: Trigger is always enabled. With this set, any trigger out will immediately generate a trigger in"]
    Aon = 1,
    #[doc = "0: Trigger is disabled. This will pause the iDMA indefinitely until enabled."]
    Disable = 0,
}
impl From<Dsp1idmatrig> for u8 {
    #[inline(always)]
    fn from(variant: Dsp1idmatrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsp1idmatrig {
    type Ux = u8;
}
impl crate::IsEnum for Dsp1idmatrig {}
#[doc = "Field `DSP1IDMATRIG` reader - DSP 1 IDMA Trigger Control"]
pub type Dsp1idmatrigR = crate::FieldReader<Dsp1idmatrig>;
impl Dsp1idmatrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsp1idmatrig {
        match self.bits {
            3 => Dsp1idmatrig::Xtrig,
            2 => Dsp1idmatrig::Sstep,
            1 => Dsp1idmatrig::Aon,
            0 => Dsp1idmatrig::Disable,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger is disabled until a cross trigger pulse is asserted. This will allow another source (determined by the XTRIGSRC register) to allow the DMA descriptor chain to proceed."]
    #[inline(always)]
    pub fn is_xtrig(&self) -> bool {
        *self == Dsp1idmatrig::Xtrig
    }
    #[doc = "Trigger is disabled until a trigger pulse (PULSE in IDMATRIG register) is asserted. This will allow a single step in the DMA descriptor chain to be enabled until next completion."]
    #[inline(always)]
    pub fn is_sstep(&self) -> bool {
        *self == Dsp1idmatrig::Sstep
    }
    #[doc = "Trigger is always enabled. With this set, any trigger out will immediately generate a trigger in"]
    #[inline(always)]
    pub fn is_aon(&self) -> bool {
        *self == Dsp1idmatrig::Aon
    }
    #[doc = "Trigger is disabled. This will pause the iDMA indefinitely until enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dsp1idmatrig::Disable
    }
}
#[doc = "Field `DSP1IDMATRIG` writer - DSP 1 IDMA Trigger Control"]
pub type Dsp1idmatrigW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsp1idmatrig, crate::Safe>;
impl<'a, REG> Dsp1idmatrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger is disabled until a cross trigger pulse is asserted. This will allow another source (determined by the XTRIGSRC register) to allow the DMA descriptor chain to proceed."]
    #[inline(always)]
    pub fn xtrig(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1idmatrig::Xtrig)
    }
    #[doc = "Trigger is disabled until a trigger pulse (PULSE in IDMATRIG register) is asserted. This will allow a single step in the DMA descriptor chain to be enabled until next completion."]
    #[inline(always)]
    pub fn sstep(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1idmatrig::Sstep)
    }
    #[doc = "Trigger is always enabled. With this set, any trigger out will immediately generate a trigger in"]
    #[inline(always)]
    pub fn aon(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1idmatrig::Aon)
    }
    #[doc = "Trigger is disabled. This will pause the iDMA indefinitely until enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp1idmatrig::Disable)
    }
}
#[doc = "Field `DSP1IDMAXTRIGSRC` reader - DSP 1 IDMA Cross Trigger Source. All enabled sources are ANDed to generate a trigger enable. \n Bit30-12:IRQ18-0 Bit11: IDMATRIGPULSE, Bit10: DSP Timer1, Bit9: DSP Timer0, Bit8: alternate DSP iDMA trigger out"]
pub type Dsp1idmaxtrigsrcR = crate::FieldReader<u32>;
#[doc = "Field `DSP1IDMAXTRIGSRC` writer - DSP 1 IDMA Cross Trigger Source. All enabled sources are ANDed to generate a trigger enable. \n Bit30-12:IRQ18-0 Bit11: IDMATRIGPULSE, Bit10: DSP Timer1, Bit9: DSP Timer0, Bit8: alternate DSP iDMA trigger out"]
pub type Dsp1idmaxtrigsrcW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - DSP 1 StatVectorSel"]
    #[inline(always)]
    pub fn dsp1statvecsel(&self) -> Dsp1statvecselR {
        Dsp1statvecselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSP1 BReset. This is the reset used for Xtensa core. S/w must clear this reset to use Dsp."]
    #[inline(always)]
    pub fn dsp1breset(&self) -> Dsp1bresetR {
        Dsp1bresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DSP1 DReset. This is the reset used for debug functionality like OCD/TRAX etc."]
    #[inline(always)]
    pub fn dsp1dreset(&self) -> Dsp1dresetR {
        Dsp1dresetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSP 1 RunStall. When asserted, DSP 1 will stall until bit is cleared."]
    #[inline(always)]
    pub fn dsp1runstall(&self) -> Dsp1runstallR {
        Dsp1runstallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DSP 1 IDMA Trigger Control"]
    #[inline(always)]
    pub fn dsp1idmatrig(&self) -> Dsp1idmatrigR {
        Dsp1idmatrigR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:30 - DSP 1 IDMA Cross Trigger Source. All enabled sources are ANDed to generate a trigger enable. \n Bit30-12:IRQ18-0 Bit11: IDMATRIGPULSE, Bit10: DSP Timer1, Bit9: DSP Timer0, Bit8: alternate DSP iDMA trigger out"]
    #[inline(always)]
    pub fn dsp1idmaxtrigsrc(&self) -> Dsp1idmaxtrigsrcR {
        Dsp1idmaxtrigsrcR::new((self.bits >> 8) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - DSP 1 StatVectorSel"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1statvecsel(&mut self) -> Dsp1statvecselW<Dsp1controlSpec> {
        Dsp1statvecselW::new(self, 0)
    }
    #[doc = "Bit 1 - DSP1 BReset. This is the reset used for Xtensa core. S/w must clear this reset to use Dsp."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1breset(&mut self) -> Dsp1bresetW<Dsp1controlSpec> {
        Dsp1bresetW::new(self, 1)
    }
    #[doc = "Bit 2 - DSP1 DReset. This is the reset used for debug functionality like OCD/TRAX etc."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1dreset(&mut self) -> Dsp1dresetW<Dsp1controlSpec> {
        Dsp1dresetW::new(self, 2)
    }
    #[doc = "Bit 3 - DSP 1 RunStall. When asserted, DSP 1 will stall until bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn dsp1runstall(&mut self) -> Dsp1runstallW<Dsp1controlSpec> {
        Dsp1runstallW::new(self, 3)
    }
    #[doc = "Bits 4:5 - DSP 1 IDMA Trigger Control"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1idmatrig(&mut self) -> Dsp1idmatrigW<Dsp1controlSpec> {
        Dsp1idmatrigW::new(self, 4)
    }
    #[doc = "Bits 8:30 - DSP 1 IDMA Cross Trigger Source. All enabled sources are ANDed to generate a trigger enable. \n Bit30-12:IRQ18-0 Bit11: IDMATRIGPULSE, Bit10: DSP Timer1, Bit9: DSP Timer0, Bit8: alternate DSP iDMA trigger out"]
    #[inline(always)]
    #[must_use]
    pub fn dsp1idmaxtrigsrc(&mut self) -> Dsp1idmaxtrigsrcW<Dsp1controlSpec> {
        Dsp1idmaxtrigsrcW::new(self, 8)
    }
}
#[doc = "DSP 1 control settings\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp1controlSpec;
impl crate::RegisterSpec for Dsp1controlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp1control::R`](R) reader structure"]
impl crate::Readable for Dsp1controlSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp1control::W`](W) writer structure"]
impl crate::Writable for Dsp1controlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP1CONTROL to value 0x06"]
impl crate::Resettable for Dsp1controlSpec {
    const RESET_VALUE: u32 = 0x06;
}
