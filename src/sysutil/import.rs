use core::ffi::c_void;

pub type osMutexId_t = *const c_void;

pub enum osStatus_t {
    osOK = 0,
    osError = -1,
    osErrorTimeout = -2,
    osErrorResource = -3,
    osErrorParameter = -4,
    osErrorNoMemory = -5,
    osErrorISR = -6,
    osStatusReserved = 0x7FFFFFFF,
}
pub struct osMutexAttr_t {
   pub  name: *const u8,
   pub  attr_bits: u32,
   pub  cb_mem: *const c_void,
   pub  cb_size: u32,
}

extern  {

    pub fn osDelay(tick: usize);
    pub fn osMutexNew(attr: *const osMutexAttr_t) -> osMutexId_t;
    pub fn osMutexRecursive(attr: *const osMutexId_t) -> osMutexId_t;
    pub fn osMutexGetName(mutex_id: osMutexId_t) -> *const u8;
    pub fn osMutexAcquire(id: osMutexId_t, timeout: usize) -> osStatus_t;
    pub fn osMutexRelease(id: osMutexId_t) -> osStatus_t;
    // pub fn osMutexGetOwner(id: osMutexId_t)->osThreadId_t;
    pub fn osMutexDelete(id: osMutexId_t) -> osStatus_t;
}
