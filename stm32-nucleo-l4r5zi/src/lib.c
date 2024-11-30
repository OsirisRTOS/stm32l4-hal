#include <hal/lib.h>

void this_should_be_included(void)
{
    while (1)
    {
        __asm__("wfi");
    }
}