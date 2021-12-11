CC = g++
CFLAGS = -Wall -g -std=c++2a -fPIC

FILEEXT = so

.PHONY: clean

XTND: main.o
	@mkdir -p out/obj
	$(CC) $(CFLAGS) -shared -o out/XTND.$(FILEEXT) out/obj/main.o

# phony targets
clean:
	@rm -rf out
	@mkdir -p out/obj

inject:
	sudo injector -n spwn out/XTND.so

# XTND targets
main.o: src/main.cpp
	$(CC) $(CFLAGS) -o out/obj/main.o -c src/main.cpp