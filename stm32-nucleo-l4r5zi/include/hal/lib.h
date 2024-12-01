#ifndef HAL_H
#define HAL_H

#include "stdint.h"
#include "stdbool.h"
#include "stdarg.h"

void default_hndlr(void);

void hal_hw_init(void);

uint32_t hal_semih_write(const int8_t *msg);

#endif /* HAL_H */
