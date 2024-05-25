#include "esp_system.h"

void app_main();

void app_main(void)
{
    extern void rust_main();
    rust_main();
}
