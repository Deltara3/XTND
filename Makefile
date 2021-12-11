CC = g++
CFLAGS = -Wall -g
 
main: main.o
	$(CC) $(CFLAGS) -shared -o out/main.so main.o
 
main.o: src/main.cpp
	$(CC) $(CFLAGS) -c src/main.cpp
