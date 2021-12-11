CC = g++
CFLAGS = -Wall -g
 
XTND: main.o
	@mkdir -p out/obj
	$(CC) $(CFLAGS) -shared -o out/main.so out/obj/main.o

clean:
	@rm -rf out
	@mkdir -p out/obj

# file targets
main.o: src/main.cpp
	$(CC) $(CFLAGS) -o out/obj/main.o -c src/main.cpp
