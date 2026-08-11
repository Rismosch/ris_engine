# Clear the implicit built in rules
.SUFFIXES:

ifeq ($(strip $(DEVKITPPC)),)
$(error "Please set DEVKITPPC in your environment. export DEVKITPPC=<path to>devkitPPC")
endif

include $(DEVKITPPC)/wii_rules

# TARGET is the name of the output
# BUILD is the directory where object files & intermediate files will be placed
# SOURCES is a list of directories containing source code
# INCLUDES is a list of directories containing extra header files
TARGET   := a
BUILD    := build
SOURCES  := src
DATA     := data
INCLUDES := include

# options for code generation
CFLAGS := \
	$(MACHDEP) \
	$(INCLUDE) \
	-std=c++20 \
	-Wall -Wextra -Wpedantic \
	-Wconversion -Wsign-conversion \
	-Wshadow \
	-Wold-style-cast \
	-Wnon-virtual-dtor \
	-Woverloaded-virtual \
	-Wnull-dereference \
	-Wdouble-promotion \
	-Wformat=2 
LDFLAGS = $(MACHDEP) -Wl,-Map,$(notdir $@).map

DEBUG ?= 1
TEST  ?= 0

# is debug?
ifeq ($(DEBUG), 1)
	CFLAGS += -g3 -Og
	CFLAGS += -D_GLIBCXX_ASSERTIONS
	LDFLAGS += -g
else
	CFLAGS += -O3 -DNDEBUG -flto
	LDFLAGS += -flto
endif

# is test?
ifeq ($(TEST), 1)
	CFLAGS += -DRIS_ENABLE_UNIT_TESTS
else
	CFLAGS += -fno-exceptions
endif

CXXFLAGS = $(CFLAGS)

# any extra libraries we wish to link with the project
LIBS := -lwiiuse -lbte -logc -lm

# list of directories containing libraries, this must be the top level containing
# include and lib
LIBDIRS :=

# no real need to edit anything past this point unless you need to add additional
# rules for different file extensions

ifneq ($(BUILD),$(notdir $(CURDIR)))

export OUTPUT := $(CURDIR)/$(TARGET)

export VPATH := \
	$(foreach dir,$(SOURCES),$(CURDIR)/$(dir)) \
	$(foreach dir,$(DATA),$(CURDIR)/$(dir))

export DEPSDIR := $(CURDIR)/$(BUILD)

# automatically build a list of object files for our project
CFILES   := $(foreach dir,$(SOURCES),$(notdir $(wildcard $(dir)/*.c)))
CPPFILES := $(foreach dir,$(SOURCES),$(notdir $(wildcard $(dir)/*.cpp)))
sFILES   := $(foreach dir,$(SOURCES),$(notdir $(wildcard $(dir)/*.s)))
SFILES   := $(foreach dir,$(SOURCES),$(notdir $(wildcard $(dir)/*.S)))
BINFILES := $(foreach dir,$(DATA),$(notdir $(wildcard $(dir)/*.*)))

# use CXX for linking C++ projects, CC for standard C
ifeq ($(strip $(CPPFILES)),)
	export LD := $(CC)
else
	export LD := $(CXX)
endif

export OFILES_BIN := $(addsuffix .o,$(BINFILES))
export OFILES_SOURCES := $(CPPFILES:.cpp=.o) $(CFILES:.c=.o) $(sFILES:.s=.o) $(SFILES:.S=.o)
export OFILES := $(OFILES_BIN) $(OFILES_SOURCES)

export HFILES := $(addsuffix .h,$(subst .,_,$(BINFILES)))

# build a list of include paths
export INCLUDE := \
	$(foreach dir,$(INCLUDES), -iquote $(CURDIR)/$(dir)) \
	$(foreach dir,$(LIBDIRS),-I$(dir)/include) \
	-I$(CURDIR)/$(BUILD) \
	-isystem $(LIBOGC_INC)

# build a list of library paths
export LIBPATHS := -L$(LIBOGC_LIB) $(foreach dir,$(LIBDIRS),-L$(dir)/lib)

export OUTPUT := $(CURDIR)/$(TARGET)
.PHONY: $(BUILD) debug release test clean run

$(BUILD):
	[ -d $@ ] || mkdir -p $@
	$(MAKE) --no-print-directory -C $(BUILD) -f $(CURDIR)/Makefile

debug:
	$(MAKE) DEBUG=1 TEST=0

release:
	$(MAKE) DEBUG=0 TEST=0

test:
	$(MAKE) DEBUG=1 TEST=1

clean:
	rm -fr $(BUILD) $(OUTPUT).elf $(OUTPUT).dol

run:
#	wiiload $(TARGET).dol
	dolphin-emu-nogui $(TARGET).dol

else

DEPENDS	:=	$(OFILES:.o=.d)

# main targets
$(OUTPUT).dol: $(OUTPUT).elf
$(OUTPUT).elf: $(OFILES)

$(OFILES_SOURCES) : $(HFILES)

# This rule links in binary data with the .jpg extension
%.jpg.o	%_jpg.h : %.jpg
	@echo $(notdir $<)
	$(bin2o)

-include $(DEPENDS)

endif

