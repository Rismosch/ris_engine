#include <gccore.h>
#include <stdio.h>
#include <stdlib.h>
// #include <wiiuse/wpad.h>

#include "ris_primitives.h"

#include "greeter.hpp"

static void *xfb = NULL;
static GXRModeObj *rmode = NULL;

int main(int, char **) {
  // Initialise the video system
  VIDEO_Init();

  // This function initialises the attached controllers
  // WPAD_Init();

  // Obtain the preferred video mode from the system
  // This will correspond to the settings in the Wii menu
  rmode = VIDEO_GetPreferredMode(NULL);

  // Allocate memory for the display in the uncached region
  xfb = MEM_K0_TO_K1(SYS_AllocateFramebuffer(rmode));

  // Initialise the console, required for printf
  console_init(xfb, 20, 20, rmode->fbWidth, rmode->xfbHeight,
               rmode->fbWidth * VI_DISPLAY_PIX_SZ);

  // Set up the video registers with the chosen mode
  VIDEO_Configure(rmode);

  // Tell the video hardware where our display memory is
  VIDEO_SetNextFramebuffer(xfb);

  // Make the display visible
  VIDEO_SetBlack(false);

  // Flush the video register changes to the hardware
  VIDEO_Flush();

  // Wait for Video setup to complete
  VIDEO_WaitVSync();
  if (rmode->viTVMode & VI_NON_INTERLACE)
    VIDEO_WaitVSync();

  // The console understands VT terminal escape codes
  // This positions the cursor on row 2, column 0
  // we can use variables for this with format codes too
  // e.g. printf ("\x1b[%d;%dH", row, column );
  printf("\x1b[2;0H");

  Greeter greeter;
  greeter.greet();
  printf("\n");

  u8 a = UINT8_MAX;
  u16 b = UINT16_MAX;
  u32 c = UINT32_MAX;
  u64 d = UINT64_MAX;
  s8 e = INT8_MIN;
  s16 f = INT16_MIN;
  s32 g = INT32_MIN;
  s64 h = INT64_MIN;

  size_t i = SIZE_MAX;
  ssize_t j = 1;

  while (true) {
    ssize_t candidate = j << 1;
    if (!candidate) {
      break;
    } else {
      j = candidate;
    }
  }

  bool k = true;

  f32 l = 12.34f;
  f64 m = -56.78;

  printf("byte order: %i\n", BYTE_ORDER);
  printf("be: %i\n", BYTE_ORDER == BIG_ENDIAN);
  printf("ll: %i\n", BYTE_ORDER == LITTLE_ENDIAN);
  printf("\n");
  printf("u8:    %u\n", a);
  printf("u16:   %u\n", b);
  printf("u32:   %u\n", c);
  printf("u64:   %llu\n", d);
  printf("i8:    %i\n", e);
  printf("i16:   %i\n", f);
  printf("i32:   %i\n", g);
  printf("i64:   %lli\n", h);
  printf("usize: %u\n", i);
  printf("isize: %i\n", j);
  printf("bool:  %i\n", k);
  printf("f32:   %f\n", static_cast<f64>(l));
  printf("f64:   %f\n", m);

  while (SYS_MainLoop()) {

    //// Call WPAD_ScanPads each loop, this reads the latest controller states
    // WPAD_ScanPads();

    //// WPAD_ButtonsDown tells us which buttons were pressed in this loop
    //// this is a "one shot" state which will not fire again until the button
    /// has / been released
    // u32 pressed = WPAD_ButtonsDown(0);

    //// We return to the launcher application via exit
    // if (pressed & WPAD_BUTTON_HOME)
    //   exit(0);

    // Wait for the next frame
    VIDEO_WaitVSync();
  }

  return 0;
}
