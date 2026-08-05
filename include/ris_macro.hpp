#ifndef __RIS_MACRO_H__
#define __RIS_MACRO_H__

#define RIS_CONCAT_IMPL(a, b) a##b
#define RIS_CONCAT(a, b) RIS_CONCAT_IMPL(a, b)
#define RIS_UNIQUE() RIS_CONCAT(__temp_, __COUNTER__)

#endif /* __RIS_MACRO_H__ */

/* END OF FILE */
