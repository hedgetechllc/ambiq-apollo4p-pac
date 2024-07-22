#[doc = "Register `CQPAUSE` reader"]
pub type R = crate::R<CqpauseSpec>;
#[doc = "Register `CQPAUSE` writer"]
pub type W = crate::W<CqpauseSpec>;
#[doc = "CQ will pause processing when ALL specified events are satisfied -- i.e. when (CQMASK and CQPAUSE)==CQMASK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Cqmask {
    #[doc = "32768: CQ Stop Flag. When set, CQ processing will complete."]
    Stop = 32768,
    #[doc = "16384: CQ Index Pointers (CURIDX/ENDIDX) match."]
    Cqidx = 16384,
    #[doc = "8192: Buffer 1 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM1START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    Buf1xoren = 8192,
    #[doc = "4096: Buffer 0 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    Buf0xoren = 4096,
    #[doc = "2048: DMA Complete Status (hardwired DMACPL bit in DMASTAT)"]
    Dmacpl = 2048,
    #[doc = "1024: PIO Operation completed (STATUS bit in CTRL register)"]
    Cmdcpl = 1024,
    #[doc = "512: (BUF1XNOREN) IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    Iom1ready = 512,
    #[doc = "256: (BUF0XNOREN) IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    Iom0ready = 256,
    #[doc = "128: Software flag 7. Can be used by software to start/pause operations."]
    Swflag7 = 128,
    #[doc = "64: Software flag 6. Can be used by software to start/pause operations."]
    Swflag6 = 64,
    #[doc = "32: Software flag 5. Can be used by software to start/pause operations."]
    Swflag5 = 32,
    #[doc = "16: Software flag 4. Can be used by software to start/pause operations."]
    Swflag4 = 16,
    #[doc = "8: Software flag 3. Can be used by software to start/pause operations."]
    Swflag3 = 8,
    #[doc = "4: Software flag 2. Can be used by software to start/pause operations."]
    Swflag2 = 4,
    #[doc = "2: Software flag 1. Can be used by software to start/pause operations. Also, IOM Buffer 1 status (same as SWFLAG1). When linked to IOM, indicates to IOM that buffer 1 is ready."]
    Swflag1 = 2,
    #[doc = "1: Software flag 0. Can be used by software to start/pause operations. Also, IOM Buffer 0 status (same as SWFLAG0). When linked to IOM, indicates to IOM that buffer 0 is ready."]
    Swflag0 = 1,
}
impl From<Cqmask> for u16 {
    #[inline(always)]
    fn from(variant: Cqmask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cqmask {
    type Ux = u16;
}
impl crate::IsEnum for Cqmask {}
#[doc = "Field `CQMASK` reader - CQ will pause processing when ALL specified events are satisfied -- i.e. when (CQMASK and CQPAUSE)==CQMASK."]
pub type CqmaskR = crate::FieldReader<Cqmask>;
impl CqmaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cqmask> {
        match self.bits {
            32768 => Some(Cqmask::Stop),
            16384 => Some(Cqmask::Cqidx),
            8192 => Some(Cqmask::Buf1xoren),
            4096 => Some(Cqmask::Buf0xoren),
            2048 => Some(Cqmask::Dmacpl),
            1024 => Some(Cqmask::Cmdcpl),
            512 => Some(Cqmask::Iom1ready),
            256 => Some(Cqmask::Iom0ready),
            128 => Some(Cqmask::Swflag7),
            64 => Some(Cqmask::Swflag6),
            32 => Some(Cqmask::Swflag5),
            16 => Some(Cqmask::Swflag4),
            8 => Some(Cqmask::Swflag3),
            4 => Some(Cqmask::Swflag2),
            2 => Some(Cqmask::Swflag1),
            1 => Some(Cqmask::Swflag0),
            _ => None,
        }
    }
    #[doc = "CQ Stop Flag. When set, CQ processing will complete."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Cqmask::Stop
    }
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match."]
    #[inline(always)]
    pub fn is_cqidx(&self) -> bool {
        *self == Cqmask::Cqidx
    }
    #[doc = "Buffer 1 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM1START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    #[inline(always)]
    pub fn is_buf1xoren(&self) -> bool {
        *self == Cqmask::Buf1xoren
    }
    #[doc = "Buffer 0 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    #[inline(always)]
    pub fn is_buf0xoren(&self) -> bool {
        *self == Cqmask::Buf0xoren
    }
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT)"]
    #[inline(always)]
    pub fn is_dmacpl(&self) -> bool {
        *self == Cqmask::Dmacpl
    }
    #[doc = "PIO Operation completed (STATUS bit in CTRL register)"]
    #[inline(always)]
    pub fn is_cmdcpl(&self) -> bool {
        *self == Cqmask::Cmdcpl
    }
    #[doc = "(BUF1XNOREN) IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    #[inline(always)]
    pub fn is_iom1ready(&self) -> bool {
        *self == Cqmask::Iom1ready
    }
    #[doc = "(BUF0XNOREN) IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    #[inline(always)]
    pub fn is_iom0ready(&self) -> bool {
        *self == Cqmask::Iom0ready
    }
    #[doc = "Software flag 7. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag7(&self) -> bool {
        *self == Cqmask::Swflag7
    }
    #[doc = "Software flag 6. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag6(&self) -> bool {
        *self == Cqmask::Swflag6
    }
    #[doc = "Software flag 5. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag5(&self) -> bool {
        *self == Cqmask::Swflag5
    }
    #[doc = "Software flag 4. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag4(&self) -> bool {
        *self == Cqmask::Swflag4
    }
    #[doc = "Software flag 3. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag3(&self) -> bool {
        *self == Cqmask::Swflag3
    }
    #[doc = "Software flag 2. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn is_swflag2(&self) -> bool {
        *self == Cqmask::Swflag2
    }
    #[doc = "Software flag 1. Can be used by software to start/pause operations. Also, IOM Buffer 1 status (same as SWFLAG1). When linked to IOM, indicates to IOM that buffer 1 is ready."]
    #[inline(always)]
    pub fn is_swflag1(&self) -> bool {
        *self == Cqmask::Swflag1
    }
    #[doc = "Software flag 0. Can be used by software to start/pause operations. Also, IOM Buffer 0 status (same as SWFLAG0). When linked to IOM, indicates to IOM that buffer 0 is ready."]
    #[inline(always)]
    pub fn is_swflag0(&self) -> bool {
        *self == Cqmask::Swflag0
    }
}
#[doc = "Field `CQMASK` writer - CQ will pause processing when ALL specified events are satisfied -- i.e. when (CQMASK and CQPAUSE)==CQMASK."]
pub type CqmaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, Cqmask>;
impl<'a, REG> CqmaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "CQ Stop Flag. When set, CQ processing will complete."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Stop)
    }
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match."]
    #[inline(always)]
    pub fn cqidx(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Cqidx)
    }
    #[doc = "Buffer 1 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM1START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    #[inline(always)]
    pub fn buf1xoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Buf1xoren)
    }
    #[doc = "Buffer 0 Ready Status (from selected IOM/MSPI). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can transfer the buffer."]
    #[inline(always)]
    pub fn buf0xoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Buf0xoren)
    }
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT)"]
    #[inline(always)]
    pub fn dmacpl(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Dmacpl)
    }
    #[doc = "PIO Operation completed (STATUS bit in CTRL register)"]
    #[inline(always)]
    pub fn cmdcpl(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Cmdcpl)
    }
    #[doc = "(BUF1XNOREN) IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    #[inline(always)]
    pub fn iom1ready(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Iom1ready)
    }
    #[doc = "(BUF0XNOREN) IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer."]
    #[inline(always)]
    pub fn iom0ready(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Iom0ready)
    }
    #[doc = "Software flag 7. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag7(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Swflag7)
    }
    #[doc = "Software flag 6. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag6(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Swflag6)
    }
    #[doc = "Software flag 5. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag5(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Swflag5)
    }
    #[doc = "Software flag 4. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag4(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Swflag4)
    }
    #[doc = "Software flag 3. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag3(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Swflag3)
    }
    #[doc = "Software flag 2. Can be used by software to start/pause operations."]
    #[inline(always)]
    pub fn swflag2(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Swflag2)
    }
    #[doc = "Software flag 1. Can be used by software to start/pause operations. Also, IOM Buffer 1 status (same as SWFLAG1). When linked to IOM, indicates to IOM that buffer 1 is ready."]
    #[inline(always)]
    pub fn swflag1(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Swflag1)
    }
    #[doc = "Software flag 0. Can be used by software to start/pause operations. Also, IOM Buffer 0 status (same as SWFLAG0). When linked to IOM, indicates to IOM that buffer 0 is ready."]
    #[inline(always)]
    pub fn swflag0(self) -> &'a mut crate::W<REG> {
        self.variant(Cqmask::Swflag0)
    }
}
impl R {
    #[doc = "Bits 0:15 - CQ will pause processing when ALL specified events are satisfied -- i.e. when (CQMASK and CQPAUSE)==CQMASK."]
    #[inline(always)]
    pub fn cqmask(&self) -> CqmaskR {
        CqmaskR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CQ will pause processing when ALL specified events are satisfied -- i.e. when (CQMASK and CQPAUSE)==CQMASK."]
    #[inline(always)]
    #[must_use]
    pub fn cqmask(&mut self) -> CqmaskW<CqpauseSpec> {
        CqmaskW::new(self, 0)
    }
}
#[doc = "Command Queue Pause Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`cqpause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqpause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqpauseSpec;
impl crate::RegisterSpec for CqpauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqpause::R`](R) reader structure"]
impl crate::Readable for CqpauseSpec {}
#[doc = "`write(|w| ..)` method takes [`cqpause::W`](W) writer structure"]
impl crate::Writable for CqpauseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQPAUSE to value 0"]
impl crate::Resettable for CqpauseSpec {
    const RESET_VALUE: u32 = 0;
}
