#include <stddef.h>

typedef struct BufferDimensions {
  size_t width;
  size_t height;
  size_t unpadded_bytes_per_row;
  size_t padded_bytes_per_row;
} BufferDimensions;

BufferDimensions buffer_dimensions_new(size_t width, size_t height);

void save_png(const char *path, const unsigned char *data,
              const BufferDimensions *buffer_dimensions);
