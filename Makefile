CC        = rustc

SOURCES   = node.rs gaddag.rs
OBJECTS   = $(patsubst %.rs, %.o, $(SOURCES))
TARGET    = gaddag

$(TARGET): $(OBJECTS)
	$(CC) $^ -o $@

$(OBJECTS): %.o : %.rs
	$(CC) -c $< -o $@

clean:
	rm -f $(OBJECTS) $(TARGET)
