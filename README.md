# CRC - Cyclic Redundancy Codes

Checksum to detect and/or correct errors in communications transmissions. This
algorithm is used to calculate the checksum in Ethernet frames amongst other
things.

## Features

The algorithm is self is general so for CRC spesifications that require
reflected data or reflected remainder there are features to toggle them on/off.

```text
--features reflect_data         # reflects the data bits
--features reflect_remainder    # reflects the remainder bits
--features reflect_all          # reflects both data and remainder bits
```

## Specifications

|Features| CRC-CCITT| CRC-16 | CRC-32 |
|--------|----------|--------|--------|
|Width   |16 bits|16 bits|32 bits|
|Truncated Polynominal|0x1021|0x8005|0x04C11DB7|
|Initial Remainder|0xFFFF|0x0000|0xFFFFFFFF|
|Final XOR value|0x0000|0x0000|0xFFFFFFFF|
|Reflect data?|No|Yes|Yes|
|Reflect remainder?|No|Yes|Yes|
|Check value*|0x29B1|0xBB3D|0xCBF43926|

## Further reading

https://barrgroup.com/Embedded-Systems/How-To/CRC-Calculation-C-Code