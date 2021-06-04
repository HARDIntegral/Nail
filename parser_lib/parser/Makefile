CC = clang
CFLAGS = -g -Wall -Wextra
SRCDIR = src/
OBJDIR = obj/
SRCS = $(wildcard $(SRCDIR)*.c) $(wildcard $(SRCDIR)*/*.c)
OBJS = $(patsubst $(SRCDIR)%.c, $(OBJDIR)%.o, $(SRCS))
NAME = ../parse_lib.a

all: $(OBJS)
	ar rvs $(NAME) $(OBJS)

$(OBJDIR)%.o: $(SRCDIR)%.c
	$(CC) $(CFLAGS) -c $< -o $@