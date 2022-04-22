#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define SUCCESS (0U)

typedef uint8_t error_t;

error_t read_sensor_data(uint16_t* pResult) {
    if (pResult == NULL) {
        printf("Explode!");
    }

    *pResult = 42;
    return SUCCESS;
}

error_t process_sensor_data(void) {
    uint16_t sensor_value;
    error_t err = read_sensor_data(&sensor_value);
    if (err == SUCCESS) {
        printf("Value = %d\n", sensor_value);
        return SUCCESS;
    } else {
        return err;
    }
}

int main(void) {
    (void) process_sensor_data();
    return 0;
}
