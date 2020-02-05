#include "sys_dummy.h"

extern void osDelay(uintptr_t tick);

osStatus_t osMessageQueueGet(osMessageQueueId_t id,
                             void *msg_ptr,
                             uint8_t msg_prio,
                             uint32_t timeout)
{
    return dummyOk;
}

osMessageQueueId_t osMessageQueueNew(uint32_t msg_count,
                                     uint32_t msg_size,
                                     const osMessageQueueAttr_t *attr_t)
{
    return (osMessageQueueId_t)(0);
}

osStatus_t osMessageQueuePut(osMessageQueueId_t id,
                             const void *msg_ptr,
                             uint8_t msg_prio,
                             uint32_t timeout)
{
    return dummyOk;
}

osStatus_t osMutexAcquire(osMutexId_t id, uintptr_t timeout)
{
    return dummyOk;
}

osStatus_t osMutexDelete(osMutexId_t id)
{
    return dummyOk;
}

const uint8_t *osMutexGetName(osMutexId_t mutex_id)
{
    return NULL;
}

osMutexId_t osMutexNew(const osMutexAttr_t *attr)
{
    return (osMutexId_t)0;
}

osMutexId_t osMutexRecursive(const osMutexId_t *attr)
{
    return (osMutexId_t)(0);
}

osStatus_t osMutexRelease(osMutexId_t id)
{
    return dummyOk;
}
