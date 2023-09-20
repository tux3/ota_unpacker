#ifndef OTA_UNPACKER_FEC_ENCODE_SIMPLE_H
#define OTA_UNPACKER_FEC_ENCODE_SIMPLE_H

#include <cassert>
#include <cstdint>
#include "image.h"

extern "C" {

// Call encode_simple_free_ctx after done with the result
image encode_simple(int input_fd, unsigned num_roots, uint64_t input_start_off, uint64_t input_end_off, int verbose);
void encode_simple_free_ctx(image ctx);

}

#endif //OTA_UNPACKER_FEC_ENCODE_SIMPLE_H
