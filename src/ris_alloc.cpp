#include "ris_alloc.hpp"

#include <malloc.h>
#include <new>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "ris_assert.hpp"
#include "ris_macro.hpp"

void *ris_untracked_alloc(size_t size, size_t align) {
  return memalign(align, size);
}

void ris_untracked_free(void *ptr) { free(ptr); }

void *ris_alloc(RisAllocationArgs args) {
#ifdef RIS_DEBUG
  // assert alignment is a power of 2
  RIS_ASSERT(args.align != 0);
  RIS_ASSERT((args.align & (args.align - 1)) == 0);
#endif /* RIS_DEBUG */

  // allocate memory
  void *ptr = ris_untracked_alloc(args.size, args.align);

#ifdef RIS_DEBUG

  // check if allocation was successful
  if (!ptr) {
    RIS_PANIC("bad alloc");
  }

  // track allocation
  // TODO
#endif /* RIS_DEBUG */

  return ptr;
}

void ris_free(void *ptr) {
  if (!ptr)
    return;

  // track allocation
  // TODO

  // free memory
  ris_untracked_free(ptr);
}

RisAllocationArgs __ris_alloc_args(size_t size, size_t align) {
  return RisAllocationArgs{
      .size = size,
      .align = align,
      .file = RIS_UNKNOWN_FILE,
      .line = RIS_UNKNOWN_LINE,
  };
}

// ----------------------------------------------------------------------------
// operator new
// ----------------------------------------------------------------------------
void *operator new(size_t size) {
  auto args = __ris_alloc_args(size, alignof(max_align_t));
  return ris_alloc(args);
}

void *operator new[](size_t size) {
  auto args = __ris_alloc_args(size, alignof(max_align_t));
  return ris_alloc(args);
}

void *operator new(size_t size, std::align_val_t alignment) {
  auto args = __ris_alloc_args(size, static_cast<size_t>(alignment));
  return ris_alloc(args);
}

void *operator new[](size_t size, std::align_val_t alignment) {
  auto args = __ris_alloc_args(size, static_cast<size_t>(alignment));
  return ris_alloc(args);
}

// -----------------------------------------------------------------------------
// nothrow operator new
// -----------------------------------------------------------------------------

void *operator new(size_t size, const std::nothrow_t &) noexcept {
#ifdef __cpp_exceptions
  try {
    return ::operator new(size);
  } catch (...) {
    return nullptr;
  }
#else
  return ::operator new(size);
#endif
}

void *operator new[](size_t size, const std::nothrow_t &) noexcept {
#ifdef __cpp_exceptions
  try {
    return ::operator new[](size);
  } catch (...) {
    return nullptr;
  }
#else
  return ::operator new[](size);
#endif
}

void *operator new(size_t size, std::align_val_t alignment,
                   const std::nothrow_t &) noexcept {
#ifdef __cpp_exceptions
  try {
    return ::operator new(size, alignment);
  } catch (...) {
    return nullptr;
  }
#else
  return ::operator new(size, alignment);
#endif
}

void *operator new[](std::size_t size, std::align_val_t alignment,
                     const std::nothrow_t &) noexcept {
#ifdef __cpp_exceptions
  try {
    return ::operator new[](size, alignment);
  } catch (...) {
    return nullptr;
  }
#else
  return ::operator new[](size, alignment);
#endif
}

// -----------------------------------------------------------------------------
// operator delete
// -----------------------------------------------------------------------------
void operator delete(void *ptr) noexcept { ris_free(ptr); }

void operator delete[](void *ptr) noexcept { ris_free(ptr); }

// -----------------------------------------------------------------------------
// sized delete
// -----------------------------------------------------------------------------

void operator delete(void *ptr, std::size_t /*size*/) noexcept {
  ris_free(ptr);
}

void operator delete[](void *ptr, std::size_t /*size*/) noexcept {
  ris_free(ptr);
}

// -----------------------------------------------------------------------------
// aligned delete
// -----------------------------------------------------------------------------

void operator delete(void *ptr, std::align_val_t /*alignment*/) noexcept {
  ris_free(ptr);
}

void operator delete[](void *ptr, std::align_val_t /*alignment*/) noexcept {
  ris_free(ptr);
}

// -----------------------------------------------------------------------------
// sized + aligned delete
// -----------------------------------------------------------------------------

void operator delete(void *ptr, std::size_t /*size*/,
                     std::align_val_t /*alignment*/) noexcept {
  ris_free(ptr);
}

void operator delete[](void *ptr, std::size_t /*size*/,
                       std::align_val_t /*alignment*/) noexcept {
  ris_free(ptr);
}
