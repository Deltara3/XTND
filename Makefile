CC = g++
CFLAGS = -Wall -g -std=c++2a -fPIC

.PHONY: clean

XTND: main.o
	@mkdir -p out/obj
	$(CC) $(CFLAGS) -shared -o out/XTND.so out/obj/main.o

# phony targets
clean:
	@rm -rf out
	@mkdir -p out/obj

inject:
	sudo injector -n spwn out/XTND.so

# XTND targets
main.o: src/XTND/main.cpp
	$(CC) $(CFLAGS) -o out/obj/main.o -c src/XTND/main.cpp