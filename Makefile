CXX = g++
FLAGS = -g -Wall -Wextra
SRCDIR = src/lexer/
OBJDIR = $(SRCDIR)lex-build/obj/
EXECDIR = $(SRCDIR)lex-build/exec/
SRCS = $(wildcard $(SRCDIR)*.cpp) $(wildcard $(SRCDIR)*/*.cpp)
OBJS = $(patsubst $(SRCDIR)%.cpp, $(OBJDIR)%.o, $(SRCS))
NAME = lexer.out
BUILD = $(EXECDIR)$(NAME)

all: $(OBJS)
	$(CXX) $(OBJS) -o $(BUILD)

$(OBJDIR)%.o: $(SRCDIR)%.cpp
	$(CXX) $(FLAGS) -c $< -o $@