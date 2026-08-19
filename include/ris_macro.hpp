#ifndef __RIS_MACRO_H__
#define __RIS_MACRO_H__

#define RIS_CONCAT_IMPL(a, b) a##b
#define RIS_CONCAT(a, b) RIS_CONCAT_IMPL(a, b)
#define RIS_UNIQUE_NAME() RIS_CONCAT(__temp_, __COUNTER__)

#ifdef NDEBUG
#define RIS_RELEASE
#else
#define RIS_DEBUG
#endif /* NDEBUG */

#define RIS_UNKNOWN_FILE "__unknown__"
#define RIS_UNKNOWN_LINE ~static_cast<uint32_t>(0)

#endif /* __RIS_MACRO_H__ */

/* END OF FILE */
