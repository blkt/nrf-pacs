#[doc = "Register `TASKS_START` writer"]
pub type W = crate::W<TasksStartSpec>;
#[doc = "Field `TASKS_START` writer - "]
pub type TasksStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tasks_start(&mut self) -> TasksStartW<'_, TasksStartSpec> {
        TasksStartW::new(self, 0)
    }
}
#[doc = "Start Timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStartSpec;
impl crate::RegisterSpec for TasksStartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_start::W`](W) writer structure"]
impl crate::Writable for TasksStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_START to value 0"]
impl crate::Resettable for TasksStartSpec {}
