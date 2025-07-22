#include "flashlight.h"
#include <assert.h>
#include <stdio.h>

void test_create_varint() {
    struct FlashlightVarint varint;

    varint = flashlight_create_varint(0);
    assert(varint.val != NULL && varint.len == 1);
    assert(varint.val[0] == 0);
    flashlight_free_varint_object(varint);

    varint = flashlight_create_varint(1);
    assert(varint.val != NULL && varint.len == 1);
    assert(varint.val[0] == 1);
    flashlight_free_varint_object(varint);

    varint = flashlight_create_varint(2);
    assert(varint.val != NULL && varint.len == 1);
    assert(varint.val[0] == 2);
    flashlight_free_varint_object(varint);

    varint = flashlight_create_varint(127);
    assert(varint.val != NULL && varint.len == 1);
    assert(varint.val[0] == 127);
    flashlight_free_varint_object(varint);

    varint = flashlight_create_varint(128);
    assert(varint.val != NULL && varint.len == 2);
    assert(varint.val[0] == 128);
    assert(varint.val[1] == 1);
    flashlight_free_varint_object(varint);

    varint = flashlight_create_varint(255);
    assert(varint.val != NULL && varint.len == 2);
    assert(varint.val[0] == 255);
    assert(varint.val[1] == 1);
    flashlight_free_varint_object(varint);

    varint = flashlight_create_varint(25565);
    assert(varint.val != NULL && varint.len == 3);
    assert(varint.val[0] == 221);
    assert(varint.val[1] == 199);
    assert(varint.val[2] == 1);
    flashlight_free_varint_object(varint);

    varint = flashlight_create_varint(2097151);
    assert(varint.val != NULL && varint.len == 3);
    assert(varint.val[0] == 255);
    assert(varint.val[1] == 255);
    assert(varint.val[2] == 127);
    flashlight_free_varint_object(varint);

    varint = flashlight_create_varint(2147483647);
    assert(varint.val != NULL && varint.len == 5);
    assert(varint.val[0] == 255);
    assert(varint.val[1] == 255);
    assert(varint.val[2] == 255);
    assert(varint.val[3] == 255);
    assert(varint.val[4] == 7);
    flashlight_free_varint_object(varint);

    varint = flashlight_create_varint(-1);
    assert(varint.val != NULL && varint.len == 5);
    assert(varint.val[0] == 255);
    assert(varint.val[1] == 255);
    assert(varint.val[2] == 255);
    assert(varint.val[3] == 255);
    assert(varint.val[4] == 15);
    flashlight_free_varint_object(varint);

    varint = flashlight_create_varint(-2147483648);
    assert(varint.val != NULL && varint.len == 5);
    assert(varint.val[0] == 128);
    assert(varint.val[1] == 128);
    assert(varint.val[2] == 128);
    assert(varint.val[3] == 128);
    assert(varint.val[4] == 8);
    flashlight_free_varint_object(varint);
}

void test_decode_varint0(struct FlashlightVarint varint, int expected) {
    struct FlashlightVarintDecodeResult res;
    res = flashlight_decode_varint(varint);
    assert(res.res);
    assert(res.val == expected);
}

void test_decode_varint() {
    struct FlashlightVarint varint;
    struct FlashlightVarintDecodeResult res;

    unsigned char v[] = {0, 0, 0, 0, 0, 0};
    varint.val = v;
    varint.len = 6;
    res = flashlight_decode_varint(varint);
    assert(!res.res);
    assert(res.val == 0);

    unsigned char v1[] = {0};
    varint.val = v1;
    varint.len = 1;
    test_decode_varint0(varint, 0);

    unsigned char v2[] = {1};
    varint.val = v2;
    varint.len = 1;
    test_decode_varint0(varint, 1);

    unsigned char v3[] = {2};
    varint.val = v3;
    varint.len = 1;
    test_decode_varint0(varint, 2);

    unsigned char v4[] = {127};
    varint.val = v4;
    varint.len = 1;
    test_decode_varint0(varint, 127);

    unsigned char v5[] = {128, 1};
    varint.val = v5;
    varint.len = 2;
    test_decode_varint0(varint, 128);

    unsigned char v6[] = {255, 1};
    varint.val = v6;
    varint.len = 2;
    test_decode_varint0(varint, 255);

    unsigned char v7[] = {221, 199, 1};
    varint.val = v7;
    varint.len = 3;
    test_decode_varint0(varint, 25565);

    unsigned char v8[] = {255, 255, 127};
    varint.val = v8;
    varint.len = 3;
    test_decode_varint0(varint, 2097151);

    unsigned char v9[] = {255, 255, 255, 255, 7};
    varint.val = v9;
    varint.len = 5;
    test_decode_varint0(varint, 2147483647);

    unsigned char v10[] = {255, 255, 255, 255, 15};
    varint.val = v10;
    varint.len = 5;
    test_decode_varint0(varint, -1);

    unsigned char v11[] = {128, 128, 128, 128, 8};
    varint.val = v11;
    varint.len = 5;
    test_decode_varint0(varint, -2147483648);
}

int main() {
    test_create_varint();
    printf("%s: All assertions passed.\n", __FILE__);
    return 0;
}
