# Triplet Categories
## 000 - Number
Numeric literals are declared by using 000 followed 
by 8 bits, representing the number in big-endian. Numbers
are unsigned; negative numbers wrap.
### Examples:
 - `00000000001`\
   The first `000` represents the category;
   The following `00000001` is the number being initialized,
   which is `1`.
 - `00011111111`\
   This represents the maximum unsigned 8-bit number, 
   which is `255`
## 001 - Array
Arrays are a sequence of numbers, and they are declared by using
001 followed by specifying the initialization mode followed by
the arguments of the initialization mode: 
### Array Initialization Modes - 1 bit
#### 000 - Initializing mapping function
0 - {array length: number}
1 - {mapping function: function}{array length: number}
### Examples
 - `001000`
## 010 - Operations
 - 
## 011 - Stack Procedures

## 100 - Function Marker
A function marker either starts the initialization of a function or ends one

## 101 - 

## 110 - Special Functions

## 111 - Comment