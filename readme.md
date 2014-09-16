# Binary Encoder / Decoder

I needed a more efficient way to store serialized rust ofjects than just
serializing everything to JSON.

This encoder + decoder pair encodes the data that it recieves in a binary
format that is at least as space efficient as it is in memory.

This encoding strategy is not backwards compatible.  If you change the
