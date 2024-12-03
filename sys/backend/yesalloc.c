#include <stdlib.h>
#include <stddef.h>
#include <stdalign.h>
#include <stdarg.h>
#include <string.h>
#include "yesalloc.h"

/**
 * a rust backend allocation function.
 */
void* yesmalloc(size_t size, size_t align) {
    // wir wrappen jetz erst mal ´aligned_alloc´ so dass er rust angenehmer ist.
    void* ptr = aligned_alloc(align, size);
    // wir geben den pointer zurueck
    return ptr;
}

/**
 * a rust backend allocation function which initialize the bytes with 0.
 */
void* yescalloc(size_t size, size_t align) {
    // wir erstellen einen neuen pointer mit korrekter groesse und ausrichtung.
    void* ptr = yesmalloc(size, align);
    // wir pruefen auf null.
    if (ptr != NULL) {
        // da wir jetzt garantiert nicht null sind ist es vollkommen sicher den speicher auf 0 zu initialisieren.
        memset(ptr, 0x00, size);
    }
    // wir geben den pointer zurueck und im null fall free und realloc wissen mit null umzugehen.
    return ptr;
}

/**
 * a rust backend reallocation function.
 */
void* yesremalloc(void* ptr, size_t size, size_t align, size_t new_size) {
    // wir allokieren schonmal den neuen speicher.
    void* nptr = yesmalloc(new_size, align);
    // ueberpruefen `ptr` auf null
    if (ptr != NULL && nptr != NULL) {
        // da wir jetzt garantiert nicht null sind koennen wir mit `ptr` und `nptr` sicher arbeiten, dafuer kopieren wir die daten.
        memcpy(nptr, ptr, MIN(size, new_size));
        // wir koennen nun ptr freigeben, da wir die daten in nptr gespeichert haben.
        yesfree(ptr, size, align);
    };
    // und zu guter letzt geben wir den ptr zurueck. da wir alles beachtet haben und geprueft was notwendig war ist auch das sicher.#
    return nptr;
}

/**
 * a rust backend reallocation function which intialize the bytes with 0.
 */
void* yesrecalloc(void* ptr, size_t size, size_t align, size_t new_size) {
    // wir allokieren schonmal den neuen speicher.
    void* nptr = yesmalloc(new_size, align);
    // wir geben `ptr` frei und yesfree kann mit null umgehen.
    yesfree(ptr, size, align);
    // ueberpruefen `nptr` auf null
    if (nptr != NULL) {
        // da wir jetzt garantiert nicht null sind koennen wir mit `ptr` mit `nptr` sicher arbeiten und initialisieren.
        memset(nptr, 0x00, new_size);
    }
    // und zu guter letzt, den neuen pointer zuruckgeben.
    return nptr;
}

/**
 * a rust backend deallocation function which give back the memory to the heap.
 */
void yesfree(void* ptr, size_t size, size_t align) {
    // eigentlich kommt free mit aligned_allocs ptr klar, jedoch uebergibt rust einen Layout bei dealloc und dieses bacnend richtet sich daran die api auf rust vorzubereiten worauf sie noch mal besser abstrahiert wird,
    if (ptr != NULL) {
        // jetzt sind wir nicht null und koennen speicher freigeben.
        free(ptr);
    };
    return;
}