#ifndef YESALLOC_H
#define YESALLOC_H

#define MIN(a, b) ((a) < (b) ? (a) : (b))


#ifdef __cplusplus
#pragma once
#endif

#include <stddef.h>
#include <stdalign.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * a rust backend allocation function.
 */
void* yesmalloc(size_t size, size_t align);

/**
 * a rust backend allocation function which initialize the bytes with 0.
 */
void* yescalloc(size_t size, size_t align);

/**
 * a rust backend reallocation function.
 */
void* yesremalloc(void* ptr, size_t size, size_t align, size_t new_size);

/**
 * a rust backend reallocation function which intialize the bytes with 0.
 */
void* yesrecalloc(void* ptr, size_t size, size_t align, size_t new_size);

/**
 * a rust backend deallocation function which give back the memory to the heap.
 */
void yesfree(void* ptr, size_t size, size_t align);

#ifdef __cplusplus
}
#endif

#endif