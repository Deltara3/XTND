CC = g++
CFLAGS = -Wall -g -std=c++2a

.PHONY: clean inject

XTND: main.o
	@mkdir -p out/obj
	$(CC) $(CFLAGS) -shared -o out/XTND.so out/obj/main.o

launcher: main_launcher.o
	$(CC) $(CFLAGS) -o out/XTND out/obj/main_launcher.o

# phony targets
clean:
	@rm -rf out
	@mkdir -p out/obj

inject:
	sudo injector -n spwn out/XTND.so

# XTND targets
main.o: src/XTND/main.cpp
	$(CC) $(CFLAGS) -o out/obj/main.o -c src/main.cpp

# launcher targets
main_launcher.o: src/launcher/main.cpp