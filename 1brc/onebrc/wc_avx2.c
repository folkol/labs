// wc_avx2.c from GNU Coreutils + main method

#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include <x86intrin.h>
#include <unistd.h>
#include <errno.h>

struct wc_lines { int err; intmax_t lines; intmax_t bytes; };

/* This must be below 16 KB (16384) or else the accumulators can
   theoretically overflow, producing wrong result. This is 2*32 bytes below,
   so there is no single bytes in the optimal case. */
#define BUFSIZE (16320)

extern struct wc_lines
wc_lines_avx2 (int fd)
{
  intmax_t lines = 0;
  intmax_t bytes = 0;

  __m256i
    zeroes = _mm256_setzero_si256 (),
    endlines = _mm256_set1_epi8 ('\n');

  while (true)
    {
      __m256i
        accumulator = _mm256_setzero_si256 (),
        accumulator2 = _mm256_setzero_si256 (),
        avx_buf[BUFSIZE / sizeof (__m256i)];

      ssize_t bytes_read = read (fd, avx_buf, sizeof avx_buf);
      if (bytes_read <= 0)
        return (struct wc_lines) { bytes_read == 0 ? 0 : errno, lines, bytes };

      bytes += bytes_read;
      __m256i *datap = avx_buf;

      while (bytes_read >= 64)
        {
          __m256i
            to_match = _mm256_load_si256 (datap),
            to_match2 = _mm256_load_si256 (datap + 1),
            matches = _mm256_cmpeq_epi8 (to_match, endlines),
            matches2 = _mm256_cmpeq_epi8 (to_match2, endlines);

          /* Compare will set each 8 bit integer in the register to 0xFF
             on match.  When we subtract it the 8 bit accumulators
             will underflow, so this is equal to adding 1. */
          accumulator = _mm256_sub_epi8 (accumulator, matches);
          accumulator2 = _mm256_sub_epi8 (accumulator2, matches2);

          datap += 2;
          bytes_read -= 64;
        }

      /* Horizontally add all 8 bit integers in the register.  */
      accumulator = _mm256_sad_epu8 (accumulator, zeroes);
      lines +=   _mm256_extract_epi16 (accumulator, 0)
               + _mm256_extract_epi16 (accumulator, 4)
               + _mm256_extract_epi16 (accumulator, 8)
               + _mm256_extract_epi16 (accumulator, 12);

      accumulator2 = _mm256_sad_epu8 (accumulator2, zeroes);
      lines +=   _mm256_extract_epi16 (accumulator2, 0)
               + _mm256_extract_epi16 (accumulator2, 4)
               + _mm256_extract_epi16 (accumulator2, 8)
               + _mm256_extract_epi16 (accumulator2, 12);

      /* Finish up any left over bytes */
      char *end = (char *) datap + bytes_read;
      for (char *p = (char *) datap; p < end; p++)
        lines += *p == '\n';
    }
}

int main(void) {
  struct wc_lines res = wc_lines_avx2(0);
  printf("%jd %jd\n", res.lines, res.bytes);
}
