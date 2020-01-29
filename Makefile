PROGNAME := sandesh
INCDIR := sandesh/library/c
SRCDIR := sandesh/library
LIBDIR := c
OUTDIR := target/build
TARGET := $(OUTDIR)/$(PROGNAME)
SRCS := $(wildcard $(SRCDIR)/*.c) $(wildcard $(SRCDIR)/$(LIBDIR)/*.c)
OBJS := $(addprefix $(OUTDIR)/,$(patsubst %.c,%.o,$(SRCS)))
#$(warning $(OBJS))

CC = gcc
CFLAGS = -Wall -O2 -I $(INCDIR) -pthread -ggdb -g -O0

.PHONY: all clean
all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

$(OUTDIR)/%.o:%.c
	@if [ ! -e `dirname $@` ]; then mkdir -p `dirname $@`; fi
	$(CC) $(CFLAGS) -o $@ -c $<

clean:
	rm -rf $(OUTDIR)
