#ifdef RIS_ENABLE_UNIT_TESTS
#include "ris_entry_test.hpp"
#else
#include "ris_entry_engine.hpp"
#endif /* RIS_ENABLE_UNIT_TESTS */

int main(int argc, char **argv) { return entry(argc, argv); }
