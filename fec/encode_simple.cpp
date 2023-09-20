#include "encode_simple.h"

extern "C" {
#include <fec.h>
}
#include <cstring>
#include <sparse/sparse.h>

static void encode_rs(struct image_proc_ctx *ctx)
{
    struct image *fcx = ctx->ctx;
    int j;
    uint8_t data[fcx->rs_n];
    uint64_t i;

    for (i = ctx->start; i < ctx->end; i += fcx->rs_n) {
        for (j = 0; j < fcx->rs_n; ++j) {
            data[j] = image_get_interleaved_byte(i + j, fcx);
        }

        encode_rs_char(ctx->rs, data, &fcx->fec[ctx->fec_pos]);
        ctx->fec_pos += fcx->roots;
    }
}

void calculate_rounds(uint64_t size, image *ctx);
int process_chunk(void *priv, const void *data, size_t len);

static void file_image_partial_load(int fd, uint64_t start_off, uint64_t end_off, image *ctx)
{
    uint64_t size = 0;
    uint64_t len = 0;
    struct sparse_file *file;

    if (ctx->sparse) {
        file = sparse_file_import(fd, false, false);
    } else {
        file = sparse_file_import_auto(fd, false, ctx->verbose);
    }

    if (!file) {
        FATAL("failed to read file %s\n", ctx->fec_filename);
    }

    len = sparse_file_len(file, false, false);

    size += len;

    uint64_t real_size = end_off - start_off;
    calculate_rounds(real_size, ctx);
    ctx->inp_size = size;

    if (ctx->verbose) {
        INFO("allocating %" PRIu64 " bytes of memory\n", ctx->inp_size);
    }

    ctx->input = new uint8_t[ctx->inp_size];

    if (!ctx->input) {
        FATAL("failed to allocate memory\n");
    }

    memset(ctx->input, 0, ctx->inp_size);
    ctx->output = ctx->input;
    ctx->pos = 0;

    sparse_file_callback(file, false, false, process_chunk, ctx);
    sparse_file_destroy(file);

    // Artificially limit the input size, so that we don't process past this point
    assert(start_off == 0);
    ctx->inp_size = end_off;

    assert(ctx->pos % FEC_BLOCKSIZE == 0);
}

extern "C" image encode_simple(int input_fd, unsigned num_roots, uint64_t input_start_off, uint64_t input_end_off, int verbose) {
    // Context create
    image ctx;
    image_init(&ctx);
    ctx.roots = num_roots;
    ctx.verbose = verbose;

    // Image load
    assert(ctx.roots > 0 && ctx.roots < FEC_RSM);
    ctx.rs_n = FEC_RSM - ctx.roots;
    file_image_partial_load(input_fd, input_start_off, input_end_off, &ctx);

    // Image ECC new
    assert(ctx.rounds > 0); /* image_load should be called first */
    ctx.fec_size = ctx.rounds * ctx.roots * FEC_BLOCKSIZE;
    if (ctx.verbose)
        INFO("allocating %u bytes of memory\n", ctx.fec_size);
    ctx.fec = new uint8_t[ctx.fec_size];
    if (!ctx.fec)
        FATAL("failed to allocate %u bytes\n", ctx.fec_size);

    if (ctx.verbose) {
        INFO("\traw fec size: %u\n", ctx.fec_size);
        INFO("\tblocks: %" PRIu64 "\n", ctx.blocks);
        INFO("\trounds: %" PRIu64 "\n", ctx.rounds);
    }

    if (!image_process(encode_rs, &ctx)) {
        FATAL("failed to process input\n");
    }

    return ctx;
}

extern "C" void encode_simple_free_ctx(image ctx)
{
    image_free(&ctx);
}
