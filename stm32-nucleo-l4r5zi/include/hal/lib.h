#ifndef HAL_H
#define HAL_H

#include "stdint.h"
#include "stdbool.h"
#include "stdarg.h"

void default_hndlr(void);

void hal_hw_init(void);

void hal_semih_write_debug(const char *msg);

#endif /* HAL_H */
