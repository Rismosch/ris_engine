#ifndef __RIS_ENTRY_TEST_H__
#define __RIS_ENTRY_TEST_H__

#ifndef RIS_ENABLE_UNIT_TESTS
#error "this is the test entry, it may only be used when tests are enabled"
#endif /* RIS_ENABLE_UNIT_TESTS */

#include <cstddef>
#include <gccore.h>
#include <stdio.h>
#include <stdlib.h>

#include "ris_assert.hpp"

static void *xfb = NULL;
static GXRModeObj *rmode = NULL;

int entry(int, char **) {
  // setup
  VIDEO_Init();
  rmode = VIDEO_GetPreferredMode(NULL);
  xfb = MEM_K0_TO_K1(SYS_AllocateFramebuffer(rmode));
  console_init(xfb, 20, 20, rmode->fbWidth, rmode->xfbHeight,
               rmode->fbWidth * VI_DISPLAY_PIX_SZ);
  VIDEO_Configure(rmode);
  VIDEO_SetNextFramebuffer(xfb);
  VIDEO_SetBlack(false);
  VIDEO_Flush();

  VIDEO_WaitVSync();
  if (rmode->viTVMode & VI_NON_INTERLACE) {
    VIDEO_WaitVSync();
  }

  printf("\x1b[2;0H");

  // testing harness
  printf("this is the test entry :)\n");

  try {
    RIS_ASSERT(false);
  } catch (RisException exception) {
    printf("test failed: %s\n", exception.message);
  }

  // main loop
  while (SYS_MainLoop()) {
    VIDEO_WaitVSync();
  }

  return 0;
}

#endif /* __RIS_ENTRY_TEST_H__ */

/* END OF FILE */
