#[doc = "Register `PART` reader"]
pub type R = crate::R<PartSpec>;
#[doc = "Part code\n\nValue on reset: 37216"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Part {
    #[doc = "37216: nRF9160"]
    N9160 = 37216,
}
impl From<Part> for u32 {
    #[inline(always)]
    fn from(variant: Part) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Part {
    type Ux = u32;
}
impl crate::IsEnum for Part {}
#[doc = "Field `PART` reader - Part code"]
pub type PartR = crate::FieldReader<Part>;
impl PartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Part> {
        match self.bits {
            37216 => Some(Part::N9160),
            _ => None,
        }
    }
    #[doc = "nRF9160"]
    #[inline(always)]
    pub fn is_n9160(&self) -> bool {
        *self == Part::N9160
    }
}
impl R {
    #[doc = "Bits 0:31 - Part code"]
    #[inline(always)]
    pub fn part(&self) -> PartR {
        PartR::new(self.bits)
    }
}
#[doc = "Part code\n\nYou can [`read`](crate::Reg::read) this register and get [`part::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PartSpec;
impl crate::RegisterSpec for PartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`part::R`](R) reader structure"]
impl crate::Readable for PartSpec {}
#[doc = "`reset()` method sets PART to value 0x9160"]
impl crate::Resettable for PartSpec {
    const RESET_VALUE: u32 = 0x9160;
}
