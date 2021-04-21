CPPC = g++
FLAGS = -g -Wall -Wextra
SRCDIR = src/
OBJDIR = build/obj/
EXECDIR = build/executable/
SRCS = $(wildcard $(SRCDIR)*.cpp) $(wildcard $(SRCDIR)*/*.cpp)
OBJS = $(patsubst $(SRCDIR)%.cpp, $(OBJDIR)%.o, $(SRCS))
NAME = weeb
BUILD = $(EXECDIR)$(NAME)

all: $(OBJS)
	$(CPPC) $(OBJS) -o $(BUILD)

$(OBJDIR)%.o: $(SRCDIR)%.cpp
	$(CPPC) $(FLAGS) -c $< -o $@
